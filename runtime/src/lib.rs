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
//! Now that klyra is installed, you can initialize a project with Axum boilerplate:
//! ```bash
//! $ cargo klyra init --template axum my-axum-app
//! ```
//!
//! By looking at the `Cargo.toml` file of the generated `my-axum-app` project you will see it has been made to
//! be a binary crate with a few dependencies including `klyra-runtime` and `klyra-axum`.
//!
//! ```toml
//! klyra-runtime = "0.24.0"
//! axum = "0.6.10"
//! klyra-axum = "0.24.0"
//! tokio = "1.26"
//! ```
//!
//! A boilerplate code for your axum project can also be found in `src/main.rs`:
//!
//! ```rust,no_run
//! use axum::{routing::get, Router};
//!
//! async fn hello_world() -> &'static str {
//!     "Hello, world!"
//! }
//!
//! #[klyra_runtime::main]
//! async fn axum() -> klyra_axum::KlyraAxum {
//!     let router = Router::new().route("/hello", get(hello_world));
//!
//!     Ok(router.into())
//! }
//! ```
//!
//! Check out [our docs](https://docs.klyra.rs/introduction/welcome) to see all the frameworks we support, or
//! our [examples](https://github.com/klyra-hq/klyra-examples) if you prefer that format.
//!
//! ## Running locally
//! To test your app locally before deploying, use:
//!
//! ```bash
//! $ cargo klyra run
//! ```
//!
//! You should see your app build and start on the default port 8000. You can test this using;
//!
//! ```bash
//! $ curl http://localhost:8000/hello
//!
//! Hello, world!
//! ```
//!
//! ## Deploying
//!
//! You can deploy your service with the [`cargo klyra`](https://docs.rs/crate/cargo-klyra/latest) subcommand too.
//! But, you will need to authenticate with the klyra service first using:
//!
//! ```bash
//! $ cargo klyra login
//! ```
//!
//! This will open a browser window and prompt you to connect using your GitHub account.
//!
//! Before you can deploy, you have to create a project. This will start a deployer container for your
//! project under the hood, ensuring isolation from other users' projects. PS. you don't have to do this
//! now if you did in in the `cargo klyra init` flow.
//!
//! ```bash
//! $ cargo klyra project start
//! ```
//!
//! Then, deploy the service with:
//!
//! ```bash
//! $ cargo klyra deploy
//! ```
//!
//! Your service will immediately be available at `{crate_name}.klyraapp.rs`. For example:
//!
//! ```bash
//! $ curl https://my-axum-app.klyraapp.rs/hello
//! Hello, world!
//! ```
//!
//! ## Using `sqlx`
//!
//! Here is a quick example to deploy a rocket service that uses a postgres database and [sqlx](http://docs.rs/sqlx):
//!
//! Initialize a project with Rocket boilerplate:
//! ```bash
//! $ cargo klyra init --template rocket my-rocket-app
//! ```
//!
//! Add `klyra-shared-db` as a dependency with the `postgres` feature, and add `sqlx` as a dependency with the
//! `runtime-tokio-native-tls` and `postgres` features inside `Cargo.toml`:
//!
//! ```toml
//! klyra-shared-db = { version = "0.24.0", features = ["postgres"] }
//! sqlx = { version = "0.7.1", features = ["runtime-tokio-native-tls", "postgres"] }
//! ```
//!
//! Now update the `#[klyra_runtime::main]` function to take in a `PgPool`:
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate rocket;
//!
//! use rocket::State;
//! use sqlx::PgPool;
//! use klyra_rocket::KlyraRocket;
//!
//! struct MyState(PgPool);
//!
//! #[get("/hello")]
//! fn hello(state: &State<MyState>) -> &'static str {
//!     // Do things with `state.0`...
//!     "Hello, Postgres!"
//! }
//!
//! #[klyra_runtime::main]
//! async fn rocket(#[klyra_shared_db::Postgres] pool: PgPool) -> KlyraRocket {
//!     let state = MyState(pool);
//!     let rocket = rocket::build().manage(state).mount("/", routes![hello]);
//!
//!     Ok(rocket.into())
//! }
//! ```
//!
//! For a local run, klyra will automatically provision a Postgres instance inside a [Docker](https://www.docker.com/) container on your machine and connect it to the `PgPool`.
//!
//! For deploys, klyra will provision a database for your application and connect it to the `PgPool` on your behalf.
//!
//! To learn more about klyra managed resources, see our [resource docs](https://docs.klyra.rs/resources/klyra-shared-db).
//!
//! ## Configuration
//!
//! The `cargo klyra` command can be customized by creating a `Klyra.toml` in the same location as your `Cargo.toml`.
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
//! $ cargo klyra deploy --name=$PROJECT_NAME
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

/// Helper macro that generates the entrypoint required by any service - likely the only macro you need in this crate.
///
/// # Without klyra managed resources
/// The simplest usage is when your service does not require any klyra managed resources, so you only need to return a klyra supported service:
///
/// ```rust,no_run
/// use klyra_rocket::KlyraRocket;
///
/// #[klyra_rocket::main]
/// async fn rocket() -> KlyraRocket {
///     let rocket = rocket::build();
///
///     Ok(rocket.into())
/// }
/// ```
///
/// ## klyra supported services
/// The following types can be returned from a `#[klyra_service::main]` function and enjoy first class service support in klyra.
///
/// | Return type                           | Crate                                                         | Service                                     | Version    | Example                                                                               |
/// | ------------------------------------- |-------------------------------------------------------------- | ------------------------------------------- | ---------- | -----------------------------------------------------------------------------------   |
/// | `KlyraActixWeb`                     |[klyra-actix-web](https://crates.io/crates/klyra-actix-web)| [actix-web](https://docs.rs/actix-web/4.3)  | 4.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/actix-web/hello-world)      |
/// | `KlyraAxum`                         |[klyra-axum](https://crates.io/crates/klyra-axum)          | [axum](https://docs.rs/axum/0.6)            | 0.5        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/axum/hello-world)           |
/// | `KlyraPoem`                         |[klyra-poem](https://crates.io/crates/klyra-poem)          | [poem](https://docs.rs/poem/1.3)            | 1.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/poem/hello-world)           |
/// | `KlyraPoise`                        |[klyra-poise](https://crates.io/crates/klyra-poise)        | [poise](https://docs.rs/poise/0.5)          | 0.5        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/poise/hello-world)          |
/// | `KlyraRocket`                       |[klyra-rocket](https://crates.io/crates/klyra-rocket)      | [rocket](https://docs.rs/rocket/0.5.0-rc.2) | 0.5.0-rc.2 | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/rocket/hello-world)         |
/// | `KlyraSalvo`                        |[klyra-salvo](https://crates.io/crates/klyra-salvo)        | [salvo](https://docs.rs/salvo/0.37)         | 0.37       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/salvo/hello-world)          |
/// | `KlyraSerenity`                     |[klyra-serenity](https://crates.io/crates/klyra-serenity   | [serenity](https://docs.rs/serenity/0.11)   | 0.11       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/serenity/hello-world)       |
/// | `KlyraThruster`                     |[klyra-thruster](https://crates.io/crates/klyra-thruster)  | [thruster](https://docs.rs/thruster/1.3)    | 1.3        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/thruster/hello-world)       |
/// | `KlyraTower`                        |[klyra-tower](https://crates.io/crates/klyra-tower)        | [tower](https://docs.rs/tower/0.4)          | 0.4        | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tower/hello-world)          |
/// | `KlyraTide`                         |[klyra-tide](https://crates.io/crates/klyra-tide)          | [tide](https://docs.rs/tide/0.16)           | 0.16       | [GitHub](https://github.com/klyra-hq/klyra-examples/tree/main/tide/hello-world)           |
///
/// # Getting klyra managed resources
/// Klyra is able to manage resource dependencies for you. These resources are passed in as inputs to your `#[klyra_runtime::main]` function and are configured using attributes:
/// ```rust,no_run
/// use sqlx::PgPool;
/// use klyra_rocket::KlyraRocket;
///
/// struct MyState(PgPool);
///
/// #[klyra_runtime::main]
/// async fn rocket(#[klyra_shared_db::Postgres] pool: PgPool) -> KlyraRocket {
///     let state = MyState(pool);
///     let rocket = rocket::build().manage(state);
///
///     Ok(rocket.into())
/// }
/// ```
///
/// More [klyra managed resources can be found here](https://github.com/klyra-hq/klyra/tree/main/resources)
pub use klyra_codegen::main;

mod alpha;
mod args;
#[cfg(feature = "next")]
mod next;
mod provisioner_factory;
mod resource_tracker;

pub use alpha::{start, Alpha};
#[cfg(feature = "next")]
pub use next::{AxumWasm, NextArgs};
pub use provisioner_factory::ProvisionerFactory;
pub use resource_tracker::{get_resource, ResourceTracker};
pub use klyra_common::storage_manager::StorageManager;
pub use klyra_service::{CustomError, Error, Factory, ResourceBuilder, Service};

pub use async_trait::async_trait;

// Dependencies required by the codegen
pub use anyhow::Context;
pub use strfmt::strfmt;

#[cfg(feature = "setup-tracing")]
pub use {colored, tracing_subscriber};

// Print the version of the runtime.
pub fn print_version() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    println!("{name} {version}");
}
