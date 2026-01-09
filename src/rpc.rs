use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;

pub mod watchmanx_proto {
    tonic::include_proto!("watchmanx");
}

use watchmanx_proto::watchman_service_server::{WatchmanService, WatchmanServiceServer};
use watchmanx_proto::{Empty, StatusResponse, TriggerRequest, TriggerResponse};

use crate::config::Config;
use crate::executor::Executor;

pub struct MyWatchmanService {
    pub executor: Arc<Executor>,
    pub config: Arc<tokio::sync::RwLock<Config>>,
}

#[tonic::async_trait]
impl WatchmanService for MyWatchmanService {
    async fn get_status(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<StatusResponse>, Status> {
        let config = self.config.read().await;
        Ok(Response::new(StatusResponse {
            version: "0.1.0".to_string(),
            rules_count: config.watch.len() as u32,
            tasks_count: config.tasks.len() as u32,
        }))
    }

    async fn trigger_task(
        &self,
        request: Request<TriggerRequest>,
    ) -> Result<Response<TriggerResponse>, Status> {
        let task_name = request.into_inner().task_name;
        let config = self.config.read().await;
        let executor = self.executor.clone();

        if let Some(task_def) = config.tasks.get(&task_name).cloned() {
            info!("Remote trigger for task: {}", task_name);
            let name = task_name.clone();

            tokio::spawn(async move {
                let envs: std::collections::HashMap<String, String> =
                    std::collections::HashMap::new();
                if let Err(e) = executor.run_task(&name, &task_def, envs).await {
                    tracing::error!("Remote task trigger failed: {:?}", e);
                }
            });

            Ok(Response::new(TriggerResponse {
                success: true,
                message: format!("Task {} triggered successfully", task_name),
            }))
        } else {
            Ok(Response::new(TriggerResponse {
                success: false,
                message: format!("Task {} not found", task_name),
            }))
        }
    }
}

pub async fn start(service: MyWatchmanService) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    info!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(WatchmanServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
