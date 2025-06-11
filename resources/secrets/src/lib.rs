#![doc = include_str!("../README.md")]
use async_trait::async_trait;
pub use klyra_service::SecretStore;
use klyra_service::{resource::Type, Error, Factory, ResourceBuilder};

/// Secrets plugin that provides service secrets
#[derive(Default)]
pub struct Secrets;

#[async_trait]
impl ResourceBuilder for Secrets {
    const TYPE: Type = Type::Secrets;
    type Config = ();
    type Output = SecretStore;

    fn config(&self) -> &Self::Config {
        &()
    }

    async fn output(self, factory: &mut dyn Factory) -> Result<Self::Output, crate::Error> {
        let secrets = factory.get_secrets().await?;

        Ok(SecretStore::new(secrets))
    }
}
