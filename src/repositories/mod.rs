use crate::models::config::Config;
use sqlx::{MySql, MySqlPool, Pool};
use crate::models::error::ApplicationError;

pub mod anime_repository;
pub mod user_repository;

pub struct DatabaseConnection {
    connection: Pool<MySql>,
}

impl DatabaseConnection {
    pub async fn new(config: &Config) -> Self {
        let conn = MySqlPool::connect(config.database_url.as_str())
            .await
            .expect("Failed to connect to database");
        Self { connection: conn }
    }
    
    pub async fn migrate_database(&self) -> Result<(), ApplicationError> {
        sqlx::migrate!()
            .run(&self.connection)
            .await
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        
        Ok(())
    }
}
