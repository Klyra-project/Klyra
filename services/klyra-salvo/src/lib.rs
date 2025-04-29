//! Klyra service integration for the Salvo web framework.
//! ## Example
//! ```rust,no_run
//! use salvo::prelude::*;
//!
//! #[handler]
//! async fn hello_world(res: &mut Response) {
//!     res.render(Text::Plain("Hello, world!"));
//! }
//!
//! #[klyra_runtime::main]
//! async fn salvo() -> klyra_salvo::KlyraSalvo {
//!     let router = Router::new().get(hello_world);
//!
//!     Ok(router.into())
//! }
//!
//! ```
use klyra_runtime::Error;
use std::net::SocketAddr;

/// A wrapper type for [salvo::Router] so we can implement [klyra_runtime::Service] for it.
pub struct SalvoService(pub salvo::Router);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for SalvoService {
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        use salvo::prelude::*;

        #[handler]
        async fn healthz_handler(res: &mut Response) {
            res.status_code(StatusCode::OK);
        }

        let healthz_router = salvo::Router::with_path("healthz").get(healthz_handler);

        let app = salvo::Router::new().push(healthz_router).push(self.0);

        salvo::Server::new(salvo::conn::TcpListener::new(addr).bind().await)
            .serve(app)
            .await;

        Ok(())
    }
}

impl From<salvo::Router> for SalvoService {
    fn from(router: salvo::Router) -> Self {
        Self(router)
    }
}
/// The return type that should be returned from the [klyra_runtime::main] function.
pub type KlyraSalvo = Result<SalvoService, Error>;
