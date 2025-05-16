//! Klyra service integration for the Thruster web framework.
//! ## Example
//! ```rust,no_run
//! use thruster::{
//!     context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest},
//!     m, middleware_fn, App, MiddlewareNext, MiddlewareResult,
//! };
//!
//! #[middleware_fn]
//! async fn hello(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
//!     context.body("Hello, World!");
//!     Ok(context)
//! }
//!
//! #[klyra_runtime::main]
//! async fn thruster() -> klyra_thruster::KlyraThruster {
//!     let app = App::<HyperRequest, Ctx, ()>::create(generate_context, ()).get("/", m![hello]);
//!
//!     Ok(app.into())
//! }
//! ```
use klyra_runtime::Error;
use std::net::SocketAddr;
use thruster::{
    context::basic_hyper_context::{BasicHyperContext as Ctx, HyperRequest},
    m, middleware_fn, App, HyperServer, MiddlewareNext, MiddlewareResult, ThrusterServer,
};

#[middleware_fn]
async fn healthz(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    Ok(context)
}

/// A wrapper type for [thruster::ThrusterServer] so we can implement [klyra_runtime::Service] for it.
pub struct ThrusterService(pub App<HyperRequest, Ctx, ()>);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for ThrusterService {
    /// Takes the server that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let server = HyperServer::new(self.0.get("/_klyra/healthz", m![healthz]));

        server.build(&addr.ip().to_string(), addr.port()).await;

        Ok(())
    }
}

impl From<App<HyperRequest, Ctx, ()>> for ThrusterService {
    fn from(app: App<HyperRequest, Ctx, ()>) -> Self {
        Self(app)
    }
}
/// The return type that should be returned from the [klyra_runtime::main] function.
pub type KlyraThruster = Result<ThrusterService, Error>;
