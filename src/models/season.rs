use crate::arkalis_service;
use crate::arkalis_service::{AddSeasonRequest, EditSeasonRequest};
use crate::extensions::OptionToAppResult;
use crate::models::error::ApplicationError;
use crate::models::roles::Roles;
use crate::models::user::User;
use sqlx::FromRow;
use validator::Validate;

#[derive(Validate, FromRow)]
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

    pub fn edit(
        mut self,
        new_data: EditSeasonRequest,
        user: &User,
    ) -> Result<Self, ApplicationError> {
        if user.role != Roles::Admin {
            return Err(ApplicationError::Unauthorized);
        }

        if new_data.id != self.id.ok_or_app_result("entity id is null")? {
            return Err(ApplicationError::UnknownError(anyhow::Error::msg(
                "entity id does not match the request",
            )));
        }

        self.name = new_data.name;
        self.cover_id = new_data.cover_id;
        self.sequence = new_data.sequence as u16;

        Ok(self)
    }
}

impl From<Season> for arkalis_service::Season {
    fn from(value: Season) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            name: value.name,
            cover_id: value.cover_id,
            anime_id: value.anime_id,
            sequence: value.sequence as u32,
        }
    }
}
