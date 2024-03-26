use crate::arkalis_service::CreateEpisodeRequest;
use crate::models::error::ApplicationError;
use crate::models::user::User;
use khash::Digest;
use validator::Validate;

#[derive(Validate)]
pub struct Episode {
    pub id: String,
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub cover_id: Option<String>,
    pub season_id: u32,
    pub source_id: u32,
    #[validate(length(min = 1, max = 255))]
    pub lbry_media_id: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub file_name: Option<String>,
    pub is_nsfw: bool,
    pub sequence: u16,
}

impl Episode {
    pub fn new(
        episode_request: CreateEpisodeRequest,
        user: &User,
    ) -> Result<Self, ApplicationError> {
        if !user.has_uploader_or_adm_role() {
            return Err(ApplicationError::Unauthorized);
        }

        let id = uuid::Uuid::new_v4().to_string().replace('-', "");
        let name = Digest::new(&mut id.as_bytes()).collect::<String>();

        let episode = Self {
            id,
            name,
            cover_id: episode_request.cover_id,
            season_id: episode_request.season_id,
            source_id: episode_request.source_id,
            lbry_media_id: None,
            file_name: None,
            is_nsfw: episode_request.is_nsfw,
            sequence: episode_request.sequence as u16,
        };

        Ok(episode)
    }
}
