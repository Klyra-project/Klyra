use std::convert::Infallible;
use std::fmt::Formatter;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use bollard::container::{Config, CreateContainerOptions};
use bollard::errors::Error as DockerError;
use bollard::models::{
    ContainerInspectResponse, ContainerState, ContainerStateStatusEnum, HealthStatusEnum,
    HostConfig, Mount, MountTypeEnum,
};
use futures::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use tokio::time;

use super::{Context, EndState, Error, ErrorKind, IntoEndState, ProjectName, Refresh, State};

macro_rules! impl_from_variant {
    {$e:ty: $($s:ty => $v:ident $(,)?)+} => {
        $(
            impl From<$s> for $e {
                fn from(s: $s) -> $e {
                    <$e>::$v(s)
                }
            }
        )+
    };
}

#[async_trait]
impl Refresh for ContainerInspectResponse {
    type Error = DockerError;
    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Self::Error> {
        ctx.docker()
            .inspect_container(self.id.as_ref().unwrap(), None)
            .await
    }
}

impl From<DockerError> for Error {
    fn from(err: DockerError) -> Self {
        debug!("internal Docker error: {err}");
        Self::source(ErrorKind::Internal, err)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Project {
    Creating(ProjectCreating),
    Ready(ProjectReady),
    Started(ProjectStarted),
    Stopped(ProjectStopped),
    Errored(ProjectError),
}

impl_from_variant!(Project:
                   ProjectCreating => Creating,
                   ProjectReady => Ready,
                   ProjectStarted => Started,
                   ProjectStopped => Stopped,
                   ProjectError => Errored);

impl Project {
    pub async fn stop<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Error> {
        match self {
            Self::Creating(_) => Err(Error::custom(
                ErrorKind::InvalidOperation,
                "tried to stop a project that was not ready",
            )),
            Self::Ready(ProjectReady { container, .. }) => {
                Ok(Self::Stopped(ProjectStopped { container }))
            }
            Self::Started(ProjectStarted { container, .. }) => {
                ctx.docker()
                    .stop_container(container.id.as_ref().unwrap(), None)
                    .await?;
                Ok(Self::Stopped(ProjectStopped {
                    container: container.refresh(ctx).await?,
                }))
            }
            Self::Stopped(stopped) => Ok(Self::Stopped(stopped)),
            Self::Errored(err) => Ok(Self::Errored(err)),
        }
    }

    pub fn target_ip(&self) -> Result<Option<String>, Error> {
        match self.clone() {
            Self::Started(project_started) => Ok(Some(project_started.target_ip().to_string())),
            _ => Ok(None), // not ready
        }
    }

    pub fn state(&self) -> &'static str {
        match self {
            Self::Started(_) => "started",
            Self::Stopped(_) => "stopped",
            Self::Ready(_) => "ready",
            Self::Creating(_) => "creating",
            Self::Errored(_) => "error",
        }
    }

    pub async fn destroy<'c, C: Context<'c>>(self, ctx: &C) -> Result<(), Error> {
        match self {
            Self::Ready(ProjectReady {
                container: ContainerInspectResponse { id, .. },
                ..
            })
            | Self::Started(ProjectStarted {
                container: ContainerInspectResponse { id, .. },
                ..
            })
            | Self::Stopped(ProjectStopped {
                container: ContainerInspectResponse { id, .. },
            }) => {
                ctx.docker()
                    .remove_container(id.as_ref().unwrap(), None)
                    .await?;
                Ok(())
            }
            Self::Creating(_) | Self::Errored(_) => Ok(()),
        }
    }
}

#[async_trait]
impl<'c> State<'c> for Project {
    type Next = Self;
    type Error = Infallible;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        match self {
            Self::Creating(creating) => creating.next(ctx).await.into_end_state(),
            Self::Ready(ready) => ready.next(ctx).await.into_end_state(),
            Self::Started(started) => started.next(ctx).await.into_end_state(),
            Self::Stopped(stopped) => stopped.next(ctx).await.into_end_state(),
            Self::Errored(errored) => Ok(Self::Errored(errored)),
        }
    }
}

impl<'c> EndState<'c> for Project {
    fn is_done(&self) -> bool {
        matches!(self, Self::Errored(_) | Self::Started(_))
    }
}

#[async_trait]
impl Refresh for Project {
    type Error = Error;

    /// TODO: we could be a bit more clever than this by using the
    /// health checks instead of matching against the raw container
    /// state which is probably prone to erroneously setting the
    /// project into the wrong state if the docker is transitioning
    /// the state of its resources under us
    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Self::Error> {
        let next = match self {
            Self::Creating(creating) => Self::Creating(creating),
            Self::Ready(ProjectReady { container })
            | Self::Started(ProjectStarted { container, .. })
            | Self::Stopped(ProjectStopped { container }) => {
                let container_name = container.name.as_ref().unwrap().to_owned();
                match container.refresh(ctx).await {
                    Ok(container) => {
                        match container.state.as_ref().unwrap().status.as_ref().unwrap() {
                            ContainerStateStatusEnum::RUNNING => {
                                let service = Service::from_container(container.clone(), ctx);
                                Self::Started(ProjectStarted { container, service })
                            }
                            ContainerStateStatusEnum::CREATED => {
                                Self::Ready(ProjectReady { container })
                            }
                            ContainerStateStatusEnum::EXITED => {
                                Self::Stopped(ProjectStopped { container })
                            }
                            _ => todo!(),
                        }
                    }
                    Err(_err) => todo!(),
                }
            }
            Self::Errored(err) => Self::Errored(err),
        };
        Ok(next)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectCreating {
    project_name: ProjectName,
    initial_key: String,
}

impl ProjectCreating {
    pub fn new(project_name: ProjectName, prefix: String, initial_key: String) -> Self {
        Self {
            project_name,
            initial_key,
        }
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectCreating {
    type Next = ProjectReady;
    type Error = ProjectError;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        let pg_password = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
        let pg_password_env = format!("PG_PASSWORD={}", pg_password);

        let initial_key_env = format!("klyra_INITIAL_KEY={}", self.initial_key);

        let prefix = ctx.args().prefix.as_str();

        let volume_name = format!("{}{}_vol", prefix, self.project_name);
        let container_name = format!("{}{}_run", prefix, self.project_name);
        let container = ctx
            .docker()
            .inspect_container(&container_name.clone(), None)
            .or_else(|err| async move {
                if matches!(err, DockerError::DockerResponseServerError { status_code, .. } if status_code == 404) {
                    let opts = CreateContainerOptions {
                        name: container_name.clone()
                    };
                    let config = Config {
                        image: Some(ctx.args().image.as_str()),
                        env: Some(vec![
                            "PROXY_PORT=8000",
                            "API_PORT=8001",
                            "PG_PORT=5432",
                            "PG_DATA=/opt/klyra/postgres",
                            &pg_password_env,
                            &initial_key_env,
                            "COPY_PG_CONF=/opt/klyra/conf/postgres",
                            "PROXY_FQDN=klyraapp.rs"
                        ]),
                        labels: Some(vec![
                            ("klyra_prefix", prefix)
                        ].into_iter().collect()),
                        host_config: Some(HostConfig {
                            mounts: Some(vec![Mount {
                                target: Some("/opt/klyra".to_string()),
                                source: Some(volume_name),
                                typ: Some(MountTypeEnum::VOLUME),
                                ..Default::default()
                            }]),
                            ..Default::default()
                        }),
                        ..Default::default()
                    };
                    ctx.docker()
                        .create_container(Some(opts), config)
                        .and_then(|_| ctx.docker().inspect_container(&container_name, None))
                        .await
                } else {
                    Err(err)
                }
            })
            .await?;
        Ok(ProjectReady { container })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectReady {
    container: ContainerInspectResponse,
}

#[async_trait]
impl<'c> State<'c> for ProjectReady {
    type Next = ProjectStarted;
    type Error = ProjectError;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        let container_id = self.container.id.as_ref().unwrap();
        ctx.docker()
            .start_container::<String>(container_id, None)
            .await
            .or_else(|err| {
                if matches!(err, DockerError::DockerResponseServerError { status_code, .. } if status_code == 304) {
                    // Already started
                    Ok(())
                } else {
                    Err(err)
                }
            })?;

        let mut container = None;
        for _ in 0..9 {
            let latest = self.container.clone().refresh(ctx).await?;
            if matches!(
                latest
                    .clone()
                    .state
                    .unwrap()
                    .health
                    .unwrap()
                    .status
                    .unwrap(),
                HealthStatusEnum::HEALTHY
            ) {
                container = Some(latest);
                break;
            } else {
                time::sleep(Duration::from_secs(1)).await;
            }
        }

        if let Some(container) = container {
            let service = Service::from_container(container.clone(), ctx);
            Ok(Self::Next { container, service })
        } else {
            Err(ProjectError::custom(
                "timed out waiting for runtime to become healthy",
            ))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectStarted {
    container: ContainerInspectResponse,
    service: Service,
}

impl ProjectStarted {
    pub fn name(&self) -> &str {
        &self.service.name
    }

    pub fn target_ip(&self) -> &IpAddr {
        &self.service.target
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    name: String,
    target: IpAddr,
}

impl Service {
    pub fn from_container<'c, C: Context<'c>>(
        container: ContainerInspectResponse,
        ctx: &C,
    ) -> Self {
        let name = container
            .name
            .as_ref()
            .unwrap()
            .strip_suffix("_run")
            .unwrap()
            .strip_prefix("/")
            .unwrap()
            .to_string();

        // assumes the container is reachable on a "docker subnet" ip known as "bridge" to docker
        let target = container
            .clone()
            .network_settings
            .unwrap()
            .networks
            .unwrap()
            .remove("bridge")
            .unwrap()
            .ip_address
            .unwrap()
            .parse()
            .unwrap();

        Self { name, target }
    }
}

#[async_trait]
impl Refresh for ProjectStarted {
    type Error = Error;

    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Self::Error> {
        let container = self.container.refresh(ctx).await.unwrap();
        let service = Service::from_container(container.clone(), ctx);
        Ok(Self { container, service })
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectStarted {
    type Next = Self;
    type Error = ProjectError;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        Ok(self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectStopped {
    container: ContainerInspectResponse,
}

#[async_trait]
impl Refresh for ProjectStopped {
    type Error = Error;

    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Self::Error> {
        Ok(Self {
            container: self.container.refresh(ctx).await?,
        })
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectStopped {
    type Next = ProjectReady;
    type Error = ProjectError;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        // If stopped, try to restart
        Ok(ProjectReady {
            container: self.container,
        })
    }
}

/// A runtime error coming from inside a project
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectError {
    message: String,
}

impl ProjectError {
    pub fn custom<S: AsRef<str>>(message: S) -> Self {
        Self {
            message: message.as_ref().to_string(),
        }
    }
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProjectError {}

impl From<DockerError> for ProjectError {
    fn from(err: DockerError) -> Self {
        Self {
            message: format!("{:?}", err),
        }
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectError {
    type Next = Self;
    type Error = Infallible;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        Ok(self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::World;

    #[tokio::test]
    async fn create_project() {
        let world = World::new();
        let ctx = world.context();
        let mut project = Project::Creating(ProjectCreating::new(
            "test_project_do_not_upvote".parse().unwrap(),
        ));
        while !matches!(&project, Project::Started(..)) {
            project = project.next(&ctx).await.unwrap();
        }
        project = project.stop(&ctx).await.unwrap();
        assert!(matches!(project, Project::Stopped(_)));
        project.destroy(&ctx).await.unwrap();
    }
}
