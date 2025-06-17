use async_trait::async_trait;
pub use klyra_service::{DeploymentMetadata as Metadata, Environment, SecretStore};
use klyra_service::{Error, ResourceFactory, ResourceInputBuilder};

#[derive(Default)]
pub struct KlyraMetadata;

#[async_trait]
impl ResourceInputBuilder for KlyraMetadata {
    type Input = Metadata;
    type Output = Metadata;

    async fn build(self, factory: &ResourceFactory) -> Result<Self::Input, crate::Error> {
        Ok(factory.get_metadata())
    }
}
