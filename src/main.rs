use tonic::transport::Server;

use crate::arkalis_service::arkalis_core_service_server::ArkalisCoreServiceServer;
use crate::grpc_calls::ArkalisGrpcServerServices;

mod authentication;
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
    let addr = "127.0.0.1:8000".parse()?;
    let service = ArkalisGrpcServerServices::new().await;
    service.startup_routine().await.expect("Failed to migrate database");

    Server::builder()
        .add_service(ArkalisCoreServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
