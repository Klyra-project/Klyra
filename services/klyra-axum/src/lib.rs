//! Klyra service integration for the Axum web framework.
//! ## Example
//! ```rust,no_run
//! use axum::{routing::get, Router};
//!
//! async fn hello_world() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[klyra_runtime::main]
//! async fn axum() -> klyra_axum::KlyraAxum {
//!     let router = Router::new().route("/hello", get(hello_world));
//!
//!     Ok(router.into())
//! }
//! ```
use klyra_runtime::{CustomError, Error};
use std::net::SocketAddr;

/// A wrapper type for [axum::Router] so we can implement [klyra_runtime::Service] for it.
pub struct AxumService(pub axum::Router);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for AxumService {
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        axum::Server::bind(&addr)
            .serve(self.0.into_make_service())
            .await
            .map_err(CustomError::new)?;

        Ok(())
    }
}

impl From<axum::Router> for AxumService {
    fn from(router: axum::Router) -> Self {
        Self(router)
    }
}
/// The return type that should be returned from the [klyra_runtime::main] function.
pub type KlyraAxum = Result<AxumService, Error>;
