use thiserror::Error;
use tonic::{Code, Status};
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Unknown error")]
    UnknownError(anyhow::Error),
    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),
    #[error("Authentication failed")]
    Unauthorized,
    #[error("Data provided is invalid")]
    InvalidData(anyhow::Error),
    #[error("Entity not found")]
    NotFound,
}

impl From<ApplicationError> for Status {
    fn from(value: ApplicationError) -> Self {
        match value {
            ApplicationError::UnknownError(err) => Status::new(Code::Internal, err.to_string()),
            ApplicationError::ValidationError(err) => {
                Status::new(Code::InvalidArgument, err.to_string())
            }
            ApplicationError::Unauthorized => Status::new(Code::Unauthenticated, value.to_string()),
            ApplicationError::InvalidData(err) => {
                Status::new(Code::InvalidArgument, err.to_string())
            }
            ApplicationError::NotFound => Status::new(Code::NotFound, value.to_string()),
        }
    }
}
