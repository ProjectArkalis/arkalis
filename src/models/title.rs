use crate::arkalis_service;
use crate::models::error::ApplicationError;
use crate::models::title_type::TitleType;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct Title {
    #[validate(length(min = 1, max = 1024))]
    pub name: String,
    pub title_type: TitleType,
    pub is_main: bool,
}

impl Title {
    pub fn from_grpc(value: arkalis_service::Title) -> Result<Self, ApplicationError> {
        let title_type = TitleType::from_i32(value.title_type).ok_or(
            ApplicationError::InvalidData(anyhow::Error::msg("title type is invalid")),
        )?;
        let title = Self {
            name: value.name,
            is_main: value.is_main,
            title_type,
        };

        title.validate()?;

        Ok(title)
    }

    pub fn from_grpc_arr(arr: Vec<arkalis_service::Title>) -> Result<Vec<Self>, ApplicationError> {
        let mut converted_titles = Vec::new();
        for title in arr {
            converted_titles.push(Self::from_grpc(title)?);
        }
        Ok(converted_titles)
    }
}

impl From<Title> for arkalis_service::Title {
    fn from(value: Title) -> Self {
        Self {
            name: value.name,
            is_main: value.is_main,
            title_type: value.title_type.to_i32().unwrap_or(0),
        }
    }
}
