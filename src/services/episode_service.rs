use std::sync::Arc;

use validator::Validate;

use crate::arkalis_service::{
    CreateEpisodeRequest, CreateEpisodeResponse, GetEpisodeByIdRequest, GetEpisodeByIdResponse,
    GetEpisodesBySeasonAndSourceRequest, GetEpisodesBySeasonAndSourceResponse,
    UpdateEpisodeRequest, UpdateEpisodeResponse,
};
use crate::models::episode::Episode;
use crate::models::error::ApplicationError;
use crate::models::user::User;
use crate::repositories::{episode_repository, DatabaseConnection};

pub struct EpisodeService {
    pub database_connection: Arc<DatabaseConnection>,
}

impl EpisodeService {
    pub async fn add_episode(
        &self,
        ep: CreateEpisodeRequest,
        user: &User,
    ) -> Result<CreateEpisodeResponse, ApplicationError> {
        let episode = Episode::new(ep, user)?;
        episode.validate()?;

        let id = episode.id.clone();
        let name = episode.name.clone();

        episode_repository::episode_add(&self.database_connection, episode).await?;

        Ok(CreateEpisodeResponse { id, name })
    }

    pub async fn update_episode(
        &self,
        data: UpdateEpisodeRequest,
        user: &User,
    ) -> Result<UpdateEpisodeResponse, ApplicationError> {
        let episode =
            episode_repository::episode_get_by_id(&self.database_connection, &data.id).await?;
        let episode = episode.update_episode(data, user).await?;
        episode_repository::episode_update(&self.database_connection, episode).await?;

        Ok(UpdateEpisodeResponse {})
    }

    pub async fn get_episodes_by_season_and_source(
        &self,
        filter: GetEpisodesBySeasonAndSourceRequest,
    ) -> Result<GetEpisodesBySeasonAndSourceResponse, ApplicationError> {
        let episodes = episode_repository::episode_get_by_season_and_source(
            &self.database_connection,
            filter.season_id,
            filter.source_id,
        )
        .await?;
        let episodes_grpc = Episode::parse_to_grpc_vec_model(episodes)?;
        Ok(GetEpisodesBySeasonAndSourceResponse {
            episodes: episodes_grpc,
        })
    }

    pub async fn get_episode_bu_id(
        &self,
        filter: GetEpisodeByIdRequest,
    ) -> Result<GetEpisodeByIdResponse, ApplicationError> {
        let episode =
            episode_repository::episode_get_by_id(&self.database_connection, &filter.id).await?;
        let episode_grpc = Episode::parse_to_grpc_model(episode)?;
        Ok(GetEpisodeByIdResponse {
            episode: Some(episode_grpc),
        })
    }
}
