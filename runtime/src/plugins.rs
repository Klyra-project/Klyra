use crate::async_trait;
use serde::{Deserialize, Serialize};
use klyra_service::{
    resource::{ProvisionResourceRequestBeta, ResourceTypeBeta},
    DeploymentMetadata, Error, IntoResource, ResourceFactory, ResourceInputBuilder, SecretStore,
};

/// ## Klyra Metadata
///
/// Plugin for getting various metadata at runtime.
///
/// ### Usage
///
/// ```rust,ignore
/// #[klyra_runtime::main]
/// async fn main(
///     #[klyra_runtime::Metadata] metadata: DeploymentMetadata,
/// ) -> __ { ... }
#[derive(Default)]
pub struct Metadata;

#[async_trait]
impl ResourceInputBuilder for Metadata {
    type Input = DeploymentMetadata;
    type Output = DeploymentMetadata;

    async fn build(self, factory: &ResourceFactory) -> Result<Self::Input, Error> {
        Ok(factory.get_metadata())
    }
}

/// ## Klyra Secrets
///
/// Plugin for getting secrets in your [Klyra](https://www.klyra.dev) service.
///
/// ### Usage
///
/// Add a `Secrets.toml` file to the root of your crate with the secrets you'd like to store.
/// Make sure to add `Secrets*.toml` to `.gitignore` to omit your secrets from version control.
///
/// Next, add `#[klyra_runtime::Secrets] secrets: SecretStore` as a parameter to your `klyra_service::main` function.
/// `SecretStore::get` can now be called to retrieve your API keys and other secrets at runtime.
///
/// ### Example
///
/// ```rust,ignore
/// #[klyra_runtime::main]
/// async fn main(
///     #[klyra_runtime::Secrets] secrets: SecretStore
/// ) -> KlyraAxum {
///     // get secret defined in `Secrets.toml` file.
///     let secret = secrets.get("MY_API_KEY").unwrap();
///
///     let router = Router::new()
///         .route("/", || async move { format!("My secret is: {}", secret) });
///
///     Ok(router.into())
/// }
/// ```
#[derive(Default)]
pub struct Secrets;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretsOutputWrapper(SecretStore);

#[async_trait]
impl ResourceInputBuilder for Secrets {
    type Input = ProvisionResourceRequestBeta;
    type Output = SecretsOutputWrapper;

    async fn build(self, _factory: &ResourceFactory) -> Result<Self::Input, Error> {
        Ok(ProvisionResourceRequestBeta {
            r#type: ResourceTypeBeta::Secrets,
            config: serde_json::Value::Null,
        })
    }
}

#[async_trait]
impl IntoResource<SecretStore> for SecretsOutputWrapper {
    async fn into_resource(self) -> Result<SecretStore, Error> {
        Ok(self.0)
    }
}
