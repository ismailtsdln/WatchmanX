mod cli;
mod config;
mod executor;
mod logger;
mod watcher;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();
    logger::print_banner();

    let args = Cli::parse();
    info!("Starting {}", "WatchmanX".bright_cyan().bold());

    match args.command {
        cli::Commands::Watch(cmd) => watcher::run(cmd).await?,
        cli::Commands::Task(cmd) => executor::manage_tasks(cmd).await?,
        cli::Commands::Run(cmd) => executor::run_once(cmd).await?,
        cli::Commands::Config(cmd) => config::manage(cmd).await?,
        cli::Commands::Debug(cmd) => logger::debug_info(cmd).await?,
    }

    Ok(())
}
