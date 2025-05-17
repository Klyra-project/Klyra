use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use klyra_service::{error::Error, Factory, ResourceBuilder, Type};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The Klyra service name.
    service_name: String,
}

impl Metadata {
    /// Get the Klyra service name.
    pub fn service_name(&self) -> &str {
        &self.service_name
    }
}

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
        Ok(Metadata {
            service_name: factory.get_service_name().to_string(),
        })
    }

    async fn build(build_data: &Self::Output) -> Result<Metadata, Error> {
        Ok(build_data.clone())
    }
}
