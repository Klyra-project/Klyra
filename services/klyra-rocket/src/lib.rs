#![doc = include_str!("../README.md")]
use std::net::SocketAddr;

/// A wrapper type for [rocket::Rocket<rocket::Build>] so we can implement [klyra_runtime::Service] for it.
pub struct RocketService(pub rocket::Rocket<rocket::Build>);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for RocketService {
    /// Takes the router that is returned by the user in their [klyra_runtime::main] function
    /// and binds to an address passed in by klyra.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), klyra_runtime::Error> {
        let shutdown = rocket::config::Shutdown {
            ctrlc: false,
            ..rocket::config::Shutdown::default()
        };

        let config = self
            .0
            .figment()
            .clone()
            .merge((rocket::Config::ADDRESS, addr.ip()))
            .merge((rocket::Config::PORT, addr.port()))
            .merge((rocket::Config::LOG_LEVEL, rocket::config::LogLevel::Off))
            .merge((rocket::Config::SHUTDOWN, shutdown));

        let _rocket = self
            .0
            .configure(config)
            .launch()
            .await
            .map_err(klyra_runtime::CustomError::new)?;

        Ok(())
    }
}

impl From<rocket::Rocket<rocket::Build>> for RocketService {
    fn from(router: rocket::Rocket<rocket::Build>) -> Self {
        Self(router)
    }
}

#[doc = include_str!("../README.md")]
pub type KlyraRocket = Result<RocketService, klyra_runtime::Error>;
