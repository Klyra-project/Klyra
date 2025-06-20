use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use klyra_common::constants::STORAGE_DIRNAME;
pub use klyra_common::{
    database,
    deployment::{DeploymentMetadata, Environment},
    resource::{self, KlyraResourceOutput},
    secrets::Secret,
    ContainerRequest, ContainerResponse, DatabaseInfo, DatabaseResource, DbInput, SecretStore,
};

pub use crate::error::{CustomError, Error};

#[cfg(feature = "builder")]
pub mod builder;
pub mod error;
#[cfg(feature = "runner")]
pub mod runner;

/// Allows implementing plugins for the Klyra main function.
///
/// ## Creating your own Klyra plugin
///
/// You can add your own implementation of this trait along with [`IntoResource`] to customize the
/// input type `R` that gets into the Klyra main function on an existing resource.
///
/// You can also make your own plugin, for example to generalise the connection logic to a third-party service.
/// One example of this is `klyra-qdrant`.
///
/// Please refer to `klyra-examples/custom-resource` for examples of how to create a custom resource. For more advanced provisioning
/// of custom resources, please [get in touch](https://discord.gg/klyra) and detail your use case. We'll be interested to see what you
/// want to provision and how to do it on your behalf on the fly.
#[async_trait]
pub trait ResourceInputBuilder: Default {
    /// The input for requesting this resource.
    ///
    /// If the input is a [`klyra_common::resource::ProvisionResourceRequest`],
    /// then the resource will be provisioned and the [`ResourceInputBuilder::Output`]
    /// will be a [`klyra_common::resource::KlyraResourceOutput<T>`] with the resource's associated output type.
    type Input: Serialize + DeserializeOwned;

    /// The output from provisioning this resource.
    ///
    /// For custom resources that don't provision anything from Klyra,
    /// this should be the same type as [`ResourceInputBuilder::Input`].
    ///
    /// This type must implement [`IntoResource`] for the desired final resource type `R`.
    type Output: Serialize + DeserializeOwned;

    /// Construct this resource config. The [`ResourceFactory`] provides access to secrets and metadata.
    async fn build(self, factory: &ResourceFactory) -> Result<Self::Input, crate::Error>;
}

/// A factory for getting metadata when building resources
pub struct ResourceFactory {
    project_name: String,
    secrets: BTreeMap<String, Secret<String>>,
    env: Environment,
}

impl ResourceFactory {
    pub fn new(
        project_name: String,
        secrets: BTreeMap<String, Secret<String>>,
        env: Environment,
    ) -> Self {
        Self {
            project_name,
            secrets,
            env,
        }
    }

    pub fn get_secrets(&self) -> BTreeMap<String, Secret<String>> {
        self.secrets.clone()
    }

    pub fn get_metadata(&self) -> DeploymentMetadata {
        DeploymentMetadata {
            env: self.env,
            project_name: self.project_name.to_string(),
            storage_path: PathBuf::from(STORAGE_DIRNAME),
        }
    }
}

/// Implement this on an [`ResourceInputBuilder::Output`] type to turn the
/// base resource into the end type exposed to the Klyra main function.
#[async_trait]
pub trait IntoResource<R>: Serialize + DeserializeOwned {
    /// Initialize any logic for creating the final resource of type `R` from the base resource.
    ///
    /// Example: turn a connection string into a connection pool.
    async fn into_resource(self) -> Result<R, crate::Error>;
}

// Base impls for [`ResourceInputBuilder::Output`] types that don't need to convert into anything else
#[async_trait]
impl<R: Serialize + DeserializeOwned + Send> IntoResource<R> for R {
    async fn into_resource(self) -> Result<R, crate::Error> {
        Ok(self)
    }
}
#[async_trait]
impl<R: Serialize + DeserializeOwned + Send> IntoResource<R> for KlyraResourceOutput<R> {
    async fn into_resource(self) -> Result<R, crate::Error> {
        Ok(self.output)
    }
}

/// The core trait of the Klyra platform. Every service deployed to Klyra needs to implement this trait.
///
/// An `Into<Service>` implementor is what is returned in the `klyra_runtime::main` macro
/// in order to run it on the Klyra servers.
#[async_trait]
pub trait Service: Send {
    /// This function is run exactly once on startup of a deployment.
    ///
    /// The passed [`SocketAddr`] receives proxied HTTP traffic from you Klyra subdomain (or custom domain).
    /// Binding to the address is only relevant if this service is an HTTP server.
    async fn bind(mut self, addr: SocketAddr) -> Result<(), error::Error>;
}
