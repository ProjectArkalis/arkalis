use std::sync::Arc;
use validator::Validate;

use crate::arkalis_service::{
    CreateAdminRequest, CreateAdminResponse, CreateRecoveryKeyResponse, CreateTokenRequest,
    CreateTokenResponse, RecoveryUserRequest, RecoveryUserResponse,
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

    pub async fn get_recovery_key(
        &self,
        mut user: User,
    ) -> Result<CreateRecoveryKeyResponse, ApplicationError> {
        let mnemonic = user.get_recovery_mnemonic();
        user_repository::user_update_recovery_key(&self.database_connection, user).await?;
        Ok(CreateRecoveryKeyResponse {
            recovery_key: mnemonic,
        })
    }

    pub async fn recovery_user(
        &self,
        recovery_data: RecoveryUserRequest,
    ) -> Result<RecoveryUserResponse, ApplicationError> {
        let user = user_repository::user_get_by_recovery_key(
            &self.database_connection,
            recovery_data.recovery_key,
        )
        .await?;
        let token = user.generate_token(&self.config)?;
        Ok(RecoveryUserResponse { token })
    }
}
