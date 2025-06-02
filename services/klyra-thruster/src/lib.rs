#![doc = include_str!("../README.md")]
use klyra_runtime::Error;
use std::net::SocketAddr;

/// A wrapper type for [thruster::ThrusterServer] so we can implement [klyra_runtime::Service] for it.
pub struct ThrusterService<T>(pub T);

#[klyra_runtime::async_trait]
impl<T> klyra_runtime::Service for ThrusterService<T>
where
    T: thruster::ThrusterServer + Send + 'static,
{
    /// Takes the server that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        self.0.build(&addr.ip().to_string(), addr.port()).await;

        Ok(())
    }
}

impl<T> From<T> for ThrusterService<T>
where
    T: thruster::ThrusterServer + Send + 'static,
{
    fn from(router: T) -> Self {
        Self(router)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraThruster<T> = Result<ThrusterService<T>, Error>;
