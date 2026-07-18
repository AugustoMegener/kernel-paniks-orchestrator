use tokio::sync::oneshot;
use tracing::info;
use tonic::transport::Server;
use transport::server::OrchestratorService;
use greeter::orchestrator_server::OrchestratorServer;

mod transport;

pub mod greeter {
    tonic::include_proto!("main");
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

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
