use tokio::sync::oneshot;
use tracing::info;

mod transport;

pub mod greeter {
    tonic::include_proto!("main");
}

use tonic::transport::Server;
use transport::server::OrchestratorService;
use greeter::orchestrator_server::OrchestratorServer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Orchestrator starting");

    let addr = "[::1]:50051".parse()?;
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let orchestrator = OrchestratorService::new(shutdown_tx);

    Server::builder()
        .add_service(OrchestratorServer::new(orchestrator))
        .serve_with_shutdown(addr, async {
            shutdown_rx.await.ok();
        })
        .await?;

    info!("Orchestrator stopped");
    Ok(())
}
