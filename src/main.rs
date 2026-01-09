mod cli;
mod config;
mod executor;
mod logger;
mod rpc;
mod server;
mod watcher;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();
    logger::print_banner();

    let args = Cli::parse();
    info!("Starting {}", "WatchmanX".bright_cyan().bold());

    // Initialize Dashboard State
    let (server_tx, _) = tokio::sync::broadcast::channel(100);
    let server_state = std::sync::Arc::new(server::ServerState {
        tx: server_tx.clone(),
    });
    let dashboard_state = server_state.clone();

    tokio::spawn(async move {
        server::start(dashboard_state).await;
    });

    // Initialize gRPC (Roadmap)
    // We'll need a way to share the current executor/config
    // For now, let's create a shared executor
    let global_executor = std::sync::Arc::new(executor::Executor::new(10));
    let initial_config = std::sync::Arc::new(config::load_config(None).await.unwrap_or_default());

    let rpc_service = rpc::MyWatchmanService {
        executor: global_executor.clone(),
        config: initial_config.clone(),
    };

    tokio::spawn(async move {
        if let Err(e) = rpc::start(rpc_service).await {
            error!("gRPC Server error: {:?}", e);
        }
    });

    let token = tokio_util::sync::CancellationToken::new();
    let cloned_token = token.clone();

    tokio::spawn(async move {
        if let Ok(_) = tokio::signal::ctrl_c().await {
            info!("{}", "Shutting down gracefully...".yellow());
            cloned_token.cancel();
        }
    });

    loop {
        let result = match args.command.clone() {
            cli::Commands::Watch(cmd) => {
                tokio::select! {
                    res = watcher::run(cmd.clone(), cloned_token.clone(), server_tx.clone(), global_executor.clone()) => res,
                    _ = cloned_token.cancelled() => Ok(()),
                }
            }
            cli::Commands::Task(cmd) => executor::manage_tasks(cmd.clone()).await,
            cli::Commands::Run(cmd) => executor::run_once(cmd.clone()).await,
            cli::Commands::Config(cmd) => config::manage(cmd.clone()).await,
            cli::Commands::Debug(cmd) => logger::debug_info(cmd.clone()).await,
        };

        if let Err(e) = result {
            // Check if it's a reload signal
            if e.to_string().contains("RELOAD_CONFIG") {
                info!(
                    "{}",
                    "Config change detected. Reloading..."
                        .bright_magenta()
                        .bold()
                );
                continue;
            }
            error!("Error: {:?}", e);
            break;
        } else {
            break;
        }
    }

    Ok(())
}
