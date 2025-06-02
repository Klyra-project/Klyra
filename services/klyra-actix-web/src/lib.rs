#![doc = include_str!("../README.md")]
use std::net::SocketAddr;

/// A wrapper type for a closure that returns an [actix_web::web::ServiceConfig] so we can implement
/// [klyra_runtime::Service] for it.
#[derive(Clone)]
pub struct ActixWebService<F>(pub F);

#[klyra_runtime::async_trait]
impl<F> klyra_runtime::Service for ActixWebService<F>
where
    F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
{
    async fn bind(mut self, addr: SocketAddr) -> Result<(), klyra_runtime::Error> {
        // Start a worker for each cpu, but no more than 4.
        let worker_count = num_cpus::get().min(4);

        let server =
            actix_web::HttpServer::new(move || actix_web::App::new().configure(self.0.clone()))
                .workers(worker_count)
                .bind(addr)?
                .run();

        server.await.map_err(klyra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<F> From<F> for ActixWebService<F>
where
    F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
{
    fn from(service_config: F) -> Self {
        Self(service_config)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraActixWeb<F> = Result<ActixWebService<F>, klyra_runtime::Error>;
