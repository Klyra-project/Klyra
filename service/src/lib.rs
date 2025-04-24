use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use async_trait::async_trait;

pub mod error;
pub use error::{CustomError, Error};

use serde::{de::DeserializeOwned, Serialize};
pub use klyra_common::{
    database, resource::Type, DatabaseReadyInfo, DbInput, DbOutput, SecretStore,
};

#[cfg(feature = "codegen")]
extern crate klyra_codegen;
#[cfg(feature = "codegen")]
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
/// | `KlyraActixWeb`                     |[klyra-actix-web](https://crates.io/crates/klyra-actix-web)| [actix-web](https://docs.rs/actix-web/4.3)  | 4.3        | [GitHub](https://github.com/klyra-hq/examples/tree/main/actix-web/hello-world)      |
/// | `KlyraAxum`                         |[klyra-axum](https://crates.io/crates/klyra-axum)          | [axum](https://docs.rs/axum/0.6)            | 0.5        | [GitHub](https://github.com/klyra-hq/examples/tree/main/axum/hello-world)           |
/// | `KlyraPoem`                         |[klyra-poem](https://crates.io/crates/klyra-poem)          | [poem](https://docs.rs/poem/1.3)            | 1.3        | [GitHub](https://github.com/klyra-hq/examples/tree/main/poem/hello-world)           |
/// | `KlyraPoise`                        |[klyra-poise](https://crates.io/crates/klyra-poise)        | [poise](https://docs.rs/poise/0.5)          | 0.5        | [GitHub](https://github.com/klyra-hq/examples/tree/main/poise/hello-world)          |
/// | `KlyraRocket`                       |[klyra-rocket](https://crates.io/crates/klyra-rocket)      | [rocket](https://docs.rs/rocket/0.5.0-rc.2) | 0.5.0-rc.2 | [GitHub](https://github.com/klyra-hq/examples/tree/main/rocket/hello-world)         |
/// | `KlyraSalvo`                        |[klyra-salvo](https://crates.io/crates/klyra-salvo)        | [salvo](https://docs.rs/salvo/0.37)         | 0.37       | [GitHub](https://github.com/klyra-hq/examples/tree/main/salvo/hello-world)          |
/// | `KlyraSerenity`                     |[klyra-serenity](https://crates.io/crates/klyra-serenity   | [serenity](https://docs.rs/serenity/0.11)   | 0.11       | [GitHub](https://github.com/klyra-hq/examples/tree/main/serenity/hello-world)       |
/// | `KlyraThruster`                     |[klyra-thruster](https://crates.io/crates/klyra-thruster)  | [thruster](https://docs.rs/thruster/1.3)    | 1.3        | [GitHub](https://github.com/klyra-hq/examples/tree/main/thruster/hello-world)       |
/// | `KlyraTower`                        |[klyra-tower](https://crates.io/crates/klyra-tower)        | [tower](https://docs.rs/tower/0.4)          | 0.4        | [GitHub](https://github.com/klyra-hq/examples/tree/main/tower/hello-world)          |
/// | `KlyraTide`                         |[klyra-tide](https://crates.io/crates/klyra-tide)          | [tide](https://docs.rs/tide/0.16)           | 0.16       | [GitHub](https://github.com/klyra-hq/examples/tree/main/tide/hello-world)           |
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

#[cfg(feature = "builder")]
pub mod builder;

pub use klyra_common::{deployment::Environment, project::ProjectName as ServiceName};

/// Factories can be used to request the provisioning of additional resources (like databases).
///
/// An instance of factory is passed by the deployer as an argument to [ResourceBuilder::build][ResourceBuilder::output] in the initial phase of deployment.
///
/// Also see the [main][main] macro.
#[async_trait]
pub trait Factory: Send + Sync {
    /// Get a database connection
    async fn get_db_connection(
        &mut self,
        db_type: database::Type,
    ) -> Result<DatabaseReadyInfo, crate::Error>;

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
/// Your resource will be available on a [klyra_runtime::main][main] function as follow:
/// ```
/// #[klyra_runtime::main]
/// async fn my_service([custom_resource_crate::namespace::B] custom_resource: T)
///     -> klyra_axum::KlyraAxum {}
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
///     const TYPE: Type = Type::Custom;
///
///     type Config = Self;
///
///     type Output = String;
///
///     fn new() -> Self {
///         Self {
///             name: String::new(),
///         }
///     }
///
///     fn config(&self) -> &Self::Config {
///         &self
///     }
///
///
///     async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, klyra_service::Error> {
///         Ok(self.name)
///     }
///
///     async fn build(build_data: &Self::Output) -> Result<Resource, klyra_service::Error> {
///         Ok(Resource { name: build_data })
///     }
/// }
/// ```
///
/// Then using this resource in a service:
/// ```
/// #[klyra_runtime::main]
/// async fn my_service(
///     [custom_resource_crate::Builder(name = "John")] resource: custom_resource_crate::Resource
/// )
///     -> klyra_axum::KlyraAxum {}
/// ```
#[async_trait]
pub trait ResourceBuilder<T> {
    /// The type of resource this creates
    const TYPE: Type;

    /// The internal config being constructed by this builder. This will be used to find cached [Self::Output].
    type Config: Serialize;

    /// The output type used to build this resource later
    type Output: Serialize + DeserializeOwned;

    /// Create a new instance of this resource builder
    fn new() -> Self;

    /// Get the internal config state of the builder
    ///
    /// If the exact same config was returned by a previous deployement that used this resource, then [Self::output()]
    /// will not be called to get the builder output again. Rather the output state of the previous deployment
    /// will be passed to [Self::build()].
    fn config(&self) -> &Self::Config;

    /// Get the config output of this builder
    ///
    /// This method is where the actual resource provisioning should take place and is expected to take the longest. It
    /// can at times even take minutes. That is why the output of this method is cached and calling this method can be
    /// skipped as explained in [Self::config()].
    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, crate::Error>;

    /// Build this resource from its config output
    async fn build(build_data: &Self::Output) -> Result<T, crate::Error>;
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
pub const RUNTIME_NAME: &str = "klyra-runtime";
