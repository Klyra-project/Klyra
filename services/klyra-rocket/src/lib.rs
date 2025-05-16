//! Klyra service integration for the Rocket web framework.
//! ## Example
//! ```rust,no_run
//! #[macro_use]
//! extern crate rocket;
//!
//! # fn main() {
//! #[get("/")]
//! fn index() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[klyra_runtime::main]
//! async fn rocket() -> klyra_rocket::KlyraRocket {
//!     let rocket = rocket::build().mount("/hello", routes![index]);
//!
//!     Ok(rocket.into())
//! }
//! # }
//! ```
use rocket::http::Status;
use rocket::response::status;
use std::net::SocketAddr;

/// A wrapper type for [rocket::Rocket<rocket::Build>] so we can implement [klyra_runtime::Service] for it.
pub struct RocketService(pub rocket::Rocket<rocket::Build>);

#[rocket::get("/_klyra/healthz")]
fn health_check() -> status::Custom<()> {
    status::Custom(Status::Ok, ())
}

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
            .mount("/", rocket::routes![health_check])
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

/// The return type that should be returned from the [klyra_runtime::main] function.
pub type KlyraRocket = Result<RocketService, klyra_runtime::Error>;
