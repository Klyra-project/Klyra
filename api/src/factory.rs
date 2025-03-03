use async_trait::async_trait;
use klyra_common::DatabaseReadyInfo;
use klyra_service::Factory;

use crate::database;

pub(crate) struct KlyraFactory {
    database: database::State,
}

impl KlyraFactory {
    pub(crate) fn new(database: database::State) -> Self {
        Self { database }
    }
}

impl KlyraFactory {
    pub(crate) fn to_database_info(&self) -> Option<DatabaseReadyInfo> {
        self.database.to_info()
    }
}

#[async_trait]
impl Factory for KlyraFactory {
    async fn get_sql_connection_string(&mut self) -> Result<String, klyra_service::Error> {
        let conn_str = self
            .database
            .request()
            .await
            .map_err(klyra_service::error::CustomError::new)?
            .connection_string("localhost");
        debug!("giving a sql connection string: {}", conn_str);
        Ok(conn_str)
    }
}
