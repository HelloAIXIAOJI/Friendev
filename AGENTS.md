# AGENTS.md

## Overview

Friendev is an AI-powered development assistant built in Rust. It provides an interactive REPL environment with prompt optimization, multi-line input support, and various development tools for AI-assisted programming tasks.

## Dev Environment

### Prerequisites
- Rust 1.70+ (edition 2021)
- Cargo package manager
- Git
- Cross-compilation tools (for building multiple targets)

### Setup Steps
```bash
# Clone the repository
git clone <repository-url>
cd friendev

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Version Requirements
- Rust: 1.70+
- Cargo: Latest stable
- tokio: 1.0 with full features
- reedline: 0.28
- crossterm: 0.27

## Project Structure

```
friendev/
├── agents_md_file/     # AGENTS.md file generation and analysis
├── api/               # API client and integration
├── app/               # Main application logic and REPL
├── chat/              # Chat functionality
├── commands/          # Command handling system
├── config/            # Configuration management
├── history/           # Session history management
├── i18n/              # Internationalization support
├── prompts/           # Prompt templates and optimization
├── search_tool/       # Web search capabilities
├── security/          # Security and authentication
├── src/               # Legacy source code (being migrated)
├── tools/             # AI tools and utilities
├── tui/               # Terminal UI components
└── ui/                # UI components and i18n integration
```

### Key Components
- **app**: Main application entry point and REPL loop
- **commands**: Command processing and execution
- **api**: External API integrations (OpenAI, etc.)
- **config**: Configuration management
- **history**: Session persistence and history
- **i18n**: Multi-language support
- **tools**: AI-powered development tools
- **ui**: User interface components

## Build & Compilation

### Build Commands
```bash
# Build for current platform
cargo build --release

# Build for specific target
cargo build --release --target <target-triple>

# Build all workspace members
cargo build --workspace --release

# Run tests
cargo test

# Check code
cargo check
```

### Output Locations
- Release binaries: `target/release/`
- Debug binaries: `target/debug/`
- Documentation: `target/doc/`

### Cross-compilation
```bash
# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Build for Windows
cross build --release --target x86_64-pc-windows-msvc
```

## Testing

### Test Execution
```bash
# Run all tests
cargo test

# Run tests with coverage
cargo test -- --nocapture

# Run specific test
cargo test <test_name>

# Run tests for specific crate
cargo test -p <crate_name>
```

### Test Patterns
- Unit tests in `tests/` directories
- Integration tests in `tests/` directories
- Doctest examples in source files

### Test Coverage
- Coverage reports: `cargo llvm-cov --lcov --output-path coverage.lcov`
- Minimum coverage: 80% (enforced in CI)

## Code Style & Standards

### Formatting
```bash
# Format all code
cargo fmt

# Check formatting without applying
cargo fmt -- --check
```

### Linting
```bash
# Run clippy lints
cargo clippy

# Run clippy with warnings treated as errors
cargo clippy -- -D warnings
```

### Conventions
- Rust edition 2021
- Async/await pattern for all I/O operations
- Error handling with `anyhow::Result`
- Structured logging with tracing
- Type-safe configuration with serde

## Running the Application

### Start Command
```bash
# Development mode
cargo run

# Release mode
cargo run --release

# With specific configuration
FRIENDEV_CONFIG_PATH=/path/to/config.toml cargo run
```

### Environment Variables
- `FRIENDEV_CONFIG_PATH`: Path to configuration file
- `FRIENDEV_API_KEY`: API key for AI services
- `FRIENDEV_MODEL`: Default AI model
- `RUST_LOG`: Logging level (debug, info, warn, error)

### Configuration
Configuration file location: `~/.friendev/config.toml`
```toml
[api]
model = "gpt-3.5-turbo"
api_key = "your-api-key-here"
base_url = "https://api.openai.com/v1"

[ui]
language = "en"
theme = "dark"

[tools]
auto_approve = false
```

## API & Dependencies

### External Dependencies
- **tokio**: Async runtime
- **reqwest**: HTTP client
- **serde**: Serialization/deserialization
- **reedline**: Line editor
- **crossterm**: Terminal manipulation
- **anyhow**: Error handling
- **uuid**: UUID generation

### Version Constraints
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
reedline = "0.28"
crossterm = "0.27"
anyhow = "1.0"
uuid = { version = "1", features = ["v4"] }
```

### Workspace Dependencies
- All crates are part of a workspace
- Inter-crate dependencies use relative paths
- Version consistency enforced across all crates

## Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clear cargo cache
cargo clean

# Update dependencies
cargo update

# Check Rust toolchain
rustup update
rustup show
```

#### API Connection Issues
- Verify API key configuration
- Check network connectivity
- Validate API endpoint URLs
- Review rate limits and quotas

#### Terminal Issues
- Ensure terminal supports UTF-8
- Check terminal capabilities for reedline
- Verify terminal size for multi-line input

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Enable trace logging
RUST_LOG=trace cargo run
```

### Performance Issues
- Monitor memory usage with `cargo watch`
- Profile with `cargo flamegraph`
- Check for memory leaks in long-running sessions

## Contributing

### Git Workflow
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and commit
git commit -m "feat: add new feature"

# Push to remote
git push origin feature/your-feature-name

# Create pull request
gh pr create --title "feat: add new feature" --body "Description of changes"
```

### Commit Message Format
```
<type>(<scope>): <description>

[body]

[footer]
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build process or auxiliary tool changes

### PR Guidelines
- Include tests for new features
- Update documentation if needed
- Ensure all checks pass
- Review and approve changes
- Squash commits before merging

### Code Review
- All PRs require at least one approval
- Address all review comments
- Keep PRs focused and small
- Update changelog for significant changes