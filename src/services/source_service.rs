use crate::arkalis_service::{
    CreateSourceRequest, CreateSourceResponse, EditSourceRequest, EditSourceResponse,
    GetSourceByIdRequest, GetSourceByIdResponse, GetSourcesBySeasonIdRequest,
    GetSourcesBySeasonIdResponse, GetSourcesRequest, GetSourcesResponse, Sources,
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

    pub async fn update_source(
        &self,
        edit_data: EditSourceRequest,
        user: &User,
    ) -> Result<EditSourceResponse, ApplicationError> {
        let source =
            source_repository::source_by_id(&self.database_connection, edit_data.id).await?;
        let source = source.edit(edit_data, user)?;
        source.validate()?;
        source_repository::source_update(&self.database_connection, source).await?;
        Ok(EditSourceResponse {})
    }

    pub async fn get_source_by_id(
        &self,
        id: GetSourceByIdRequest,
    ) -> Result<GetSourceByIdResponse, ApplicationError> {
        let source = source_repository::source_by_id(&self.database_connection, id.id).await?;
        Ok(GetSourceByIdResponse {
            source: Some(source.into()),
        })
    }

    pub async fn get_source_by_season_id(
        &self,
        filter: GetSourcesBySeasonIdRequest,
    ) -> Result<GetSourcesBySeasonIdResponse, ApplicationError> {
        let sources =
            source_repository::sources_by_season_id(&self.database_connection, filter.season_id)
                .await?;

        Ok(GetSourcesBySeasonIdResponse {
            sources: sources.into_iter().map(Sources::from).collect(),
        })
    }
}
