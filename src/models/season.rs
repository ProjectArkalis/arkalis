use crate::arkalis_service::AddSeasonRequest;
use crate::models::error::ApplicationError;
use crate::models::roles::Roles;
use crate::models::user::User;
use validator::Validate;

#[derive(Validate)]
pub struct Season {
    pub id: Option<u32>,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub cover_id: Option<String>,
    pub anime_id: u32,
    pub sequence: u16,
}

impl Season {
    pub fn new(season_request: AddSeasonRequest, user: &User) -> Result<Self, ApplicationError> {
        if user.role != Roles::Admin {
            return Err(ApplicationError::Unauthorized);
        }

        let season = Self {
            anime_id: season_request.anime_id,
            cover_id: season_request.cover_id,
            sequence: season_request.sequence as u16,
            name: season_request.name,
            id: None,
        };

        Ok(season)
    }
}
