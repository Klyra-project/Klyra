#![doc = include_str!("../README.md")]
use async_trait::async_trait;
pub use klyra_service::SecretStore;
use klyra_service::{
    resource::{ProvisionResourceRequest, KlyraResourceOutput, Type},
    Error, ResourceFactory, ResourceInputBuilder,
};

/// Secrets plugin that provides service secrets
#[derive(Default)]
pub struct Secrets;

#[async_trait]
impl ResourceInputBuilder for Secrets {
    type Input = ProvisionResourceRequest;
    type Output = KlyraResourceOutput<SecretStore>;

    async fn build(self, _factory: &ResourceFactory) -> Result<Self::Input, crate::Error> {
        Ok(ProvisionResourceRequest::new(
            Type::Secrets,
            serde_json::Value::Null,
            serde_json::Value::Null,
        ))
    }
}
