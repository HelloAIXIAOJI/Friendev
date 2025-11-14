# AGENTS.md

## Overview

Friendev is an AI-powered development assistant written in Rust that provides a command-line interface for interacting with AI models. It features file system operations, web search capabilities, and a terminal-based UI for enhanced user interaction.

## Dev Environment

### Prerequisites
- Rust 1.70+ (edition 2021)
- Cargo package manager
- OpenAI API key

### Setup
```bash
# Clone the repository
git clone <repository-url>
cd friendev

# Build the project
cargo build --release

# Run the application
cargo run
```

### First-time Configuration
When first running Friendev, you'll be prompted to configure:
- OpenAI API key
- API URL (default: https://api.openai.com/v1)
- Default model (default: gpt-4)

## Project Structure

```
friendev/
├── src/
│   ├── agents.rs        # AGENTS.md generation and analysis
│   ├── api.rs           # API client implementation
│   ├── chat.rs          # Chat interaction handling
│   ├── commands.rs      # CLI command processing
│   ├── config.rs        # Configuration management
│   ├── history.rs       # Chat session history
│   ├── i18n.rs          # Internationalization
│   ├── main.rs          # Application entry point
│   ├── prompts.rs       # System prompts
│   ├── search_tool.rs   # Web search functionality
│   ├── security.rs      # Security checks
│   ├── tools/           # Tool implementations
│   │   ├── executor.rs  # Tool execution logic
│   │   ├── definitions.rs  # Tool definitions
│   │   ├── types.rs     # Tool types
│   │   ├── utils.rs     # Utility functions
│   │   └── args.rs      # Tool argument structures
│   └── ui.rs            # User interface components
├── example/             # Example HTML files
└── Cargo.toml           # Project configuration
```

## Build & Compilation

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name
```

The binary will be located at `target/release/friendev` for release builds.

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test modules::name
```

## Code Style & Standards

- Uses Rust 2021 edition
- Formatting with `rustfmt`
- Linting with `clippy`
- Uses `anyhow` for error handling
- Async code using `tokio` runtime

## Running the Application

```bash
# Run from source
cargo run

# Run binary
./target/release/friendev
```

### Available Commands

- `/help` - Show available commands
- `/model list` - List available models
- `/model switch <name>` - Switch to a different model
- `/history list` - Show chat history
- `/history new` - Create new chat session
- `/history switch <id>` - Switch to chat session
- `/history del <id>` - Delete chat session
- `/language ui <lang>` - Set UI language (en/zh)
- `/language ai <lang>` - Set AI language (en/zh)
- `/agents.md` - Generate AGENTS.md file for current project
- `/exit` - Exit the application

## API & Dependencies

### Key Dependencies
- `tokio` v1 - Async runtime
- `reqwest` v0.11 - HTTP client
- `serde` v1.0 - Serialization
- `clap` v4 - CLI argument parsing
- `ratatui` v0.26 - Terminal UI
- `reedline` v0.28 - Command line editing
- `anyhow` v1.0 - Error handling

### API Configuration
Friendev uses OpenAI-compatible APIs. Configuration is stored in:
- Linux/Mac: `~/.config/friendev/config.json`
- Windows: `%APPDATA%\friendev\config.json`

## Troubleshooting

### Common Issues

1. **API Connection Errors**
   - Verify API key is correct
   - Check network connectivity
   - Ensure API URL is accessible

2. **File Operation Failures**
   - Check file permissions
   - Verify file paths
   - Ensure directory exists

3. **Configuration Issues**
   - Delete config file to reinitialize
   - Check config file JSON syntax

4. **Build Errors**
   - Ensure Rust version is compatible
   - Run `cargo clean` then rebuild
   - Check for dependency conflicts

## Contributing

### Git Workflow
1. Fork the repository
2. Create a feature branch
3. Make changes with proper commit messages
4. Submit a pull request

### Commit Message Format
```
type(scope): description

[optional body]

[optional footer]
```

Types: feat, fix, docs, style, refactor, test, chore

### Code Standards
- Follow Rust conventions
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting