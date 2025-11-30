# AGENTS.md

## Overview

Friendev is an AI-powered development assistant written in Rust. It provides an interactive REPL interface for AI-powered coding assistance, file management, and development workflows. The project is structured as a multi-crate workspace with modules for API integration, UI, commands, configuration, and various development tools.

## Dev Environment

### Prerequisites
- Rust 1.70+ (stable)
- Cargo (Rust package manager)
- Git
- Cross-compilation tools (for building on multiple platforms)

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

### Version Information
- **Rust**: Edition 2021
- **Cargo**: Latest stable version
- **Target**: Multi-platform support (Windows, Linux, macOS, Android, FreeBSD, etc.)

## Project Structure

```
friendev/
├── agents_md_file/     # AGENTS.md generation and management
├── api/               # API client and network communication
├── app/               # Main application logic and REPL
├── chat/              # Chat session management
├── commands/          # Command implementations
├── config/            # Configuration management
├── history/           # Chat history persistence
├── i18n/              # Internationalization support
├── prompts/           # Prompt templates and optimization
├── search_tool/       # Web search functionality
├── security/          # Security and approval mechanisms
├── src/               # Root source (main entry point)
├── target/            # Build artifacts
├── tools/             # Development tools and utilities
└── ui/                # User interface components
```

### Key Components

- **app/**: Core application logic containing REPL loop, startup sequence, and terminal UI
- **config/**: Configuration management with persistence and interactive setup
- **api/**: HTTP client for AI API communication with streaming support
- **commands/**: Built-in commands like `/model`, `/history`, `/agents.md`
- **ui/**: Terminal UI components with colored output and progress indicators
- **tools/**: File editing, network access, and system command execution capabilities

## Build & Compilation

### Build Commands
```bash
# Build for current platform
cargo build --release

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Cross-compilation (requires cross tool)
cargo install cross --locked
cross build --release --target x86_64-unknown-linux-musl
```

### Output Locations
- **Default**: `target/release/friendev` (Linux/macOS) or `target/release/friendev.exe` (Windows)
- **Cross-compilation**: `target/<target>/release/friendev`

### Multi-Platform Support
The project supports compilation for multiple platforms including:
- Windows (x86_64, i686, aarch64)
- Linux (x86_64, i686, aarch64, armv7, riscv64, powerpc64le, s390x)
- macOS (x86_64, aarch64)
- Android (aarch64, armv7, x86_64, i686)
- FreeBSD (x86_64, i686)

## Testing

### Test Execution
```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --verbose

# Run tests for specific crate
cargo test -p app
```

### Test Patterns
- Unit tests for individual modules
- Integration tests for API communication
- UI component tests using crossterm
- Configuration and persistence tests

### Coverage
Test coverage is maintained for critical components:
- Configuration management
- API client functionality
- File editing operations
- Command parsing and execution

## Code Style & Standards

### Linting Rules
```bash
# Check code style
cargo fmt -- --check

# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings
```

### Formatting Conventions
- Rust standard formatting with `cargo fmt`
- Clippy warnings treated as errors (`-D warnings`)
- Consistent error handling with `anyhow::Result`
- Async/await patterns using `tokio`

### Naming Conventions
- Snake_case for functions and variables
- PascalCase for types and structs
- SCREAMING_SNAKE_CASE for constants
- Clear, descriptive names for public APIs

## Running the Application

### Start Command
```bash
# Basic startup
cargo run --release

# With auto-approval (bypasses approval prompts)
cargo run --release -- --ally

# With smart approval mode (AI reviews approval prompts)
cargo run --release -- --shorekeeper

# Force setup configuration
cargo run --release -- --setup
```

### Environment Variables
- `RUST_BACKTRACE=1`: Enable detailed backtraces
- `RUST_LOG=debug`: Enable debug logging
- `CARGO_TERM_COLOR=always`: Force colored output

### Configuration
Configuration is stored in `~/.friendev/config.json` and includes:
- AI model settings
- UI language preferences
- API endpoint configuration
- Session management settings

### Command Line Options
- `--ally` / `--yolo`: Auto-approve all prompts
- `--shorekeeper`: Smart approval mode (AI reviews prompts)
- `--setup`: Force interactive setup
- `--help`: Show help information

## API & Dependencies

### External Dependencies
| Dependency | Version | Purpose |
|------------|---------|---------|
| tokio | 1.x | Async runtime |
| reqwest | 0.11 | HTTP client |
| serde | 1.0 | JSON serialization |
| crossterm | 0.27 | Terminal UI |
| reedline | 0.28 | Line editor |
| anyhow | 1.0 | Error handling |
| uuid | 1.0 | UUID generation |

### Internal Dependencies
The project uses a workspace structure with internal dependencies:
- `agents` → AGENTS.md management
- `app` → Main application logic
- `api` → HTTP client
- `config` → Configuration management
- `ui` → Terminal UI components
- `commands` → Built-in commands
- `chat` → Session management
- `history` → History persistence
- `i18n` → Internationalization
- `prompts` → Prompt templates
- `security` → Security features
- `tools` → Development utilities

### Version Constraints
All internal dependencies use path dependencies with version constraints:
```toml
[dependencies]
api = { path = "../api", version = "0.1.0" }
config = { path = "../config", version = "0.1.0" }
# ... etc
```

## Troubleshooting

### Common Issues

**Build Failures**
```bash
# Update Rust toolchain
rustup update stable

# Clean build artifacts
cargo clean

# Check for missing dependencies
cargo check --all
```

**Configuration Issues**
```bash
# Reset configuration
rm -rf ~/.friendev/
cargo run --release -- --setup
```

**API Connection Problems**
- Check network connectivity
- Verify API endpoint configuration
- Ensure proper authentication setup
- Check firewall/proxy settings

**Terminal UI Issues**
- Ensure terminal supports ANSI colors
- Check terminal size and encoding
- Verify crossterm compatibility

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug cargo run --release

# Enable backtraces
RUST_BACKTRACE=1 cargo run --release
```

### Error Codes
- `Error::ConfigLoad`: Configuration file corruption
- `Error::ApiConnection`: Network/API issues
- `Error::PermissionDenied`: File system permission issues
- `Error::InvalidInput`: User input validation failure

## Contributing

### Git Workflow
```bash
# Feature branch workflow
git checkout -b feature/new-feature
git commit -m "feat: add new feature"
git push origin feature/new-feature

# Pull request process
git checkout main
git pull origin main
git checkout -b feature/bugfix
# ... make changes ...
git commit -m "fix: resolve issue #123"
git push origin feature/bugfix
```

### Commit Message Format
Conventional Commits standard:
```
<type>(<scope>): <description>

[body]

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code formatting
- `refactor`: Code refactoring
- `test`: Test additions/modifications
- `chore`: Build process or auxiliary tool changes

### Pull Request Guidelines
1. Include tests for new functionality
2. Update documentation as needed
3. Ensure all checks pass
4. Follow existing code style
5. Provide clear description of changes
6. Link to relevant issues

### Development Workflow
1. Create feature branch from `main`
2. Make changes and write tests
3. Run `cargo test` and `cargo clippy`
4. Commit changes with conventional commit message
5. Push branch and create pull request
6. Address review comments
7. Merge after approval