use std::sync::Arc;
use validator::Validate;

use crate::arkalis_service::{
    CreateAnimeRequest, CreateAnimeResponse, EditAnimeRequest, EditAnimeResponse,
    GetAnimeByIdRequest, GetAnimeByIdResponse, SearchAnimeRequest, SearchAnimeResponse,
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
        user: &Option<User>,
    ) -> Result<GetAnimeByIdResponse, ApplicationError> {
        let id = data.id;
        let show_all = if let Some(user) = user {
            user.has_uploader_or_adm_role()
        } else {
            false
        };
        let anime = anime_repository::anime_get_by_id(&self.database_connection, id, show_all)
            .await?
            .into();
        Ok(GetAnimeByIdResponse { anime: Some(anime) })
    }

    pub async fn search_anime(
        &self,
        filters: SearchAnimeRequest,
        user: &Option<User>,
    ) -> Result<SearchAnimeResponse, ApplicationError> {
        let show_all = if let Some(user) = user {
            user.has_uploader_or_adm_role()
        } else {
            false
        };
        let animes =
            anime_repository::anime_search(&self.database_connection, filters, show_all).await?;
        Ok(SearchAnimeResponse {
            animes: animes.into_iter().map(|anime| anime.into()).collect(),
        })
    }

    pub async fn update_anime(
        &self,
        anime_update: EditAnimeRequest,
        user: &User,
    ) -> Result<EditAnimeResponse, ApplicationError> {
        let show_all = user.has_uploader_or_adm_role();
        let anime =
            anime_repository::anime_get_by_id(&self.database_connection, anime_update.id, show_all)
                .await?;
        let anime = anime.update(anime_update, user)?;
        anime.validate()?;
        anime_repository::anime_update(&self.database_connection, anime).await?;
        Ok(EditAnimeResponse {})
    }
}
