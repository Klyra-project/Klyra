//! Klyra service integration for the Axum web framework.
//!
//! ## Example
//!
//! ```rust,no_run
//! use axum::{routing::get, Router};
//!
//! async fn hello_world() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[klyra_runtime::main]
//! async fn axum() -> klyra_axum::KlyraAxum {
//!     let router = Router::new().route("/", get(hello_world));
//!
//!     Ok(router.into())
//! }
//! ```
use klyra_runtime::{CustomError, Error};
use std::net::SocketAddr;

#[cfg(feature = "axum")]
use axum::Router;
#[cfg(feature = "axum-0-7")]
use axum_0_7::Router;

/// A wrapper type for [axum::Router] so we can implement [klyra_runtime::Service] for it.
pub struct AxumService(pub Router);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for AxumService {
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        #[cfg(feature = "axum")]
        axum::Server::bind(&addr)
            .serve(self.0.into_make_service())
            .await
            .map_err(CustomError::new)?;
        #[cfg(feature = "axum-0-7")]
        axum_0_7::serve(
            klyra_runtime::tokio::net::TcpListener::bind(addr)
                .await
                .map_err(CustomError::new)?,
            self.0,
        )
        .await
        .map_err(CustomError::new)?;

        Ok(())
    }
}

impl From<Router> for AxumService {
    fn from(router: Router) -> Self {
        Self(router)
    }
}

/// Return type from the `[klyra_runtime::main]` macro for a Axum-based service.
///
/// ## Example
///
/// ```rust,no_run
/// use axum::{routing::get, Router};
///
/// async fn hello_world() -> &'static str {
///     "Hello, world!"
/// }
///
/// #[klyra_runtime::main]
/// async fn axum() -> klyra_axum::KlyraAxum {
///     let router = Router::new().route("/", get(hello_world));
///
///     Ok(router.into())
/// }
/// ```
pub type KlyraAxum = Result<AxumService, Error>;
