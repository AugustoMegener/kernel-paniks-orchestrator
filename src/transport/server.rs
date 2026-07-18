use tokio::sync::oneshot;
use tonic::{Request, Response, Status};

use crate::greeter::{orchestrator_server::Orchestrator, ResultResponse};

pub struct OrchestratorService {
    shutdown_tx: std::sync::Mutex<Option<oneshot::Sender<()>>>,
}

impl OrchestratorService {
    pub fn new(shutdown_tx: oneshot::Sender<()>) -> Self {
        Self {
            shutdown_tx: std::sync::Mutex::new(Some(shutdown_tx)),
        }
    }
}

#[tonic::async_trait]
impl Orchestrator for OrchestratorService {
    async fn shutdown_daemon(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ResultResponse>, Status> {
        if let Some(tx) = self.shutdown_tx.lock().unwrap().take() {
            let _ = tx.send(());
        }

        Ok(Response::new(ResultResponse { is_sucess: true, message: None }))
    }

    async fn ping(&self, _request: Request<()>) -> Result<Response<()>, Status> {

        Ok(Response::new(()))
    }
}
