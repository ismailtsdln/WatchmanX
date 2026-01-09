use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "watchmanx")]
#[command(about = "A high-performance file watcher and automation tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Start watching given paths
    Watch(WatchArgs),
    /// Create/Edit/Delete tasks
    Task(TaskArgs),
    /// Execute configured tasks manually
    Run(RunArgs),
    /// Create/Edit configuration
    Config(ConfigArgs),
    /// Output detailed diagnostics
    Debug(DebugArgs),
}

#[derive(Args, Debug, Clone)]
pub struct WatchArgs {
    /// Paths to watch
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Optional WASM plugin for event filtering
    #[arg(short, long)]
    pub plugin: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct TaskArgs {
    /// Task operation
    pub operation: String, // Placeholder
}

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    /// Task name to run
    pub task_name: String,
}

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    /// Config operation
    pub operation: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct DebugArgs {
    /// Enable verbose debug output
    #[arg(short, long)]
    pub verbose: bool,
}
