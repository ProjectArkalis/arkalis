use clap::Parser;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::arkalis_service::arkalis_core_service_server::ArkalisCoreService;
use crate::arkalis_service::{
    AddSeasonRequest, AddSeasonResponse, CreateAdminRequest, CreateAdminResponse,
    CreateAnimeRequest, CreateAnimeResponse, CreateRecoveryKeyRequest, CreateRecoveryKeyResponse,
    CreateSourceRequest, CreateSourceResponse, CreateTokenRequest, CreateTokenResponse,
    EditAnimeRequest, EditAnimeResponse, EditSeasonRequest, EditSeasonResponse, EditSourceRequest,
    EditSourceResponse, GetAnimeByIdRequest, GetAnimeByIdResponse, GetAnimeSeasonsRequest,
    GetAnimeSeasonsResponse, GetLastSeasonSequenceRequest, GetLastSeasonSequenceResponse,
    GetSourceByIdRequest, GetSourceByIdResponse, GetSourcesRequest, GetSourcesResponse,
    GetUserInfoRequest, GetUserInfoResponse, RecoveryUserRequest, RecoveryUserResponse,
    SearchAnimeRequest, SearchAnimeResponse,
};
use crate::extensions::Authentication;
use crate::models::arguments::Cli;
use crate::models::config::Config;
use crate::models::error::ApplicationError;
use crate::repositories::DatabaseConnection;
use crate::services::anime_service::AnimeService;
use crate::services::season_service::SeasonService;
use crate::services::source_service::SourceService;
use crate::services::user_service::UserService;

pub struct ArkalisGrpcServerServices {
    user_service: UserService,
    config: Arc<Config>,
    anime_service: AnimeService,
    database_connection: Arc<DatabaseConnection>,
    season_service: SeasonService,
    source_service: SourceService,
}

impl ArkalisGrpcServerServices {
    pub async fn new() -> Self {
        let args = Arc::new(Cli::parse());
        let config = Arc::new(Config::new(&args));
        let database_connection = DatabaseConnection::new(&config).await;
        let database_connection = Arc::new(database_connection);

        ArkalisGrpcServerServices {
            config: config.clone(),
            user_service: UserService {
                config,
                database_connection: database_connection.clone(),
            },
            anime_service: AnimeService {
                database_connection: database_connection.clone(),
            },
            database_connection: database_connection.clone(),
            season_service: SeasonService {
                database_connection: database_connection.clone(),
            },
            source_service: SourceService {
                database_connection,
            },
        }
    }

    pub async fn startup_routine(&self) -> Result<(), ApplicationError> {
        self.database_connection.migrate_database().await
    }
}

#[tonic::async_trait]
impl ArkalisCoreService for ArkalisGrpcServerServices {
    async fn create_token(
        &self,
        request: Request<CreateTokenRequest>,
    ) -> Result<Response<CreateTokenResponse>, Status> {
        let response = self
            .user_service
            .generate_token(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }

    async fn create_admin(
        &self,
        request: Request<CreateAdminRequest>,
    ) -> Result<Response<CreateAdminResponse>, Status> {
        let response = self
            .user_service
            .create_adm_token(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }

    async fn get_user_info(
        &self,
        request: Request<GetUserInfoRequest>,
    ) -> Result<Response<GetUserInfoResponse>, Status> {
        let user = request.get_user(&self.config)?;
        Ok(Response::new(user.into()))
    }

    async fn create_anime(
        &self,
        request: Request<CreateAnimeRequest>,
    ) -> Result<Response<CreateAnimeResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self
            .anime_service
            .add_anime(request.into_inner(), &user)
            .await?;
        Ok(Response::new(response))
    }

    async fn create_recovery_key(
        &self,
        request: Request<CreateRecoveryKeyRequest>,
    ) -> Result<Response<CreateRecoveryKeyResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self.user_service.get_recovery_key(user).await?;
        Ok(Response::new(response))
    }

    async fn recovery_user(
        &self,
        request: Request<RecoveryUserRequest>,
    ) -> Result<Response<RecoveryUserResponse>, Status> {
        let response = self
            .user_service
            .recovery_user(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }

    async fn get_anime_by_id(
        &self,
        request: Request<GetAnimeByIdRequest>,
    ) -> Result<Response<GetAnimeByIdResponse>, Status> {
        let anime = self.anime_service.get_anime(request.into_inner()).await?;
        Ok(Response::new(anime))
    }

    async fn search_anime(
        &self,
        request: Request<SearchAnimeRequest>,
    ) -> Result<Response<SearchAnimeResponse>, Status> {
        let animes = self
            .anime_service
            .search_anime(request.into_inner())
            .await?;
        Ok(Response::new(animes))
    }

    async fn edit_anime(
        &self,
        request: Request<EditAnimeRequest>,
    ) -> Result<Response<EditAnimeResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self
            .anime_service
            .update_anime(request.into_inner(), &user)
            .await?;
        Ok(Response::new(response))
    }

    async fn add_season(
        &self,
        request: Request<AddSeasonRequest>,
    ) -> Result<Response<AddSeasonResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self
            .season_service
            .add_season(request.into_inner(), &user)
            .await?;
        Ok(Response::new(response))
    }

    async fn get_last_season_sequence(
        &self,
        request: Request<GetLastSeasonSequenceRequest>,
    ) -> Result<Response<GetLastSeasonSequenceResponse>, Status> {
        let response = self
            .season_service
            .get_last_season_sequence(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }

    async fn get_anime_seasons(
        &self,
        request: Request<GetAnimeSeasonsRequest>,
    ) -> Result<Response<GetAnimeSeasonsResponse>, Status> {
        let response = self
            .season_service
            .get_by_anime(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }

    async fn edit_season(
        &self,
        request: Request<EditSeasonRequest>,
    ) -> Result<Response<EditSeasonResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self
            .season_service
            .update_season(request.into_inner(), &user)
            .await?;
        Ok(Response::new(response))
    }

    async fn create_source(
        &self,
        request: Request<CreateSourceRequest>,
    ) -> Result<Response<CreateSourceResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self
            .source_service
            .add_source(request.into_inner(), &user)
            .await?;
        Ok(Response::new(response))
    }

    async fn get_sources(
        &self,
        request: Request<GetSourcesRequest>,
    ) -> Result<Response<GetSourcesResponse>, Status> {
        let response = self
            .source_service
            .get_sources(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }

    async fn edit_source(
        &self,
        request: Request<EditSourceRequest>,
    ) -> Result<Response<EditSourceResponse>, Status> {
        let user = request.get_user(&self.config)?;
        let response = self
            .source_service
            .update_source(request.into_inner(), &user)
            .await?;
        Ok(Response::new(response))
    }

    async fn get_source_by_id(
        &self,
        request: Request<GetSourceByIdRequest>,
    ) -> Result<Response<GetSourceByIdResponse>, Status> {
        let response = self
            .source_service
            .get_source_by_id(request.into_inner())
            .await?;
        Ok(Response::new(response))
    }
}
