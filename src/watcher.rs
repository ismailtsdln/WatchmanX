use anyhow::{Context, Result};
use colored::Colorize;
use glob::Pattern;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::sync::mpsc::{channel as tokio_channel, Receiver};
use tracing::{debug, error, info, warn};

use crate::cli::WatchArgs;
use crate::config::{load_config, Config as AppConfig};
use crate::executor::Executor;

pub async fn run(args: WatchArgs) -> Result<()> {
    // Load config
    let config = load_config(None).await?;
    info!(
        "Loaded configuration with {} watch rules",
        config.watch.len()
    );

    // Initialize Executor
    let executor = Executor::new(5); // TODO: Make configurable

    info!("Starting file watcher on paths: {:?}", args.paths);

    let (tx, mut rx) = tokio_channel(100);
    let (std_tx, std_rx) = channel();

    let mut watcher =
        RecommendedWatcher::new(std_tx, Config::default()).context("Failed to create watcher")?;

    for path in &args.paths {
        if path.exists() {
            watcher
                .watch(path, RecursiveMode::Recursive)
                .with_context(|| format!("Failed to watch path {:?}", path))?;
            info!("Watching {}", format!("{:?}", path).bright_blue());
        } else {
            error!("Path {} does not exist", format!("{:?}", path).red());
        }
    }

    tokio::task::spawn_blocking(move || {
        while let Ok(res) = std_rx.recv() {
            match res {
                Ok(event) => {
                    let _ = tx.blocking_send(event);
                }
                Err(e) => error!("Watch error: {:?}", e),
            }
        }
    });

    process_events(&mut rx, &config, &executor).await;

    Ok(())
}

async fn process_events(rx: &mut Receiver<Event>, config: &AppConfig, executor: &Executor) {
    let mut debounce_timer = None;
    let mut pending_events = Vec::new();
    let debounce_duration = Duration::from_millis(500);

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                if !event.kind.is_access() {
                    pending_events.push(event);
                    // Reset or start timer
                    debounce_timer = Some(tokio::time::sleep(debounce_duration));
                }
            }
            _ = async {
                if let Some(ref mut timer) = debounce_timer {
                    timer.await;
                } else {
                    futures::future::pending::<()>().await;
                }
            }, if debounce_timer.is_some() => {
                // Timer expired, process all pending events
                let events = std::mem::take(&mut pending_events);
                debounce_timer = None;
                handle_batched_events(events, config, executor).await;
            }
            else => break,
        }
    }
}

async fn handle_batched_events(events: Vec<Event>, config: &AppConfig, executor: &Executor) {
    let mut rule_triggers: std::collections::HashMap<usize, Vec<PathBuf>> =
        std::collections::HashMap::new();

    for event in events {
        for path in event.paths {
            for (idx, rule) in config.watch.iter().enumerate() {
                if matches_rule(&path, rule) {
                    rule_triggers.entry(idx).or_default().push(path);
                }
            }
        }
    }

    for (rule_idx, paths) in rule_triggers {
        let rule = &config.watch[rule_idx];

        // Remove duplicates and construct paths string
        let mut unique_paths: Vec<_> = paths
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        unique_paths.sort();
        unique_paths.dedup();
        let paths_str = unique_paths.join(",");

        info!(
            "{} events matched rule at {}. Triggering tasks: {:?}",
            unique_paths.len().to_string().bright_green(),
            rule.path.to_string_lossy().bright_blue(),
            rule.tasks
        );

        for task_name in &rule.tasks {
            if let Some(task_def) = config.tasks.get(task_name) {
                // Add context to environment
                let mut envs = std::collections::HashMap::new();
                envs.insert("WATCHMANX_EVENT_PATHS".to_string(), paths_str.clone());
                envs.insert("WATCHMANX_TASK_NAME".to_string(), task_name.clone());

                if let Err(e) = executor.run_task(task_name, task_def, envs).await {
                    error!("Failed to run task {}: {:?}", task_name, e);
                }
            } else {
                warn!("Task {} not found in config", task_name);
            }
        }
    }
}

fn matches_rule(path: &PathBuf, rule: &crate::config::WatchRule) -> bool {
    // Convert path to string for glob matching
    let path_str = match path.to_str() {
        Some(s) => s,
        None => return false,
    };

    // If patterns is empty, maybe match everything? Or nothing?
    // Let's assume if patterns are provided, must match at least one
    if rule.patterns.is_empty() {
        return true;
    }

    // Check inclusion patterns
    let mut matched = false;
    for pattern_str in &rule.patterns {
        if let Ok(pattern) = Pattern::new(pattern_str) {
            if pattern.matches(path_str) {
                matched = true;
                break;
            }
        } else {
            warn!("Invalid glob pattern: {}", pattern_str);
        }
    }

    if !matched {
        return false;
    }

    // Check exclusion patterns
    for ignore_str in &rule.ignore {
        if let Ok(pattern) = Pattern::new(ignore_str) {
            if pattern.matches(path_str) {
                debug!("Path {:?} ignored by pattern {}", path, ignore_str);
                return false;
            }
        }
    }

    true
}
