use async_trait::async_trait;
use klyra_service::{error::Error, resource::Type, Factory, ResourceBuilder};
pub use klyra_service::{DeploymentMetadata as Metadata, Environment};

#[derive(Default)]
pub struct KlyraMetadata;

#[async_trait]
impl ResourceBuilder for KlyraMetadata {
    const TYPE: Type = Type::Metadata;
    type Config = ();
    type Output = Metadata;

    fn config(&self) -> &Self::Config {
        &()
    }

    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, Error> {
        Ok(factory.get_metadata())
    }
}
