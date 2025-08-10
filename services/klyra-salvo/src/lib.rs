#![doc = include_str!("../README.md")]
use salvo::Listener;
use klyra_runtime::Error;
use std::net::SocketAddr;

pub use salvo;

/// A wrapper type for [salvo::Router] so we can implement [klyra_runtime::Service] for it.
pub struct SalvoService(pub salvo::Router);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for SalvoService {
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let listener = salvo::conn::TcpListener::new(addr).bind().await;

        salvo::Server::new(listener).serve(self.0).await;

        Ok(())
    }
}

impl From<salvo::Router> for SalvoService {
    fn from(router: salvo::Router) -> Self {
        Self(router)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraSalvo = Result<SalvoService, Error>;
