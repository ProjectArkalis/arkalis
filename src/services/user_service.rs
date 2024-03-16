use std::sync::Arc;
use validator::Validate;

use crate::arkalis_service::{
    CreateAdminRequest, CreateAdminResponse, CreateTokenRequest, CreateTokenResponse,
};
use crate::models::config::Config;
use crate::models::error::ApplicationError;
use crate::models::user::User;
use crate::repositories::{user_repository, DatabaseConnection};

pub struct UserService {
    pub config: Arc<Config>,
    pub database_connection: Arc<DatabaseConnection>,
}

impl UserService {
    pub async fn generate_token(
        &self,
        data: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, ApplicationError> {
        let user = User::from(data);
        user.validate()?;
        user_repository::user_add(&self.database_connection, user.clone()).await?;
        let token = user.generate_token(&self.config)?;
        Ok(CreateTokenResponse { token })
    }

    pub async fn create_adm_token(
        &self,
        data: CreateAdminRequest,
    ) -> Result<CreateAdminResponse, ApplicationError> {
        let user = User::create_adm_user(&self.config, data.display_name, &data.admin_master_key)?;
        user.validate()?;
        user_repository::user_add(&self.database_connection, user.clone()).await?;
        let token = user.generate_token(&self.config)?;

        Ok(CreateAdminResponse { token })
    }
}
