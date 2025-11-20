# AGENTS.md

## Overview

Friendev is an AI-powered development assistant written in Rust that provides a terminal-based REPL interface for interacting with AI models and development tools. It enables developers to leverage AI capabilities for coding assistance, file management, web search, and command execution.

## Dev Environment

### Prerequisites
- Rust 1.70+ (stable toolchain)
- Cargo package manager

### Setup
```bash
# Clone the repository
git clone https://github.com/your-repo/friendev.git
cd friendev

# Install dependencies
cargo build --release
```

## Project Structure

```
src/
├── agents/          # AI agent implementations
├── api/             # API client implementations
├── app/             # Application initialization and main logic
├── chat/            # Chat functionality and message handling
├── commands/        # Command parsing and execution
├── config/          # Configuration management
├── history/         # Session history management
├── i18n/            # Internationalization support
├── search_tool/     # Web search functionality
├── tools/           # Development tools implementation
├── ui/              # Terminal UI components
├── main.rs          # Application entry point
├── prompts.rs       # AI prompt definitions
└── security.rs      # Security-related functionality
```

## Build & Compilation

### Build Commands
```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release

# Cross-platform builds (requires cross tool for non-native targets)
cargo install cross --locked
cross build --release --target <target-triple>
```

### Output Locations
- Development builds: `target/debug/friendev`
- Release builds: `target/release/friendev`
- Cross-platform builds: `target/<target-triple>/release/friendev`

## Testing

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo test -- --nocapture

# Run specific test module
cargo test <module_name>
```

## Code Style & Standards

### Rust Formatting
```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

### Linting
```bash
# Run clippy lints
cargo clippy

# Run clippy with all checks
cargo clippy --all-targets --all-features -- -D warnings
```

### Code Conventions
- Follow Rust API Guidelines (RUST-0001 to RUST-0021)
- Use `anyhow::Result` for error handling
- Implement `async/await` patterns for I/O operations
- Structure modules with clear separation of concerns

## Running the Application

### Start Command
```bash
# Development version
cargo run

# Release version
./target/release/friendev

# With specific parameters
./target/release/friendev --setup --ally
```

### Command Line Options
- `--setup`: Force initial setup workflow
- `--ally`: Automatically approve all "Approval Required" prompts

### Configuration
- Configuration files are stored in user's config directory (determined by `dirs` crate)
- Language preferences are stored in the config file
- Session history is automatically managed

## API & Dependencies

### Core Dependencies
- `tokio` v1: Async runtime with full features
- `reqwest` v0.11: HTTP client with JSON, streaming, and TLS support
- `serde` v1.0: Serialization framework with derive features
- `clap` v4: Command line argument parsing
- `uuid` v1: UUID generation with v4 and serde support

### UI Dependencies
- `ratatui` v0.26: Modern TUI framework
- `crossterm` v0.27: Cross-platform terminal control
- `reedline` v0.28: Modern readline implementation
- `indicatif` v0.17: Progress indicators and spinners

### Tool Dependencies
- `syntect` v5.1: Syntax highlighting
- `scraper` v0.19: HTML parsing
- `regex` v1.10: Regular expressions
- `url` v2.5: URL parsing and manipulation

## Troubleshooting

### Common Issues

**Build Failures**
```bash
# Update Rust toolchain
rustup update stable

# Clean build artifacts
cargo clean
```

**Cross-compilation Issues**
```bash
# Install cross-compilation tool
cargo install cross --locked

# Build for specific target
cross build --release --target <target-triple>
```

**Configuration Issues**
```bash
# Reset configuration
rm -rf ~/.config/friendev
./target/release/friendev --setup
```

## Contributing

### Git Workflow
1. Create feature branch from `main`
2. Make changes with appropriate commits
3. Ensure all tests pass and code is formatted
4. Submit pull request to `main` branch

### Commit Message Format
Follow conventional commits format:
```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Pull Request Guidelines
- Include description of changes
- Link to relevant issues
- Ensure CI builds successfully
- Update documentation as needed