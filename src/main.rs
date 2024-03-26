use models::arguments::Cli;
use tonic::transport::Server;
use clap::Parser;

use crate::arkalis_service::arkalis_core_service_server::ArkalisCoreServiceServer;
use crate::grpc_calls::ArkalisGrpcServerServices;
use crate::models::config::Config;

mod extensions;
mod grpc_calls;
mod models;
mod repositories;
mod services;

pub mod arkalis_service {
    tonic::include_proto!("arkalis");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    
    let args = Cli::parse();
    let config = Config::new(&args);

    let service = ArkalisGrpcServerServices::new(config.clone()).await;
    let addr = config.bind_url.unwrap_or("127.0.0.1:8000".into());

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
