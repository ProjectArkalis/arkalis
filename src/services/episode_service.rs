use crate::arkalis_service::{CreateEpisodeRequest, CreateEpisodeResponse};
use crate::models::episode::Episode;
use crate::models::error::ApplicationError;
use crate::models::user::User;
use crate::repositories::{episode_repository, DatabaseConnection};
use std::sync::Arc;
use validator::Validate;

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
}
