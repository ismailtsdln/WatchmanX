use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

use crate::cli::ConfigArgs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub watch: Vec<WatchRule>,
    pub tasks: HashMap<String, TaskDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchRule {
    pub path: PathBuf,
    pub recursive: bool,
    pub patterns: Vec<String>,
    pub ignore: Vec<String>,
    pub tasks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDefinition {
    pub command: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch: vec![],
            tasks: HashMap::new(),
        }
    }
}

pub async fn load_config(path: Option<&PathBuf>) -> Result<Config> {
    let config_path = if let Some(p) = path {
        p.clone()
    } else {
        // Try default locations
        let defaults = vec![
            "watchmanx.yml",
            "watchmanx.yaml",
            "watchmanx.json",
            "watchmanx.toml",
        ];
        let mut found = None;
        for d in defaults {
            let p = PathBuf::from(d);
            if fs::try_exists(&p).await.unwrap_or(false) {
                found = Some(p);
                break;
            }
        }

        match found {
            Some(p) => p,
            None => return Ok(Config::default()),
        }
    };

    let content = fs::read_to_string(&config_path)
        .await
        .with_context(|| format!("Failed to read config file {:?}", config_path))?;

    let config: Config = if config_path.extension().map_or(false, |e| e == "json") {
        serde_json::from_str(&content)?
    } else if config_path.extension().map_or(false, |e| e == "toml") {
        toml::from_str(&content)?
    } else {
        serde_yaml::from_str(&content)?
    };

    Ok(config)
}

pub async fn manage(_args: ConfigArgs) -> Result<()> {
    // For now, just print the current config
    let config = load_config(None).await?;
    println!("Current Configuration:\n{:#?}", config);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_default_config() {
        let config = Config::default();
        assert!(config.watch.is_empty());
        assert!(config.tasks.is_empty());
    }

    #[tokio::test]
    async fn test_load_yaml_config() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("watchmanx.yml");
        let mut file = File::create(&file_path)?;
        writeln!(
            file,
            "
watch:
  - path: .
    recursive: true
    patterns: ['*.rs']
    ignore: ['target/*']
    tasks: ['test']
tasks:
  test:
    command: cargo
    args: ['test']
"
        )?;

        let config = load_config(Some(&file_path)).await?;
        assert_eq!(config.watch.len(), 1);
        assert_eq!(config.watch[0].patterns[0], "*.rs");
        assert!(config.tasks.contains_key("test"));
        Ok(())
    }
}
