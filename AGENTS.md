# AGENTS.md

## Overview

Friendev is an AI-powered development assistant written in Rust that provides a command-line interface for interacting with AI models through OpenAI-compatible APIs. It supports streaming responses, tool execution for file operations, web search, and maintains conversation history with context persistence.

## Dev Environment

- Rust 1.70+ (Edition 2021)
- Operating Systems: Windows, Linux, macOS
- Required: OpenAI API key and compatible API endpoint

### Initial Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/your-repo/friendev.git
cd friendev

# Build the project
cargo build --release

# Run the application (first run will prompt for configuration)
cargo run --release
```

## Project Structure

```
friendev/
├── src/
│   ├── main.rs              # Application entry point and REPL loop
│   ├── api.rs               # API client for OpenAI-compatible endpoints
│   ├── agents.rs            # AGENTS.md handling utilities
│   ├── chat.rs              # Chat message handling
│   ├── commands.rs          # CLI command processing
│   ├── config.rs            # Configuration management
│   ├── history.rs           # Chat session management
│   ├── i18n.rs              # Internationalization support
│   ├── prompts.rs           # System prompts and messages
│   ├── search_tool.rs       # Web search functionality
│   ├── security.rs          # Input security checks
│   ├── tools/               # Tool execution modules
│   │   ├── mod.rs           # Tool module definitions
│   │   ├── definitions.rs   # Tool schema definitions
│   │   ├── executor.rs      # Tool execution logic
│   │   ├── types.rs         # Tool type definitions
│   │   ├── args.rs          # Argument parsing for tools
│   │   └── utils.rs         # Utility functions
│   └── ui.rs                # Terminal UI components
├── example/                 # Example files for testing
├── target/                  # Build output directory (gitignored)
├── Cargo.toml               # Project dependencies and metadata
├── Cargo.lock               # Dependency lock file (gitignored)
└── README.md                # Project documentation
```

## Build & Compilation

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

The compiled binary will be located at:
- Debug: `target/debug/friendev`
- Release: `target/release/friendev`

## Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test modulename

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

Test files are located in the standard Rust locations:
- Unit tests: Within each source file in `src/`
- Integration tests: `tests/` directory (if present)

## Code Style & Standards

- Rustfmt for code formatting: `cargo fmt`
- Clippy for linting: `cargo clippy`
- Conventions:
  - Use `Result<T>` for error handling with `anyhow` crate
  - Async/await pattern with Tokio runtime
  - Structured logging with appropriate error messages
  - Module organization following Rust best practices

## Running the Application

```bash
# Development build
cargo run

# Release build
cargo run --release

# Or directly execute the binary
./target/release/friendev
```

### Configuration

On first run, Friendev will prompt for:
- OpenAI API key
- API endpoint URL (default: https://api.openai.com/v1)
- Default model (default: gpt-4)
- UI language (default: en)
- AI language (default: en)

Configuration is stored in:
- Windows: `%APPDATA%\friendev\config.json`
- Linux/macOS: `~/.config/friendev/config.json`

### Available Commands

- `/help` - Show available commands
- `/model <name>` - Change the current AI model
- `/models` - List available models from the API
- `/language <lang>` - Change UI language
- `/reset` - Reset conversation history
- `/agents.md` - Generate AGENTS.md for current project
- `/exit` or `Ctrl+D` - Exit the application

## API & Dependencies

### Key Dependencies

- `tokio` (1.0+) - Async runtime
- `reqwest` (0.11+) - HTTP client with TLS support
- `serde` (1.0+) - Serialization/deserialization
- `clap` (4.0+) - Command-line argument parsing
- `ratatui` (0.26+) - Terminal UI framework
- `reedline` (0.28+) - Modern readline implementation
- `crossterm` (0.27+) - Cross-platform terminal control

### API Requirements

The application requires an OpenAI-compatible API endpoint that supports:
- Chat completions endpoint (`/chat/completions`)
- Streaming responses
- Tool/function calling
- Models listing endpoint (`/models`)

### Tool Capabilities

Friendev supports these tools for AI interaction:
- `file_list` - List directory contents
- `file_read` - Read file contents
- `file_write` - Write content to files
- `file_replace` - Replace content in files
- `network_search_duckduckgo` - Search using DuckDuckGo
- `network_search_bing` - Search using Bing
- `network_search_auto` - Search with automatic fallback

## Troubleshooting

### Common Issues

1. **API Connection Errors**
   - Verify API key is correct
   - Check network connectivity
   - Confirm API endpoint URL is accessible

2. **JSON Parsing Errors**
   - Some models may produce incomplete JSON for tool calls
   - The application includes automatic JSON repair mechanisms
   - If persistent, try switching to a different model

3. **Terminal Display Issues**
   - Ensure your terminal supports ANSI escape codes
   - Windows users may need to enable virtual terminal processing
   - Try reducing terminal width if display is corrupted

4. **Configuration Problems**
   - Delete config file and reinitialize: `rm ~/.config/friendev/config.json`
   - Check file permissions on config directory

### Debug Mode

To enable more verbose output for troubleshooting:
```bash
RUST_LOG=debug cargo run
```

## Contributing

### Git Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make changes and add tests
4. Run tests and linting: `cargo test && cargo clippy`
5. Commit with conventional commit format
6. Push to your fork
7. Create a pull request

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: feat, fix, docs, style, refactor, test, chore

Example:
```
feat(api): add retry mechanism for failed requests

Implements exponential backoff with configurable max retries
and delay between attempts.

Closes #123
```

### Code Review Guidelines

- Ensure all tests pass
- Follow existing code style
- Add documentation for new features
- Update CHANGELOG.md for user-facing changes