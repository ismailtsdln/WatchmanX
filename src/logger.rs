use crate::cli::DebugArgs;
use anyhow::Result;
use colored::*;

pub fn init() {
    tracing_subscriber::fmt().with_target(false).init();
}

pub fn print_banner() {
    let banner = r#"
    __      __         _         _                               __   __
    \ \    / /        | |       | |                              \ \ / /
     \ \  / /_ _ _ ___| |__  _ _| | __ _ _ _  __  __   _____  __ \   / 
      \ \/ / _` | ' __| '_ \| ' | |/ _` | ' \|  \/  | / _ \ \/ / /   \ 
       \  / (_| | |   | | | | | | | (_| | | | | |  | | | (_| >  < / / \ \
        \/ \__,_|_|   |_| |_|_| |_|\__,_|_| |_|_|  |_| \__/_/\_\/_/   \_\
    "#;
    println!("{}", banner.bright_cyan().bold());
    println!(
        "    {} v{}\n",
        "Advanced File Watching & Automation".bright_white(),
        env!("CARGO_PKG_VERSION").bright_green()
    );
}

pub async fn debug_info(args: DebugArgs) -> Result<()> {
    println!("Debug info: {:?}", args);
    Ok(())
}
