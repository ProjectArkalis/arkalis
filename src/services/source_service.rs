use crate::arkalis_service::{
    CreateSourceRequest, CreateSourceResponse, GetSourcesRequest, GetSourcesResponse, Sources,
};
use crate::models::error::ApplicationError;
use crate::models::source::Source;
use crate::models::user::User;
use crate::repositories::{source_repository, DatabaseConnection};
use std::sync::Arc;
use validator::Validate;

pub struct SourceService {
    pub database_connection: Arc<DatabaseConnection>,
}

impl SourceService {
    pub async fn add_source(
        &self,
        data: CreateSourceRequest,
        user: &User,
    ) -> Result<CreateSourceResponse, ApplicationError> {
        let source = Source::new(data, user)?;
        source.validate()?;
        let id = source_repository::source_add(&self.database_connection, source).await?;
        Ok(CreateSourceResponse { id })
    }

    pub async fn get_sources(
        &self,
        filters: GetSourcesRequest,
    ) -> Result<GetSourcesResponse, ApplicationError> {
        let sources = source_repository::source_get(&self.database_connection, filters).await?;
        let items = sources.into_iter().map(Sources::from).collect();
        Ok(GetSourcesResponse { sources: items })
    }
}
