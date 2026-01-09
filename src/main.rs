mod cli;
mod config;
mod executor;
mod logger;
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
                    res = watcher::run(cmd.clone(), token.clone()) => res,
                    _ = token.cancelled() => Ok(()),
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
