<p align="center">
  <img src="/Users/ismailtasdelen/.gemini/antigravity/brain/6e01e81a-1cc9-441f-8e56-481942d92d6a/watchmanx_logo_v1_1767958634561.png" width="200" alt="WatchmanX Logo">
</p>

<h1 align="center">WatchmanX</h1>

<p align="center">
  <strong>Advanced, high-performance file watcher and automation engine built in Rust.</strong>
</p>

<p align="center">
  <a href="https://github.com/ismailtsdln/WatchmanX"><img src="https://img.shields.io/badge/WatchmanX-v0.1.0-cyan.svg" alt="Version"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg" alt="Rust"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License"></a>
  <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome">
</p>

---

## 🌟 Overview

**WatchmanX** is a modern, blazing-fast file system monitor designed for developers who need reliable automation. Whether you're building a custom asset pipeline, running automated tests, or managing live-reloads for complex environments, WatchmanX provides the precision and performance required.

Built on top of the `tokio` async runtime and the `notify` crate, it handles thousands of file events with minimal overhead, while offering advanced features like event debouncing and configuration hot-reload.

## ✨ Key Features

- ⚡ **Asynchronous Core**: Powered by Rust's `tokio` for non-blocking I/O and high concurrency.
- 🔍 **Native Watching**: Cross-platform support using native OS APIs (Inotify, FSEvents, ReadDirectoryChangesW).
- 🧠 **Event Debouncing**: Group rapid file changes (500ms window) into single triggers to optimize resource usage.
- 🔄 **Hot-Reload**: Automatically re-applies configuration changes without restarting the main process.
- 🛠️ **Task Context**: Inject event metadata directly into your automation via environment variables.
- 🎨 **Modern CLI**: Beautiful, colorized output with professional ASCII art and structured logging.
- 🛡️ **Safe & Reliable**: Graceful handling of `SIGINT` (Ctrl+C) ensures task cleanup and process safety.

## 🚀 Quick Start

### Installation

Clone the repository and install using `cargo`:

```bash
git clone https://github.com/ismailtsdln/WatchmanX.git
cd WatchmanX
cargo install --path .
```

### Configuration

Create a `watchmanx.yml` file in your project directory:

```yaml
watch:
  - path: "./src"
    recursive: true
    patterns:
      - "**/*.rs"
    ignore:
      - "**/target/**"
    tasks:
      - "build-test"

tasks:
  build-test:
    command: "cargo"
    args: ["test", "--color", "always"]
    cwd: "."
```

### Usage

**Start Watching:**
```bash
watchmanx watch .
```

**List Available Tasks:**
```bash
watchmanx task list
```

**Run a Task Manually:**
```bash
watchmanx run build-test
```

## 📖 Advanced Concepts

### Event Context (Env Vars)
When WatchmanX triggers a task, it populates the environment with useful metadata:

- `WATCHMANX_EVENT_PATHS`: Comma-separated list of paths that changed.
- `WATCHMANX_TASK_NAME`: The name of the currently executing task definition.

Example task using context:
```yaml
tasks:
  logger:
    command: "echo"
    args: ["Changed files: $WATCHMANX_EVENT_PATHS"]
```

### Debouncing Logic
WatchmanX uses a "sliding window" for events. If multiple changes occur within 500ms of each other, they are grouped. This is essential for tools that perform multiple sequential writes (like compilers or formatters).

### Hot-Reloading
If you modify your `watchmanx.yml` or `.json` configuration file, WatchmanX will automatically detect the change, reload its internal state, and restart the watcher threads with the new rules—all without dropping the process.

## 🗺️ Roadmap

- [x] High-performance Async Watcher
- [x] Configurable Debouncing
- [x] Config Hot-Reload
- [x] Environment Variable Context
- [ ] Plugin Support (WASM-based event filters)
- [ ] Web-based Monitoring Dashboard
- [ ] Remote Task Execution via gRPC

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  Built with ❤️ using <b>Rust</b>
</p>
