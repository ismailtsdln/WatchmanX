# WatchmanX 🚀

Advanced, high-performance file watcher and automation tool written in Rust.

![WatchmanX Logo](/Users/ismailtasdelen/.gemini/antigravity/brain/6e01e81a-1cc9-441f-8e56-481942d92d6a/watchmanx_logo_v1_1767958634561.png)

![WatchmanX Banner](https://img.shields.io/badge/WatchmanX-CLI-cyan)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)
![License](https://img.shields.io/badge/License-MIT-green)

## Features

- **Blazing Fast**: Built with Rust and `tokio` for high-performance event processing.
- **Cross-Platform**: Uses `notify` crate for native OS events on Linux, macOS, and Windows.
- **Advanced Debouncing**: Groups rapid file changes (500ms window) to avoid task flooding.
- **Configuration Hot-Reload**: Automatically reloads monitoring rules when the config file changes.
- **Task Context**: Passes event paths and task metadata via environment variables (`$WATCHMANX_EVENT_PATHS`).
- **Flexible Rules**: Glob-based pattern matching for inclusion and exclusion.
- **Colorful CLI**: Professional UI with stylized banners and status logs.
- **Graceful Shutdown**: Cleanly terminates event loops and child processes on `Ctrl+C`.

## Installation

```bash
cargo install --path .
```

## Configuration

Create a `watchmanx.yml` in your project root:

```yaml
watch:
  - path: .
    recursive: true
    patterns:
      - "src/**/*.rs"
    ignore:
      - "target/**"
    tasks:
      - lint-and-test

tasks:
  lint-and-test:
    command: cargo
    args: ["test"]
    cwd: .
```

## Usage

### Watch for changes
```bash
watchmanx watch .
```

### Run a task manually
```bash
watchmanx run lint-and-test
```

### List configured tasks
```bash
watchmanx task list
```

## Environment Variables for Tasks

When a task is triggered, the following variables are available:
- `WATCHMANX_EVENT_PATHS`: Comma-separated list of paths that triggered the task.
- `WATCHMANX_TASK_NAME`: The name of the triggering task.

## Roadmap

- [x] Core Watcher Engine
- [x] Advanced Debouncing
- [x] Hot-Reloading
- [x] Environment Context for Tasks
- [ ] Remote Monitoring Dashboard
- [ ] Plugin System (WASM)

## License
MIT
