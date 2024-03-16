use clap::Parser;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::arkalis_service::arkalis_core_service_server::ArkalisCoreService;
use crate::arkalis_service::{
    CreateAdminRequest, CreateAdminResponse, CreateAnimeRequest, CreateAnimeResponse,
    CreateTokenRequest, CreateTokenResponse, GetAnimeByIdRequest, GetAnimeByIdResponse,
    GetUserInfoRequest, GetUserInfoResponse, SearchAnimeRequest, SearchAnimeResponse,
};
use crate::authentication::Authentication;
use crate::models::arguments::Cli;
use crate::models::config::Config;
use crate::repositories::DatabaseConnection;
use crate::services::anime_service::AnimeService;
use crate::services::user_service::UserService;

pub struct ArkalisGrpcServerServices {
    user_service: UserService,
    config: Arc<Config>,
    anime_service: AnimeService,
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
                database_connection,
            },
        }
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
}
