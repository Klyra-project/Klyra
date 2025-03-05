use std::fmt::Formatter;
use std::net::{IpAddr, Ipv4Addr};

use bollard::container::{
    Config,
    CreateContainerOptions
};
use bollard::errors::Error as DockerError;
use bollard::models::{
    ContainerInspectResponse,
    ContainerStateStatusEnum,
    HostConfig,
    Mount,
    MountTypeEnum
};
use futures::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use serde::{
    Deserialize,
    Serialize
};

use super::{
    Context,
    Error,
    ProjectName,
    Refresh,
    State
};

const DEFAULT_IMAGE: &'static str = "public.ecr.aws/d7w6e9t1/backend:latest";

#[async_trait]
impl Refresh for ContainerInspectResponse {
    type Error = DockerError;
    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Self::Error> {
        ctx.docker()
            .inspect_container(self.id.as_ref().unwrap(), None)
            .await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Project {
    Creating(ProjectCreating),
    Ready(ProjectReady),
    Started(ProjectStarted),
    Stopped(ProjectStopped)
}

#[derive(Debug)]
pub struct DestroyError {
    error: Error
}

impl std::fmt::Display for DestroyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for DestroyError {}

impl Project {
    pub async fn stop<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Error> {
        match self {
            Self::Started(ProjectStarted { container, .. }) => {
                ctx.docker()
                    .stop_container(container.id.as_ref().unwrap(), None)
                    .await
                    .unwrap();
                Ok(Self::Stopped(ProjectStopped {
                    container: container.refresh(ctx).await.unwrap()
                }))
            }
            _otherwise => todo!()
        }
    }

    pub fn target_ip(&self) -> Result<Option<String>, Error> {
        match self.clone() {
            Self::Started(project_started) => {
                Ok(Some(project_started.target_ip().to_string()))
            }
            _ => Ok(None) // not ready
        }
    }

    pub fn state(&self) -> &'static str {
        match self {
            Self::Started(_) => "started",
            Self::Stopped(_) => "stopped",
            Self::Ready(_) => "ready",
            Self::Creating(_) => "creating"
        }
    }

    pub async fn destroy<'c, C: Context<'c>>(self, ctx: &C) -> Result<(), DestroyError> {
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
                container: ContainerInspectResponse { id, .. }
            }) => {
                ctx.docker()
                    .remove_container(id.as_ref().unwrap(), None)
                    .await
                    .unwrap();
                Ok(())
            }
            _ => todo!()
        }
    }
}

#[async_trait]
impl<'c> State<'c> for Project {
    type Next = Self;
    type Error = Error;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        let next = match self {
            Self::Creating(creating) => Self::Ready(creating.next(ctx).await?),
            Self::Ready(ready) => Self::Started(ready.next(ctx).await?),
            Self::Started(started) => Self::Started(started.next(ctx).await?),
            Self::Stopped(stopped) => Self::Ready(stopped.next(ctx).await?)
        };
        Ok(next)
    }
}

#[async_trait]
impl Refresh for Project {
    type Error = Error;

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
                                let service = Service::from_container(container.clone(), ctx).await?;
                                Self::Started(ProjectStarted { container, service })
                            }
                            ContainerStateStatusEnum::CREATED => {
                                Self::Ready(ProjectReady { container })
                            }
                            ContainerStateStatusEnum::EXITED => {
                                Self::Stopped(ProjectStopped { container })
                            }
                            _ => todo!()
                        }
                    }
                    Err(_err) => todo!()
                }
            }
        };
        Ok(next)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectCreating {
    project_name: ProjectName,
    prefix: String,
    initial_key: String,
}

impl ProjectCreating {
    pub fn new(project_name: ProjectName, prefix: String, initial_key: String) -> Self {
        Self {
            project_name,
            prefix,
            initial_key
        }
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectCreating {
    type Next = ProjectReady;
    type Error = Error;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        let pg_password = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
        let pg_password_env = format!("PG_PASSWORD={}", pg_password);

        let initial_key_env = format!("klyra_INITIAL_KEY={}", self.initial_key);

        let volume_name = format!("{}{}_vol", self.prefix, self.project_name);
        let container_name = format!("{}{}_run", self.prefix, self.project_name);
        let container = ctx
            .docker()
            .inspect_container(&container_name.clone(), None)
            .or_else(|err| async move {
                if matches!(err, DockerError::DockerResponseNotFoundError { .. }) {
                    let opts = CreateContainerOptions {
                        name: container_name.clone()
                    };
                    let config = Config {
                        image: Some(DEFAULT_IMAGE),
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
                            ("klyra_prefix", self.prefix.as_str())
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
            .await
            .unwrap();
        Ok(ProjectReady { container })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectReady {
    container: ContainerInspectResponse
}

#[async_trait]
impl<'c> State<'c> for ProjectReady {
    type Next = ProjectStarted;
    type Error = Error;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        let container_id = self.container.id.as_ref().unwrap();
        ctx.docker()
            .start_container::<String>(container_id, None)
            .await
            .or_else(|err| {
                if matches!(err, DockerError::DockerResponseNotModifiedError { .. }) {
                    // Already started
                    Ok(())
                } else {
                    Err(err)
                }
            })
            .unwrap();
        let container = self.container.refresh(ctx).await.unwrap();
        let service = Service::from_container(container.clone(), ctx).await.unwrap();
        Ok(Self::Next {
            container,
            service
        })
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
    pub async fn from_container<'c, C: Context<'c>>(container: ContainerInspectResponse, ctx: &C) -> Result<Self, Error> {
        let name = container.name
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

        Ok(Self {
            name,
            target,
        })
    }
}

#[async_trait]
impl Refresh for ProjectStarted {
    type Error = Error;
    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Error> {
        let container = self.container.refresh(ctx).await.unwrap();
        let service = Service::from_container(container.clone(), ctx).await?;
        Ok(Self {
            container,
            service
        })
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectStarted {
    type Next = Self;
    type Error = Error;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        self.refresh(ctx).await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectStopped {
    container: ContainerInspectResponse
}

#[async_trait]
impl Refresh for ProjectStopped {
    type Error = Error;

    async fn refresh<'c, C: Context<'c>>(self, ctx: &C) -> Result<Self, Self::Error> {
        Ok(Self {
            container: self.container.refresh(ctx).await.unwrap()
        })
    }
}

#[async_trait]
impl<'c> State<'c> for ProjectStopped {
    type Next = ProjectReady;
    type Error = Error;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        let ProjectStopped { container } = self.refresh(ctx).await?;
        Ok(ProjectReady { container })
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
            "test_project_do_not_upvote".parse().unwrap()
        ));
        while !matches!(&project, Project::Started(..)) {
            project = project.next(&ctx).await.unwrap();
        }
        project = project.stop(&ctx).await.unwrap();
        assert!(matches!(project, Project::Stopped(_)));
        project.destroy(&ctx).await.unwrap();
    }
}
