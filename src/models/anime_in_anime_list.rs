use crate::arkalis_service;
use crate::models::anime_list::AnimeList;
use crate::models::error::ApplicationError;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct AnimeInAnimeList {
    anime_list: AnimeList,
    #[validate(length(min = 1, max = 100))]
    id_in_list: String,
}

impl AnimeInAnimeList {
    pub fn from_grpc(value: arkalis_service::AnimeInAnimeList) -> Result<Self, ApplicationError> {
        let anime_list = AnimeList::from_i32(value.anime_list).ok_or(
            ApplicationError::InvalidData(anyhow::Error::msg("anime_list type is invalid")),
        )?;

        let anime_in_anime_list = Self {
            anime_list,
            id_in_list: value.id_in_list,
        };

        anime_in_anime_list.validate()?;

        Ok(anime_in_anime_list)
    }

    pub fn from_grpc_arr(
        arr: Vec<arkalis_service::AnimeInAnimeList>,
    ) -> Result<Vec<Self>, ApplicationError> {
        let mut converted_anime_list = Vec::new();
        for anime in arr {
            converted_anime_list.push(Self::from_grpc(anime)?);
        }
        Ok(converted_anime_list)
    }
}

impl From<AnimeInAnimeList> for arkalis_service::AnimeInAnimeList {
    fn from(value: AnimeInAnimeList) -> Self {
        Self {
            anime_list: value.anime_list.to_i32().unwrap_or(0),
            id_in_list: value.id_in_list,
        }
    }
}
