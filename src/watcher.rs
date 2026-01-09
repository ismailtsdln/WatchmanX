use anyhow::{Context, Result};
use glob::Pattern;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
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
            info!("Watching {:?}", path);
        } else {
            error!("Path {:?} does not exist", path);
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
    while let Some(event) = rx.recv().await {
        debug!("Received event: {:?}", event);
        handle_event(event, config, executor).await;
    }
}

async fn handle_event(event: Event, config: &AppConfig, executor: &Executor) {
    // Only care about modification, creation, or removal
    // Notify events can be complex, for now let's just react to any non-access event
    if event.kind.is_access() {
        return;
    }

    for path in event.paths {
        // Check against config rules
        for rule in &config.watch {
            if matches_rule(&path, rule) {
                info!(
                    "File {:?} matched rule. Triggering tasks: {:?}",
                    path, rule.tasks
                );
                for task_name in &rule.tasks {
                    if let Some(task_def) = config.tasks.get(task_name) {
                        // Fire and forget (or await if we want sequential?)
                        // We use the executor which handles concurrency
                        // We clone because run_task is async and might outlive loop?
                        // Actually run_task is &self.
                        if let Err(e) = executor.run_task(task_name, task_def).await {
                            error!("Failed to run task {}: {:?}", task_name, e);
                        }
                    } else {
                        warn!("Task {} not found in config", task_name);
                    }
                }
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
