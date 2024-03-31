use anyhow::anyhow;
use khash::Digest;
use sqlx::FromRow;
use validator::Validate;

use crate::arkalis_service::{CreateEpisodeRequest, UpdateEpisodeRequest};
use crate::models::error::ApplicationError;
use crate::models::user::User;
use crate::{arkalis_service, view_models};

#[derive(Validate, FromRow)]
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

    pub async fn update_episode(
        mut self,
        new_data: UpdateEpisodeRequest,
        user: &User,
    ) -> Result<Self, ApplicationError> {
        if !user.has_uploader_or_adm_role() {
            return Err(ApplicationError::Unauthorized);
        }

        if new_data.id != self.id {
            return Err(ApplicationError::UnknownError(anyhow::Error::msg(
                "entity id does not match the request",
            )));
        }

        if self.lbry_media_id.is_none() && new_data.lbry_url.is_none() {
            return Err(ApplicationError::InvalidData(anyhow!(
                "LBRY URL is required"
            )));
        }

        if let Some(media_url) = &new_data.lbry_url {
            let media_id = Self::get_lbry_media_id(media_url)?;
            let media_id_some = Some(media_id.clone());
            if media_id_some != self.lbry_media_id {
                self.lbry_media_id = media_id_some;
                self.file_name = Some(Self::get_file_name(&media_id).await?)
            }
        }

        self.cover_id = new_data.cover_id;
        self.sequence = new_data.sequence as u16;

        Ok(self)
    }

    pub fn parse_to_grpc_model(self) -> Result<arkalis_service::Episode, ApplicationError> {
        let ep =
            arkalis_service::Episode {
                id: self.id,
                name: self.name,
                cover_id: self.cover_id,
                season_id: self.season_id,
                source_id: self.source_id,
                lbry_media_id: self.lbry_media_id.ok_or(ApplicationError::UnknownError(
                    anyhow!("lbry_media_id is null"),
                ))?,
                file_name: self
                    .file_name
                    .ok_or(ApplicationError::UnknownError(anyhow!("file_name is null")))?,
                is_nsfw: self.is_nsfw,
                sequence: self.sequence as u32,
            };

        Ok(ep)
    }

    pub fn parse_to_grpc_vec_model(
        episodes: Vec<Episode>,
    ) -> Result<Vec<arkalis_service::Episode>, ApplicationError> {
        let mut res_eps = Vec::with_capacity(episodes.len());
        for ep in episodes {
            res_eps.push(ep.parse_to_grpc_model()?);
        }

        Ok(res_eps)
    }

    fn get_lbry_media_id(url: &str) -> Result<String, ApplicationError> {
        const LBRY_BASE_URL: &str = "https://open.lbry.com/";
        if !url.starts_with(LBRY_BASE_URL) {
            return Err(ApplicationError::InvalidData(anyhow::Error::msg(
                "Url do lbry incorreta",
            )));
        }

        let media_id = url.replace(LBRY_BASE_URL, "");
        Ok(media_id)
    }

    async fn get_file_name(media_id: &str) -> Result<String, ApplicationError> {
        const ODYSEE_BASE_URL: &str = "https://odysee.com/";
        let odysee_response = reqwest::get(format!("{}{}", ODYSEE_BASE_URL, media_id))
            .await
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        let odysee_response_body = odysee_response
            .text()
            .await
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        let regex = regex::Regex::new(r#"(?s)<script type="application/ld\+json">(.*?)</script>"#)
            .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        let caps =
            regex
                .captures(odysee_response_body.as_str())
                .ok_or(ApplicationError::UnknownError(anyhow!(
                    "Failed to parse html for data"
                )))?;
        let json =
            serde_json::from_str::<view_models::odysee::Media>(caps.get(1).unwrap().as_str())
                .map_err(|e| ApplicationError::UnknownError(e.into()))?;
        json.content_url
            .ok_or(ApplicationError::UnknownError(anyhow!(
                "Failed to get file_name from odysee response"
            )))
    }
}
