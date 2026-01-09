use crate::cli::DebugArgs;
use anyhow::Result;

pub fn init() {
    tracing_subscriber::fmt::init();
}

pub async fn debug_info(args: DebugArgs) -> Result<()> {
    println!("Debug info: {:?}", args);
    Ok(())
}
