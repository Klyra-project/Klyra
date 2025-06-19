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
//! axum = "0.7.3"
//! klyra-axum = "0.41.0"
//! klyra-runtime = "0.41.0"
//! tokio = "1.28.2"
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
//! async fn main() -> klyra_axum::KlyraAxum {
//!     let router = Router::new().route("/", get(hello_world));
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
//! $ curl http://localhost:8000/
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
//! $ curl https://my-axum-app.klyraapp.rs/
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
//! klyra-shared-db = { version = "0.41.0", features = ["postgres"] }
//! sqlx = "0.7.1"
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
//! #[get("/")]
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
//! ## Join Discord
//!
//! If you have any questions, [join our Discord server](https://discord.gg/klyra). There's always someone on there that can help!
//!
//! You can also [open an issue or a discussion on GitHub](https://github.com/klyra-hq/klyra).

// Public API
pub use klyra_codegen::main;
pub use klyra_service::{
    CustomError, DbInput, Error, IntoResource, ResourceFactory, ResourceInputBuilder, Service,
};

// Useful re-exports
pub use async_trait::async_trait;
pub use tokio;

mod alpha;
mod args;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Not part of public API
#[doc(hidden)]
pub mod __internals {
    // Internals used by the codegen
    pub use crate::alpha::{start, Alpha};

    // Dependencies required by the codegen
    pub use anyhow::Context;
    #[cfg(feature = "setup-tracing")]
    pub use colored;
    pub use serde_json;
    pub use strfmt::strfmt;
    #[cfg(feature = "setup-tracing")]
    pub use tracing_subscriber;

    // Print the version of the runtime.
    pub fn print_version() {
        println!("{} {}", crate::NAME, crate::VERSION);
    }
}
