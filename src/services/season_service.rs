use crate::arkalis_service::{AddSeasonRequest, AddSeasonResponse};
use crate::models::error::ApplicationError;
use crate::models::season::Season;
use crate::models::user::User;
use crate::repositories::season_repository;
use crate::repositories::DatabaseConnection;
use std::sync::Arc;

pub struct SeasonService {
    pub database_connection: Arc<DatabaseConnection>,
}

impl SeasonService {
    pub async fn add_season(
        &self,
        data: AddSeasonRequest,
        user: &User,
    ) -> Result<AddSeasonResponse, ApplicationError> {
        let season = Season::new(data, user)?;
        let id = season_repository::season_add(&self.database_connection, season).await?;
        Ok(AddSeasonResponse { id })
    }
}
