use async_trait::async_trait;
pub use klyra_service::DeploymentMetadata as Metadata;
use klyra_service::{error::Error, Factory, ResourceBuilder, Type};

pub struct KlyraMetadata;

#[async_trait]
impl ResourceBuilder<Metadata> for KlyraMetadata {
    fn new() -> Self {
        Self
    }

    const TYPE: Type = Type::Metadata;

    type Config = ();

    type Output = Metadata;

    fn config(&self) -> &Self::Config {
        &()
    }

    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, Error> {
        Ok(factory.get_metadata())
    }

    async fn build(build_data: &Self::Output) -> Result<Metadata, Error> {
        Ok(build_data.clone())
    }
}
