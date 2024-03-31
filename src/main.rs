use clap::Parser;
use models::arguments::Cli;
use tonic::transport::Server;

use crate::arkalis_service::arkalis_core_service_server::ArkalisCoreServiceServer;
use crate::grpc_calls::ArkalisGrpcServerServices;
use crate::models::config::Config;

mod extensions;
mod grpc_calls;
mod models;
mod repositories;
mod services;
mod view_models;

pub mod arkalis_service {
    tonic::include_proto!("arkalis");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let args = Cli::parse();
    let config = Config::new(&args);
    let addr = config.bind_url.clone().unwrap_or("127.0.0.1:8000".into());

    let service = ArkalisGrpcServerServices::new(config).await;

    service
        .startup_routine()
        .await
        .expect("Failed to migrate database");

    Server::builder()
        .add_service(ArkalisCoreServiceServer::new(service))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
