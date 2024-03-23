use validator::Validate;

use crate::arkalis_service::CreateSourceRequest;
use crate::models::error::ApplicationError;
use crate::models::roles::Roles;
use crate::models::source_type::SourceType;
use crate::models::user::User;

#[derive(Validate)]
pub struct Source {
    pub id: Option<u32>,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub source_type: SourceType,
    pub priority: u8,
}

impl Source {
    pub fn new(data: CreateSourceRequest, user: &User) -> Result<Self, ApplicationError> {
        if user.role != Roles::Admin {
            return Err(ApplicationError::Unauthorized);
        }

        let source_type =
            SourceType::from_bits(data.source_type).ok_or(ApplicationError::InvalidData(
                anyhow::Error::msg("source_type is not a valid source type"),
            ))?;

        let source = Self {
            id: None,
            name: data.name,
            source_type,
            priority: data.priority as u8,
        };

        Ok(source)
    }
}
