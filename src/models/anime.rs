use crate::arkalis_service;
use arkalis_commons::enums::genres::Genre;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, FromRow, Row};
use validator::{Validate, ValidationError};

use crate::arkalis_service::{CreateAnimeRequest, EditAnimeRequest};
use crate::models::anime_in_anime_list::AnimeInAnimeList;
use crate::models::error::ApplicationError;
use crate::models::roles::Roles;
use crate::models::title::Title;
use crate::models::user::User;

#[derive(Validate)]
pub struct Anime {
    pub id: Option<u32>,
    #[validate(custom(function = "validate_titles"))]
    pub titles: Vec<Title>,
    pub title_search: String,
    #[validate(length(min = 1, max = 4000))]
    pub synopsis: String,
    #[validate(length(min = 1, max = 255))]
    pub thumbnail_id: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub banner_id: Option<String>,
    pub is_hidden: bool,
    pub is_nsfw: bool,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub genre: Genre,
    pub release_date: DateTime<Utc>,
    #[validate(length(min = 1))]
    pub anime_in_lists: Vec<AnimeInAnimeList>,
}

impl Anime {
    pub fn new(data: CreateAnimeRequest, created_by: &User) -> Result<Self, ApplicationError> {
        if created_by.role != Roles::Admin {
            return Err(ApplicationError::Unauthorized);
        }

        let titles = Title::from_grpc_arr(data.titles)?;
        let title_search = generate_title_search(&titles);
        let release_date =
            DateTime::from_timestamp(data.release_date, 0).ok_or(ApplicationError::InvalidData(
                anyhow::Error::msg("release_date is not a valid unixtime"),
            ))?;
        let anime_in_lists = AnimeInAnimeList::from_grpc_arr(data.anime_in_lists)?;
        let genre = Genre::from_bits(data.genre).ok_or(ApplicationError::InvalidData(
            anyhow::Error::msg("genre is not a valid genre flag"),
        ))?;

        let anime = Self {
            id: None,
            titles,
            title_search,
            synopsis: data.synopsis,
            thumbnail_id: data.thumbnail_id,
            banner_id: data.banner_id,
            is_hidden: data.is_hidden,
            is_nsfw: data.is_nsfw,
            created_by: created_by.id.clone(),
            created_at: Utc::now(),
            release_date,
            genre,
            anime_in_lists,
        };

        anime.validate()?;

        Ok(anime)
    }

    pub fn get_titles_json(&self) -> Result<String, ApplicationError> {
        let json = serde_json::to_string(&self.titles)
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        Ok(json)
    }

    pub fn get_anime_in_anime_list_json(&self) -> Result<String, ApplicationError> {
        let json = serde_json::to_string(&self.anime_in_lists)
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        Ok(json)
    }

    pub fn update(
        mut self,
        update_data: EditAnimeRequest,
        user: &User,
    ) -> Result<Self, ApplicationError> {
        if user.role != Roles::Admin {
            return Err(ApplicationError::Unauthorized);
        }

        if update_data.id
            != self
                .id
                .ok_or(ApplicationError::UnknownError(anyhow::Error::msg(
                    "entity id is null",
                )))?
        {
            return Err(ApplicationError::UnknownError(anyhow::Error::msg(
                "entity id does not match the request",
            )));
        }

        self.titles = Title::from_grpc_arr(update_data.titles)?;
        self.synopsis = update_data.synopsis;
        self.thumbnail_id = update_data.thumbnail_id;
        self.banner_id = update_data.banner_id;
        self.genre = Genre::from_bits(update_data.genre).ok_or(ApplicationError::InvalidData(
            anyhow::Error::msg("genre is invalid"),
        ))?;
        self.release_date = DateTime::from_timestamp(update_data.release_date, 0).ok_or(
            ApplicationError::InvalidData(anyhow::Error::msg(
                "release_date is invalid unix timestamp",
            )),
        )?;
        self.anime_in_lists = AnimeInAnimeList::from_grpc_arr(update_data.anime_in_lists)?;

        Ok(self)
    }

    fn titles_from_json(json: &str) -> Result<Vec<Title>, ApplicationError> {
        let titles = serde_json::from_str::<Vec<Title>>(json)
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        Ok(titles)
    }

    fn anime_in_anime_lists_from_json(
        json: &str,
    ) -> Result<Vec<AnimeInAnimeList>, ApplicationError> {
        let titles = serde_json::from_str::<Vec<AnimeInAnimeList>>(json)
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        Ok(titles)
    }
}

impl FromRow<'_, MySqlRow> for Anime {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, Error> {
        let titles =
            Self::titles_from_json(row.try_get("titles")?).map_err(|_| Error::TypeNotFound {
                type_name: "Vec<Title>".into(),
            })?;

        let genre = Genre::from_bits(row.try_get("genre")?).ok_or(Error::TypeNotFound {
            type_name: "Genre".into(),
        })?;

        let anime_in_lists = Self::anime_in_anime_lists_from_json(row.try_get("anime_in_lists")?)
            .map_err(|_| Error::TypeNotFound {
            type_name: "Vec<AnimeInAnimeList>".into(),
        })?;

        let release_date: NaiveDate = row.try_get("release_date")?;
        let release_date = DateTime::from_naive_utc_and_offset(release_date.into(), Utc);

        let anime = Self {
            id: row.try_get("id")?,
            titles,
            title_search: row.try_get("title_search")?,
            synopsis: row.try_get("synopsis")?,
            thumbnail_id: row.try_get("thumbnail_id")?,
            banner_id: row.try_get("banner_id")?,
            is_hidden: row.try_get("is_hidden")?,
            is_nsfw: row.try_get("is_nsfw")?,
            created_by: row.try_get("created_by")?,
            created_at: row.try_get("created_at")?,
            genre,
            release_date,
            anime_in_lists,
        };

        Ok(anime)
    }
}

impl From<Anime> for arkalis_service::Anime {
    fn from(value: Anime) -> Self {
        Self {
            id: value.id.unwrap_or(0),
            titles: value
                .titles
                .into_iter()
                .map(arkalis_service::Title::from)
                .collect::<Vec<_>>(),
            created_by: value.created_by,
            release_date: value.release_date.timestamp(),
            genre: value.genre.bits(),
            created_at: value.created_at.timestamp(),
            is_nsfw: value.is_nsfw,
            is_hidden: value.is_hidden,
            banner_id: value.banner_id,
            thumbnail_id: value.thumbnail_id,
            synopsis: value.synopsis,
            anime_in_lists: value
                .anime_in_lists
                .into_iter()
                .map(arkalis_service::AnimeInAnimeList::from)
                .collect::<Vec<_>>(),
        }
    }
}

fn generate_title_search(titles: &[Title]) -> String {
    titles
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<_>>()
        .join("_")
}

fn validate_titles(titles: &[Title]) -> Result<(), ValidationError> {
    if titles.is_empty() {
        return Err(ValidationError::new("titles cannot be empty"));
    }

    if !titles.iter().any(|x| x.is_main) {
        return Err(ValidationError::new("at least one title must be main"));
    }

    Ok(())
}
