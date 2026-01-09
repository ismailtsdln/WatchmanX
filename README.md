# WatchmanX

**WatchmanX** is a high-performance, cross-platform file watcher and automation tool written in Rust. It is designed to be a modern, robust alternative to existing tools, providing efficient file monitoring and task execution capabilities.

## Features

- **🚀 High Performance**: Built with Rust and `notify` for efficient event handling and low resource usage.
- **⚡ Parallel Task Execution**: Supports concurrent task execution with configurable limits.
- **🔧 Flexible Configuration**: Supports YAML, JSON, and TOML configuration files.
- **🔍 Advanced Filtering**: Glob and Regex pattern matching for precise file monitoring.
- **🛠️ Cross-Platform**: Works seamlessly on Linux, macOS, and Windows.

## Installation

### From Source

```bash
git clone https://github.com/ismailtsdln/WatchmanX.git
cd WatchmanX
cargo install --path .
```

## Usage

### Watch a Directory

```bash
watchmanx watch ./src
```

### Run a Task Manually

```bash
watchmanx run build
```

## Roadmap

- [ ] Core File Watching Engine
- [ ] Task Execution System
- [ ] Configuration Management
- [ ] Plugin System

## License

This project is licensed under the MIT License.
