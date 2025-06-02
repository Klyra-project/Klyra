#![doc = include_str!("../README.md")]
use std::net::SocketAddr;
use std::sync::Arc;

/// A wrapper type for [poise::Framework] so we can implement [klyra_runtime::Service] for it.
pub struct PoiseService<T, E>(pub Arc<poise::Framework<T, E>>);

#[klyra_runtime::async_trait]
impl<T, E> klyra_runtime::Service for PoiseService<T, E>
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    async fn bind(mut self, _addr: SocketAddr) -> Result<(), klyra_runtime::Error> {
        self.0
            .start_autosharded()
            .await
            .map_err(klyra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<T, E> From<Arc<poise::Framework<T, E>>> for PoiseService<T, E> {
    fn from(framework: Arc<poise::Framework<T, E>>) -> Self {
        Self(framework)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraPoise<T, E> = Result<PoiseService<T, E>, klyra_runtime::Error>;
