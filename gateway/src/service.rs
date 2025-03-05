use std::fmt::Debug;
use std::net::IpAddr;
use std::path::{Path as StdPath, PathBuf};
use std::sync::Arc;

use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use bollard::Docker;
use hyper::client::HttpConnector;
use hyper::Client as HyperClient;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{Sqlite, SqlitePool};
use sqlx::types::Json as SqlxJson;
use sqlx::{query, Row, Error as SqlxError};
use tokio::sync::{Mutex, mpsc::{channel, Receiver, Sender}};

use super::{Context, Error, ProjectName};
use crate::project::{self, Project};
use crate::{ErrorKind, Refresh, State, EndState, Service};
use crate::args::Args;

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        debug!("internal SQLx error: {err}");
        Self::source(ErrorKind::Internal, err)
    }
}

#[derive(Debug)]
pub struct Work<W = Project> {
    project_name: ProjectName,
    account_name: AccountName,
    work: W,
}

#[async_trait]
impl<'c, W> State<'c> for Work<W>
where
    W: State<'c>
{
    type Next = Work<W::Next>;

    type Error = W::Error;

    async fn next<C: Context<'c>>(self, ctx: &C) -> Result<Self::Next, Self::Error> {
        Ok(Work::<W::Next> {
            project_name: self.project_name,
            account_name: self.account_name,
            work: self.work.next(ctx).await?
        })
    }
}

impl<'c, W> EndState<'c> for Work<W>
where
    W: EndState<'c>
{
    fn is_done(&self) -> bool {
        self.work.is_done()
    }
}

pub struct Worker<Svc, W> {
    service: Svc,
    send: Sender<W>,
    recv: Receiver<W>,
}

impl<Svc, W> Worker<Svc, W>
where
    W: Send
{
    pub fn new(service: Svc) -> Self {
        let (send, recv) = channel(256);
        Self { service, send, recv }
    }
}

impl<Svc, W> Worker<Svc, W> {
    pub fn sender(&self) -> Sender<W> {
        self.send.clone()
    }
}

impl<Svc, W> Worker<Svc, W>
where
    Svc: for<'c> Service<'c, State = W, Error = Error>,
    W: Debug + Send + for<'c> EndState<'c>
{
    pub async fn start(mut self) -> Result<(), Error> {
        while let Some(work) = self.recv.recv().await {
            let context = self.service.context();

            // Safety: EndState's transitions are Infallible
            let next = work.next(&context).await.unwrap();

            match self.service.update(&next).await {
                Ok(_) => {},
                Err(err) => info!("failed to update a state: {}\nstate: {:?}", err, next),
            };

            if ! next.is_done() {
                // Safety: Should only panic if `self.recv` has
                // hanged, which at this point can't happen
                self.send.send(next).await.unwrap();
            }
        }
        Ok(())
    }
}

pub struct GatewayService {
    docker: Docker,
    hyper: HyperClient<HttpConnector, Body>,
    db: SqlitePool,
    sender: Mutex<Option<Sender<Work>>>,
    args: Args
}

const DB_PATH: &'static str = "gateway.sqlite";

use crate::auth::User;
use crate::{auth::Key, AccountName};

impl GatewayService {
    /// Initialize `GatewayService` and its required dependencies.
    ///
    /// * `args` - The [`Args`] with which the service was
    /// started. Will be passed as [`Context`] to workers and state.
    pub async fn init(args: Args) -> Arc<Self> {
        info!("docker host: tcp://127.0.0.1:2735");
        let docker = Docker::connect_with_http_defaults().unwrap();

        let hyper = HyperClient::new();
        if !StdPath::new(DB_PATH).exists() {
            Sqlite::create_database(DB_PATH).await.unwrap();
        }

        info!(
            "state db: {}",
            std::fs::canonicalize(DB_PATH).unwrap().to_string_lossy()
        );
        let db = SqlitePool::connect(DB_PATH).await.unwrap();

        query("CREATE TABLE IF NOT EXISTS projects (project_name TEXT PRIMARY KEY, account_name TEXT NOT NULL, initial_key TEXT NOT NULL, project_state JSON NOT NULL)")
            .execute(&db)
            .await
            .unwrap();

        query("CREATE TABLE IF NOT EXISTS accounts (account_name TEXT PRIMARY KEY, key TEXT UNIQUE, super_user BOOLEAN DEFAULT FALSE)")
            .execute(&db)
            .await
            .unwrap();

        let service = Arc::new(Self {
            docker,
            hyper,
            db,
            sender: Mutex::new(None),
            args
        });

        let worker = Worker::new(Arc::clone(&service));
        let sender = worker.sender();
        service.set_sender(Some(sender)).await;
        tokio::spawn({
            let service = Arc::clone(&service);
            async move {
                match worker.start().await {
                    Ok(_) => info!("worker terminated successfully"),
                    Err(err) => error!("worker error: {}", err),
                };
                service.set_sender(None).await;
            }
        });

        // Queue up all the projects for reconciliation
        for Work {
            project_name,
            account_name,
            work,
        } in service.iter_projects().await
        {
            let work = work.refresh(&service.context()).await.unwrap();
            service
                .send(Work {
                    project_name,
                    work,
                    account_name,
                })
                .await
                .ok()
                .expect("failed to queue work");
        }

        service
    }

    pub async fn set_sender(&self, sender: Option<Sender<Work>>) -> Result<(), Error> {
        *self.sender.lock().await = sender;
        Ok(())
    }

    pub async fn send(&self, work: Work) -> Result<(), Error> {
        if let Some(sender) = self.sender.lock().await.as_ref() {
            Ok(sender.send(work).await.or_else(|_| Err(ErrorKind::Internal))?)
        } else {
            Err(Error::kind(ErrorKind::Internal))
        }
    }

    pub async fn route(
        &self,
        project_name: &ProjectName,
        route: String,
        req: Request<Body>,
    ) -> Result<Response<Body>, Error> {
        let target_ip = self
            .find_project(project_name)
            .await?
            .target_ip()?
            .unwrap(); // TODO handle
        let resp = hyper_reverse_proxy::call(
            "127.0.0.1".parse().unwrap(),
            &format!("http://{}:{}/{}", target_ip, 8001, route),
            req,
        )
        .await
        .unwrap();
        Ok(resp)
    }

    async fn iter_projects(&self) -> impl Iterator<Item = Work> {
        query("SELECT * FROM projects")
            .fetch_all(&self.db)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Work {
                project_name: row.get("project_name"),
                work: row.get::<SqlxJson<Project>, _>("project_state").0,
                account_name: row.get("account_name"),
            })
    }

    pub async fn find_project(&self, project_name: &ProjectName) -> Result<Project, Error> {
        query("SELECT project_state FROM projects WHERE project_name=?1")
            .bind(project_name)
            .fetch_optional(&self.db)
            .await?
            .map(|r| {
                r.try_get::<SqlxJson<Project>, _>("project_state")
                    .unwrap()
                    .0
            })
            .ok_or_else(|| Error::kind(ErrorKind::ProjectNotFound))
    }

    async fn update_project(
        &self,
        project_name: &ProjectName,
        project: &Project,
    ) -> Result<(), Error> {
        query("UPDATE projects SET project_state = ?1 WHERE project_name = ?2")
            .bind(&SqlxJson(project))
            .bind(project_name)
            .execute(&self.db)
            .await
            .unwrap();
        Ok(())
    }

    pub async fn key_from_account_name(&self, account_name: &AccountName) -> Result<Key, Error> {
        let key = query("SELECT key FROM accounts WHERE account_name = ?1")
            .bind(account_name)
            .fetch_optional(&self.db)
            .await
            .unwrap()
            .map(|row| row.try_get("key").unwrap())
            .unwrap(); // TODO: account not found
        Ok(key)
    }

    pub async fn account_name_from_key(&self, key: &Key) -> Result<AccountName, Error> {
        let name = query("SELECT account_name FROM accounts WHERE key = ?1")
            .bind(key)
            .fetch_optional(&self.db)
            .await
            .unwrap()
            .map(|row| row.try_get("account_name").unwrap())
            .ok_or_else(|| Error::from(ErrorKind::UserNotFound))?;
        Ok(name)
    }

    pub async fn user_from_account_name(&self, name: AccountName) -> Result<User, Error> {
        let key = self.key_from_account_name(&name).await?;
        let projects = self.iter_user_projects(&name).await.collect();
        Ok(User {
            name,
            key,
            projects,
        })
    }

    pub async fn user_from_key(&self, key: Key) -> Result<User, Error> {
        let name = self.account_name_from_key(&key).await?;
        let projects = self.iter_user_projects(&name).await.collect();
        Ok(User {
            name,
            key,
            projects,
        })
    }

    pub async fn create_user(&self, name: AccountName) -> Result<User, Error> {
        let key = Key::new_random();
        query("INSERT INTO accounts (account_name, key) VALUES (?1, ?2)")
            .bind(&name)
            .bind(&key)
            .execute(&self.db)
            .await?;
        Ok(User {
            name,
            key,
            projects: Vec::default(),
        })
    }

    pub async fn is_super_user(&self, account_name: &AccountName) -> Result<bool, Error> {
        let is_super_user = query("SELECT super_user FROM accounts WHERE account_name = ?1")
            .bind(account_name)
            .fetch_optional(&self.db)
            .await?
            .map(|row| row.try_get("super_user").unwrap())
            .unwrap_or(false); // defaults to `false` (i.e. not super user)
        Ok(is_super_user)
    }

    async fn iter_user_projects(
        &self,
        AccountName(account_name): &AccountName,
    ) -> impl Iterator<Item = ProjectName> {
        query("SELECT project_name FROM projects WHERE account_name = ?1")
            .bind(account_name)
            .fetch_all(&self.db)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.try_get::<ProjectName, _>("project_name").unwrap())
    }

    pub async fn create_project(
        &self,
        project_name: ProjectName,
        account_name: AccountName,
    ) -> Result<Project, Error> {
        let initial_key = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        let project = SqlxJson(Project::Creating(project::ProjectCreating::new(
            project_name.clone(),
            self.args.prefix.clone(),
            initial_key.clone(),
        )));

        query("INSERT INTO projects (project_name, account_name, initial_key, project_state) VALUES (?1, ?2, ?3, ?4)")
            .bind(&project_name)
            .bind(&account_name)
            .bind(&initial_key)
            .bind(&project)
            .execute(&self.db)
            .await?;

        let project = project.0;

        let work = Work {
            project_name,
            work: project.clone(),
            account_name,
        };

        self.send(work).await?;

        Ok(project)
    }
}

#[async_trait]
impl<'c> Service<'c> for Arc<GatewayService> {
    type Context = GatewayContext<'c>;

    type State = Work<Project>;

    type Error = Error;

    fn context(&'c self) -> Self::Context {
        GatewayContext {
            docker: &self.docker,
            hyper: &self.hyper,
            args: &self.args
        }
    }

    async fn update(&self, Work { project_name, work, .. }: &Self::State) -> Result<(), Self::Error> {
        self.update_project(project_name, work).await
    }
}

pub struct GatewayContext<'c> {
    docker: &'c Docker,
    hyper: &'c HyperClient<HttpConnector, Body>,
    args: &'c Args
}

impl<'c> Context<'c> for GatewayContext<'c> {
    fn docker(&self) -> &'c Docker {
        self.docker
    }

    fn args(&self) -> &'c Args {
        self.args
    }
}
