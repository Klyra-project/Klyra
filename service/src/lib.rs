#![doc(
    html_logo_url = "https://raw.githubusercontent.com/klyra-hq/klyra/main/assets/logo-square-transparent.png",
    html_favicon_url = "https://raw.githubusercontent.com/klyra-hq/klyra/main/assets/favicon.ico"
)]
//! # Klyra - Deploy Rust apps with a single Cargo subcommand
//! <div style="display: flex; margin-top: 30px; margin-bottom: 30px;">
//! <img src="https://raw.githubusercontent.com/klyra-hq/klyra/main/assets/logo-rectangle-transparent.png" width="400px" style="margin-left: auto; margin-right: auto;"/>
//! </div>
//!
//! Hello, and welcome to the <span style="font-family: Sans-Serif;"><a href="https://klyra.rs">klyra</a></span> API documentation!
//!
//! Klyra is an open-source app platform that uses traits and annotations to configure your backend deployments.
//!
//! ## Usage
//! Start by installing the [`cargo klyra`](https://docs.rs/crate/cargo-klyra/latest) subcommand by running the following in a terminal:
//!
//! ```bash
//! $ cargo install cargo-klyra
//! ```
//!
//! Now that klyra is installed, you can initialize a project with Rocket boilerplate:
//! ```bash
//! $ cargo klyra init --rocket my-rocket-app
//! ```
//!
//! By looking at the `Cargo.toml` file of the generated `my-rocket-app` project you will see it has been made to
//! be a library crate with a `klyra-service` dependency with the `web-rocket` feature on the `klyra-service` dependency.
//!
//! ```toml
//! klyra-service = { version = "0.11.0", features = ["web-rocket"] }
//! ```
//!
//! A boilerplate code for your rocket project can also be found in `src/lib.rs`:
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate rocket;
//!
//! use klyra_service::KlyraRocket;
//!
//! #[get("/hello")]
//! fn hello() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[klyra_service::main]
//! async fn init() -> KlyraRocket {
//!     let rocket = rocket::build().mount("/", routes![hello]);
//!
//!     Ok(rocket)
//! }
//! ```
//!
//! See the [klyra_service::main][main] macro for more information on supported services - such as `axum`.
//! Or look at [more complete examples](https://github.com/klyra-hq/examples), but
//! take note that the examples may update before official releases.
//!
//! ## Running locally
//! To test your app locally before deploying, use:
//!
//! ```bash
//! $ cargo klyra run
//! ```
//!
//! You should see your app build and start on the default port 8000. You can test this using;
//!
//! ```bash
//! $ curl http://localhost:8000/hello
//! Hello, world!
//! ```
//!
//! ## Deploying
//!
//! You can deploy your service with the [`cargo klyra`](https://docs.rs/crate/cargo-klyra/latest) subcommand too.
//! But, you will need to authenticate with the klyra service first using:
//!
//! ```bash
//! $ cargo klyra login
//! ```
//!
//! this will open a browser window and prompt you to connect using your GitHub account.
//!
//! Before you can deploy, you have to create a project. This will start a deployer container for your
//! project under the hood, ensuring isolation from other users' projects.
//!
//! ```bash
//! $ cargo klyra project new
//! ```
//!
//! Then, deploy the service with:
//!
//! ```bash
//! $ cargo klyra deploy
//! ```
//!
//! Your service will immediately be available at `{crate_name}.klyraapp.rs`. For example:
//!
//! ```bash
//! $ curl https://my-rocket-app.klyraapp.rs/hello
//! Hello, world!
//! ```
//!
//! ## Using `sqlx`
//!
//! Here is a quick example to deploy a service that uses a postgres database and [sqlx](http://docs.rs/sqlx):
//!
//! Add `klyra-shared-db` as a dependency with the `postgres` feature, and add `sqlx` as a dependency with the `runtime-tokio-native-tls` and `postgres` features inside `Cargo.toml`:
//!
//! ```toml
//! klyra-shared-db = { version = "0.11.0", features = ["postgres"] }
//! sqlx = { version = "0.6.2", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
//!
//! Now update the `#[klyra_service::main]` function to take in a `PgPool`:
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate rocket;
//!
//! use rocket::State;
//! use sqlx::PgPool;
//! use klyra_service::KlyraRocket;
//!
//! struct MyState(PgPool);
//!
//! #[get("/hello")]
//! fn hello(state: &State<MyState>) -> &'static str {
//!     // Do things with `state.0`...
//!     "Hello, Postgres!"
//! }
//!
//! #[klyra_service::main]
//! async fn rocket(#[klyra_shared_db::Postgres] pool: PgPool) -> KlyraRocket {
//!     let state = MyState(pool);
//!     let rocket = rocket::build().manage(state).mount("/", routes![hello]);
//!
//!     Ok(rocket)
//! }
//! ```
//!
//! For a local run, klyra will automatically provision a Postgres instance inside a [Docker](https://www.docker.com/) container on your machine and connect it to the `PgPool`.
//!
//! For deploys, klyra will provision a database for your application and connect it to the `PgPool` on your behalf.
//!
//! To learn more about klyra managed resources, see [klyra_service::main][main#getting-klyra-managed-resources].
//!
//! ## Configuration
//!
//! The `cargo klyra` command can be customised by creating a `Klyra.toml` in the same location as your `Cargo.toml`.
//!
//! ##### Change the name of your service
//!
//! To have your service deployed with a different name, add a `name` entry in the `Klyra.toml`:
//!
//! ```toml
//! name = "hello-world"
//! ```
//!
//! If the `name` key is not specified, the service's name will be the same as the crate's name.
//!
//! Alternatively, you can override the project name on the command-line, by passing the --name argument to any subcommand like so:
//!
//! ```bash
//! cargo klyra deploy --name=$PROJECT_NAME
//! ```
//!
//! ##### Using Podman instead of Docker
//! If you are using [Podman](https://podman.io/) instead of Docker, then `cargo klyra run` will give
//! `got unexpected error while inspecting docker container: error trying to connect: No such file or directory` error.
//!
//! To fix this error you will need to expose a rootless socket for Podman first. This can be done using:
//!
//! ```bash
//! podman system service --time=0 unix:///tmp/podman.sock
//! ```
//!
//! Now set the `DOCKER_HOST` environment variable to point to this socket using:
//!
//! ```bash
//! export DOCKER_HOST=unix:///tmp/podman.sock
//! ```
//!
//! Now all `cargo klyra run` commands will work against Podman.
//!
//! ## Getting API keys
//!
//! After you've installed the [cargo-klyra](https://docs.rs/crate/cargo-klyra/latest) command, run:
//!
//! ```bash
//! $ cargo klyra login
//! ```
//!
//! this will open a browser window and prompt you to connect using your GitHub account.
//!
//! ## We're in alpha 🤗
//!
//! Thanks for using klyra! We're very happy to have you with us!
//!
//! During our alpha period, API keys are completely free and you can deploy as many services as you want.
//!
//! Just keep in mind that there may be some kinks that require us to take all deployments down once in a while. In certain circumstances we may also have to delete all the data associated with those deployments.
//!
//! To stay updated with the release status of klyra, [join our Discord](https://discord.gg/klyra)!
//!
//! ## Join Discord
//!
//! If you have any questions, [join our Discord server](https://discord.gg/klyra). There's always someone on there that can help!
//!
//! You can also [open an issue or a discussion on GitHub](https://github.com/klyra-hq/klyra).
//!

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use async_trait::async_trait;

pub mod error;
pub use error::{CustomError, Error};

pub use klyra_common::database;

#[cfg(feature = "codegen")]
extern crate klyra_codegen;
#[cfg(feature = "codegen")]
/// Helper macro that generates the entrypoint required by any service - likely the only macro you need in this crate.
///
/// # Without klyra managed resources
/// The simplest usage is when your service does not require any klyra managed resources, so you only need to return a klyra supported service:
///
/// ```rust,no_run
/// use klyra_service::KlyraRocket;
///
/// #[klyra_service::main]
/// async fn rocket() -> KlyraRocket {
///     let rocket = rocket::build();
///
///     Ok(rocket)
/// }
/// ```
///
/// ## klyra supported services
/// The following types can be returned from a `#[klyra_service::main]` function and enjoy first class service support in klyra. Be sure to also enable the correct feature on
/// `klyra-service` in `Cargo.toml` for the type to be recognized.
///
/// | Return type                           | Feature flag | Service                                     | Version    | Example                                                                               |
/// | ------------------------------------- | ------------ | ------------------------------------------- | ---------- | -----------------------------------------------------------------------------------   |
/// | `KlyraRocket`                       | web-rocket   | [rocket](https://docs.rs/rocket/0.5.0-rc.2) | 0.5.0-rc.2 | [GitHub](https://github.com/klyra-hq/examples/tree/main/rocket/hello-world)         |
/// | `KlyraAxum`                         | web-axum     | [axum](https://docs.rs/axum/0.5)            | 0.5        | [GitHub](https://github.com/klyra-hq/examples/tree/main/axum/hello-world)           |
/// | `KlyraSalvo`                        | web-salvo    | [salvo](https://docs.rs/salvo/0.34.3)       | 0.34.3     | [GitHub](https://github.com/klyra-hq/examples/tree/main/salvo/hello-world)          |
/// | `KlyraTide`                         | web-tide     | [tide](https://docs.rs/tide/0.16.0)         | 0.16.0     | [GitHub](https://github.com/klyra-hq/examples/tree/main/tide/hello-world)           |
/// | `KlyraPoem`                         | web-poem     | [poem](https://docs.rs/poem/1.3.35)         | 1.3.35     | [GitHub](https://github.com/klyra-hq/examples/tree/main/poem/hello-world)           |
/// | `Result<T, klyra_service::Error>`   | web-tower    | [tower](https://docs.rs/tower/0.4.12)       | 0.14.12    | [GitHub](https://github.com/klyra-hq/examples/tree/main/tower/hello-world)          |
/// | `KlyraSerenity`                     | bot-serenity | [serenity](https://docs.rs/serenity/0.11.5) | 0.11.5     | [GitHub](https://github.com/klyra-hq/examples/tree/main/serenity/hello-world)       |
/// | `KlyraPoise`                        | bot-poise    | [poise](https://docs.rs/poise/0.5.2)        | 0.5.2      | [GitHub](https://github.com/klyra-hq/examples/tree/main/poise/hello-world)          |
/// | `KlyraActixWeb`                     | web-actix-web| [actix-web](https://docs.rs/actix-web/4.2.1)| 4.2.1      | [GitHub](https://github.com/klyra-hq/examples/tree/main/actix-web/hello-world)      |
///
/// # Getting klyra managed resources
/// Klyra is able to manage resource dependencies for you. These resources are passed in as inputs to your `#[klyra_service::main]` function and are configured using attributes:
/// ```rust,no_run
/// use sqlx::PgPool;
/// use klyra_service::KlyraRocket;
///
/// struct MyState(PgPool);
///
/// #[klyra_service::main]
/// async fn rocket(#[klyra_shared_db::Postgres] pool: PgPool) -> KlyraRocket {
///     let state = MyState(pool);
///     let rocket = rocket::build().manage(state);
///
///     Ok(rocket)
/// }
/// ```
///
/// More [klyra managed resources can be found here](https://github.com/klyra-hq/klyra/tree/main/resources)
pub use klyra_codegen::main;

#[cfg(feature = "builder")]
pub mod builder;

pub use klyra_common::{deployment::Environment, project::ProjectName as ServiceName};

/// Factories can be used to request the provisioning of additional resources (like databases).
///
/// An instance of factory is passed by the deployer as an argument to [ResourceBuilder::build][ResourceBuilder::build] in the initial phase of deployment.
///
/// Also see the [main][main] macro.
#[async_trait]
pub trait Factory: Send + Sync {
    /// Declare that the [Service][Service] requires a database.
    ///
    /// Returns the connection string to the provisioned database.
    async fn get_db_connection_string(
        &mut self,
        db_type: database::Type,
    ) -> Result<String, crate::Error>;

    /// Get all the secrets for a service
    async fn get_secrets(&mut self) -> Result<BTreeMap<String, String>, crate::Error>;

    /// Get the name for the service being deployed
    fn get_service_name(&self) -> ServiceName;

    /// Get the environment for this deployment
    fn get_environment(&self) -> Environment;

    /// Get the path where the build files are stored for this service
    fn get_build_path(&self) -> Result<PathBuf, crate::Error>;

    /// Get the path where files can be stored for this deployment
    fn get_storage_path(&self) -> Result<PathBuf, crate::Error>;
}

/// Used to get resources of type `T` from factories.
///
/// This is mainly meant for consumption by our code generator and should generally not be called by users.
///
/// ## Creating your own managed resource
/// You may want to create your own managed resource by implementing this trait for some builder `B` to construct resource `T`. [`Factory`] can be used to provision resources
/// on klyra's servers if your resource will need any.
///
/// Your resource will be available on a [klyra_service::main][main] function as follow:
/// ```
/// #[klyra_service::main]
/// async fn my_service([custom_resource_crate::namespace::B] custom_resource: T)
///     -> klyra_service::KlyraAxum {}
/// ```
///
/// Here `custom_resource_crate::namespace` is the crate and namespace to a builder `B` that implements [`ResourceBuilder`] to create resource `T`.
///
/// ### Example
/// ```
/// pub struct Builder {
///     name: String,
/// }
///
/// pub struct Resource {
///     name: String,
/// }
///
/// impl Builder {
///     /// Name to give resource
///     pub fn name(self, name: &str) -> Self {
///         self.name = name.to_string();
///
///         self
///     }
/// }
///
/// #[async_trait]
/// impl ResourceBuilder<Resource> for Builder {
///     fn new() -> Self {
///         Self {
///             name: String::new(),
///         }
///     }
///
///     async fn build(
///         self,
///         factory: &mut dyn Factory,
///     ) -> Result<Resource, klyra_service::Error> {
///         Ok(Resource { name: self.name })
///     }
/// }
/// ```
///
/// Then using this resource in a service:
/// ```
/// #[klyra_service::main]
/// async fn my_service(
///     [custom_resource_crate::Builder(name = "John")] resource: custom_resource_crate::Resource
/// )
///     -> klyra_service::KlyraAxum {}
/// ```
#[async_trait]
pub trait ResourceBuilder<T> {
    fn new() -> Self;
    async fn build(self, factory: &mut dyn Factory) -> Result<T, crate::Error>;
}

/// The core trait of the klyra platform. Every crate deployed to klyra needs to implement this trait.
///
/// Use the [main][main] macro to expose your implementation to the deployment backend.
#[async_trait]
pub trait Service: Send {
    /// This function is run exactly once on each instance of a deployment.
    ///
    /// The deployer expects this instance of [Service][Service] to bind to the passed [SocketAddr][SocketAddr].
    async fn bind(mut self, addr: SocketAddr) -> Result<(), error::Error>;
}

pub const NEXT_NAME: &str = "klyra-next";
