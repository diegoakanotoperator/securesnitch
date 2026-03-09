use tonic::transport::Server;
use crate::rules_engine::protocol::ui_server::UiServer;
use crate::service::SecureSnitchService;
use log::info;

pub async fn start_grpc_server(addr: &str) -> anyhow::Result<()> {
    let service = SecureSnitchService::default();
    let socket_addr = addr.parse()?;

    info!("Starting SecureSnitch gRPC server at {}...", addr);

    Server::builder()
        .add_service(UiServer::new(service))
        .serve(socket_addr)
        .await?;

    Ok(())
}
