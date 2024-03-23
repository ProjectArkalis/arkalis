use crate::arkalis_service;
use sqlx::mysql::MySqlRow;
use sqlx::{Error, FromRow, Row};
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

impl FromRow<'_, MySqlRow> for Source {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, Error> {
        let source_type =
            SourceType::from_bits(row.try_get("source_type")?).ok_or(Error::TypeNotFound {
                type_name: "SourceType".into(),
            })?;

        let source = Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            source_type,
            priority: row.try_get("priority")?,
        };

        Ok(source)
    }
}

impl From<Source> for arkalis_service::Sources {
    fn from(value: Source) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            name: value.name,
            source_type: value.source_type.bits(),
            priority: value.priority as u32,
        }
    }
}
