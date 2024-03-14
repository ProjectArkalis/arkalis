use std::sync::Arc;

use crate::arkalis_service::{
    CreateAnimeRequest, CreateAnimeResponse, GetAnimeByIdRequest, GetAnimeByIdResponse,
};
use crate::models::anime::Anime;
use crate::models::error::ApplicationError;
use crate::models::user::User;
use crate::repositories::anime_repository;
use crate::repositories::DatabaseConnection;

pub struct AnimeService {
    pub database_connection: Arc<DatabaseConnection>,
}

impl AnimeService {
    pub async fn add_anime(
        &self,
        data: CreateAnimeRequest,
        user: &User,
    ) -> Result<CreateAnimeResponse, ApplicationError> {
        let anime = Anime::new(data, user)?;
        let id = anime_repository::anime_add(&self.database_connection, anime).await?;
        Ok(CreateAnimeResponse { id })
    }

    pub async fn get_anime(
        &self,
        data: GetAnimeByIdRequest,
    ) -> Result<GetAnimeByIdResponse, ApplicationError> {
        let id = data.id;
        let anime = anime_repository::anime_get_by_id(&self.database_connection, id)
            .await?
            .into();
        Ok(GetAnimeByIdResponse { anime: Some(anime) })
    }
}
