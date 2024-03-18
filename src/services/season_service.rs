use crate::arkalis_service::{
    AddSeasonRequest, AddSeasonResponse, GetAnimeSeasonsRequest, GetAnimeSeasonsResponse,
    GetLastSeasonSequenceRequest, GetLastSeasonSequenceResponse,
};
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

    pub async fn get_last_season_sequence(
        &self,
        filter: GetLastSeasonSequenceRequest,
    ) -> Result<GetLastSeasonSequenceResponse, ApplicationError> {
        let last_sequence =
            season_repository::season_get_last_sequence(&self.database_connection, filter.anime_id)
                .await;

        if let Err(ApplicationError::NotFound) = last_sequence {
            return Ok(GetLastSeasonSequenceResponse { last_sequence: 0 });
        }

        Ok(GetLastSeasonSequenceResponse {
            last_sequence: last_sequence? as u32,
        })
    }

    pub async fn get_by_anime(
        &self,
        anime: GetAnimeSeasonsRequest,
    ) -> Result<GetAnimeSeasonsResponse, ApplicationError> {
        let seasons =
            season_repository::season_get_by_anime(&self.database_connection, anime.anime_id)
                .await?;
        Ok(GetAnimeSeasonsResponse {
            seasons: seasons.into_iter().map(|s| s.into()).collect(),
        })
    }
}
