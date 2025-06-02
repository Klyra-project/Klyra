#![doc = include_str!("../README.md")]
use klyra_runtime::{CustomError, Error};
use std::net::SocketAddr;

/// A wrapper type for [tide::Server<T] so we can implement [klyra_runtime::Service] for it.
pub struct TideService<T>(pub tide::Server<T>);

#[klyra_runtime::async_trait]
impl<T> klyra_runtime::Service for TideService<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        self.0.listen(addr).await.map_err(CustomError::new)?;

        Ok(())
    }
}

impl<T> From<tide::Server<T>> for TideService<T> {
    fn from(router: tide::Server<T>) -> Self {
        Self(router)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraTide<T> = Result<TideService<T>, Error>;
