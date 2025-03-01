use async_trait::async_trait;
use klyra_service::Factory;

use crate::database;

pub(crate) struct KlyraFactory<'a> {
    database: &'a mut database::State,
}

impl<'a> KlyraFactory<'a> {
    pub(crate) fn new(database: &'a mut database::State) -> Self {
        Self { database }
    }
}

#[async_trait]
impl Factory for KlyraFactory<'_> {
    async fn get_sql_connection_string(&mut self) -> Result<String, klyra_service::Error> {
        let conn_str = self.database.request().connection_string("localhost");
        debug!("giving a sql connection string: {}", conn_str);
        Ok(conn_str)
    }
}
