//! Klyra service integration for the Warp web framework.
//! ## Example
//! ```rust,no_run
//! use warp::Filter;
//! use warp::Reply;
//!
//! #[klyra_runtime::main]
//! async fn warp() -> klyra_warp::KlyraWarp<(impl Reply,)> {
//!     let route = warp::any().map(|| "Hello, World!");
//!     Ok(route.boxed().into())
//! }
//! ```
use klyra_runtime::Error;
use std::net::SocketAddr;
use std::ops::Deref;

/// A wrapper type for [warp::Filter] so we can implement [klyra_runtime::Service] for it.
pub struct WarpService<T>(pub T);

#[klyra_runtime::async_trait]
impl<T> klyra_runtime::Service for WarpService<T>
where
    T: Send + Sync + Clone + 'static + warp::Filter<Error = warp::Rejection>,
    T::Extract: warp::reply::Reply,
{
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        use warp::Filter;

        let app_filter = (*self).clone();
        let health = warp::path("healthz")
            .and(warp::get())
            .and(warp::path::end())
            .map(warp::reply);

        warp::serve(health.or(app_filter)).bind(addr).await;
        Ok(())
    }
}

impl<T> From<T> for WarpService<T>
where
    T: Send + Sync + Clone + 'static + warp::Filter<Error = warp::Rejection>,
    T::Extract: warp::reply::Reply,
{
    fn from(router: T) -> Self {
        Self(router)
    }
}

impl<T> Deref for WarpService<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The return type that should be returned from the [klyra_runtime::main] function.
pub type KlyraWarp<T> = Result<WarpService<warp::filters::BoxedFilter<T>>, Error>;
