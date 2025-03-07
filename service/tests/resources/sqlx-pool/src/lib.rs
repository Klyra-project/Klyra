use klyra_service::error::CustomError;
use klyra_service::{log, IntoService, ResourceBuilder, Runtime, ServeHandle, Service};
use sqlx::PgPool;

#[macro_use]
extern crate klyra_service;

struct Args;

struct PoolService {
    runtime: Runtime,
    pool: Option<PgPool>,
}

fn init() -> Args {
    Args
}

impl IntoService for Args {
    type Service = PoolService;

    fn into_service(self) -> Self::Service {
        PoolService {
            pool: None,
            runtime: Runtime::new().unwrap(),
        }
    }
}

async fn start(pool: PgPool) -> Result<(), klyra_service::error::CustomError> {
    let (rec,): (String,) = sqlx::query_as("SELECT 'Hello world'")
        .fetch_one(&pool)
        .await
        .map_err(CustomError::new)?;

    assert_eq!(rec, "Hello world");

    Ok(())
}

#[async_trait]
impl Service for PoolService {
    fn bind(
        &mut self,
        _: std::net::SocketAddr,
    ) -> Result<ServeHandle, klyra_service::error::Error> {
        let launch = start(self.pool.take().expect("we should have an active pool"));
        let handle = self.runtime.spawn(launch);

        Ok(handle)
    }

    async fn build(
        &mut self,
        factory: &mut dyn klyra_service::Factory,
        logger: Box<dyn log::Log>,
    ) -> Result<(), klyra_service::Error> {
        self.runtime
            .spawn_blocking(move || {
                klyra_service::log::set_boxed_logger(logger)
                    .map(|()| {
                        klyra_service::log::set_max_level(klyra_service::log::LevelFilter::Info)
                    })
                    .expect("logger set should succeed");
            })
            .await
            .unwrap();

        let pool = klyra_service::shared::Postgres::new()
            .build(factory, &self.runtime)
            .await?;

        self.pool = Some(pool);

        Ok(())
    }
}

declare_service!(Args, init);
