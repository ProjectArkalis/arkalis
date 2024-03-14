use tonic::Request;

use crate::models::config::Config;
use crate::models::error::ApplicationError;
use crate::models::user::User;

pub trait Authentication {
    fn get_auth_token(&self) -> Option<String>;
    fn get_user(&self, config: &Config) -> Result<User, ApplicationError>;
    fn is_authenticated(&self, config: &Config) -> bool;
}

impl<T> Authentication for Request<T> {
    fn get_auth_token(&self) -> Option<String> {
        let metadata = self.metadata().clone();
        let headers = metadata.into_headers();
        let auth_header = headers
            .get("Authorization")?
            .to_str()
            .ok()?
            .split(' ')
            .nth(1)?
            .to_string();
        Some(auth_header)
    }

    fn get_user(&self, config: &Config) -> Result<User, ApplicationError> {
        let token = self
            .get_auth_token()
            .ok_or(ApplicationError::Unauthorized)?;
        let user = User::from_token(token, config)?;
        Ok(user)
    }

    fn is_authenticated(&self, config: &Config) -> bool {
        let user = self.get_user(config);
        user.is_ok()
    }
}
