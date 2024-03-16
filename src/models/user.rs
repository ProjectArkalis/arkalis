use std::collections::BTreeMap;

use hmac::digest::core_api::CoreWrapper;
use hmac::{Hmac, HmacCore};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::digest::Mac;
use sha2::Sha256;
use uuid::Uuid;
use validator::Validate;

use crate::arkalis_service::{CreateTokenRequest, GetUserInfoResponse};
use crate::models::config::Config;
use crate::models::error::ApplicationError;
use crate::models::roles::Roles;

#[derive(Validate, Clone)]
pub struct User {
    pub id: String,
    #[validate(length(min = 4))]
    pub display_name: String,
    pub role: Roles,
    pub mal_profile: Option<String>,
    pub anilist_profile: Option<String>,
}

impl User {
    pub fn new(display_name: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id,
            display_name,
            role: Roles::User,
            mal_profile: None,
            anilist_profile: None,
        }
    }

    pub fn generate_token(self, config: &Config) -> Result<String, ApplicationError> {
        let key = Self::get_jwt_key(config)?;
        let mut claims = BTreeMap::new();
        claims.insert("id", self.id);
        claims.insert("display_name", self.display_name);
        claims.insert("role", self.role.into());

        if let Some(mal_profile) = self.mal_profile {
            claims.insert("mal_profile", mal_profile);
        }

        if let Some(anilist_profile) = self.anilist_profile {
            claims.insert("anilist_profile", anilist_profile);
        }

        claims.sign_with_key(&key).map_err(|_| {
            ApplicationError::UnknownError(anyhow::Error::msg("Could not generate JWT"))
        })
    }

    pub fn from_token(token: String, config: &Config) -> Result<Self, ApplicationError> {
        let key = Self::get_jwt_key(config)?;

        let token: Token<Header, BTreeMap<String, String>, _> = token
            .verify_with_key(&key)
            .map_err(|_| ApplicationError::Unauthorized)?;
        let claims = token.claims();

        let mal_profile = claims.get("mal_profile").map(|x| x.clone());
        let anilist_profile = claims.get("anilist_profile").map(|x| x.clone());

        let user = Self {
            id: claims["id"].clone(),
            display_name: claims["display_name"].clone(),
            role: Roles::from(claims["role"].as_str()),
            mal_profile,
            anilist_profile,
        };

        Ok(user)
    }

    pub fn create_adm_user(
        config: &Config,
        display_name: String,
        admin_master_key: &str,
    ) -> Result<Self, ApplicationError> {
        if config.admin_master_key != admin_master_key {
            return Err(ApplicationError::Unauthorized);
        }
        let mut user = User::new(display_name);
        user.role = Roles::Admin;

        Ok(user)
    }

    fn get_jwt_key(config: &Config) -> Result<CoreWrapper<HmacCore<Sha256>>, ApplicationError> {
        let jwt = Hmac::<Sha256>::new_from_slice(config.jwt_secret.as_bytes()).map_err(|_| {
            ApplicationError::UnknownError(anyhow::Error::msg("Could not generate HMAC"))
        })?;

        Ok(jwt)
    }
}

impl From<CreateTokenRequest> for User {
    fn from(request: CreateTokenRequest) -> Self {
        Self::new(request.display_name)
    }
}

impl From<User> for GetUserInfoResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            role: value.role.into(),
            mal_profile: value.mal_profile,
            anilist_profile: value.anilist_profile,
        }
    }
}
