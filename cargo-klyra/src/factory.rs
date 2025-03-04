use anyhow::anyhow;
use async_trait::async_trait;
use klyra_service::Factory;

pub struct LocalFactory {}

#[async_trait]
impl Factory for LocalFactory {
    async fn get_sql_connection_string(&mut self) -> Result<String, klyra_service::Error> {
        Err(klyra_service::Error::Custom(anyhow!(
            "Database dependencies are not supported for local runs yet. Try deploying instead",
        )))
    }
}
