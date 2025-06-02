#![doc = include_str!("../README.md")]
/// A wrapper type for [poem::Endpoint] so we can implement [klyra_runtime::Service] for it.
pub struct PoemService<T>(pub T);

#[klyra_runtime::async_trait]
impl<T> klyra_runtime::Service for PoemService<T>
where
    T: poem::Endpoint + Send + 'static,
{
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), klyra_runtime::Error> {
        poem::Server::new(poem::listener::TcpListener::bind(addr))
            .run(self.0)
            .await
            .map_err(klyra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<T> From<T> for PoemService<T>
where
    T: poem::Endpoint + Send + 'static,
{
    fn from(router: T) -> Self {
        Self(router)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraPoem<T> = Result<PoemService<T>, klyra_runtime::Error>;
