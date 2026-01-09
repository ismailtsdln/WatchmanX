use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tracing::{error, info, instrument};

use crate::cli::{RunArgs, TaskArgs};
use crate::config::load_config;
use crate::config::TaskDefinition;

// Global concurrency limit - could be configurable
const MAX_CONCURRENT_TASKS: usize = 5;

pub struct Executor {
    semaphore: Arc<Semaphore>,
}

impl Executor {
    pub fn new(limit: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(limit)),
        }
    }

    #[instrument(skip(self, task), fields(cmd = %task.command))]
    pub async fn run_task(&self, task_name: &str, task: &TaskDefinition) -> Result<()> {
        let _permit = self.semaphore.acquire().await?;
        info!("Starting task: {}", task_name);

        let mut cmd = Command::new(&task.command);
        cmd.args(&task.args);

        if let Some(cwd) = &task.cwd {
            cmd.current_dir(cwd);
        }

        let status = cmd
            .status()
            .await
            .with_context(|| format!("Failed to execute command: {}", task.command))?;

        if status.success() {
            info!("Task {} completed successfully", task_name);
        } else {
            error!(
                "Task {} failed with exit code: {:?}",
                task_name,
                status.code()
            );
        }

        Ok(())
    }
}

pub async fn manage_tasks(args: TaskArgs) -> Result<()> {
    // TODO: Implement CRUD for tasks in config
    println!("Task management not implemented yet: {:?}", args);
    Ok(())
}

pub async fn run_once(args: RunArgs) -> Result<()> {
    let config = load_config(None).await?;

    if let Some(task) = config.tasks.get(&args.task_name) {
        let executor = Executor::new(MAX_CONCURRENT_TASKS);
        executor.run_task(&args.task_name, task).await?;
    } else {
        error!("Task '{}' not found in configuration", args.task_name);
    }

    Ok(())
}
