# AGENTS.md Generation System

## Your Task

Analyze the provided project structure and generate a comprehensive AGENTS.md file that follows the AGENTS.md open standard.

**CRITICAL**: You MUST use available file-editing tools (such as `file_write` or `file_replace`) to create or update the AGENTS.md file in the project root. Do NOT just print the content without writing it to the file.

---

## What is AGENTS.md?

AGENTS.md is a Markdown file placed in the project root directory that provides AI coding assistants with essential information about how to work with the project.

**Purpose**: Enable AI agents to:
- Understand project structure and architecture
- Execute correct build and test commands
- Follow established code conventions
- Navigate the codebase effectively
- Troubleshoot common issues

---

## Core Principles

### 1. Clarity
- Use specific commands and paths, not vague descriptions
- Provide exact command syntax that can be copy-pasted
- Include concrete examples, not abstract explanations
- Use absolute or relative paths consistently

**Good**: `cargo build --release --target x86_64-unknown-linux-gnu`
**Bad**: "build the project for Linux"

### 2. Completeness
- Include all information an AI agent needs to work effectively
- Cover the full development lifecycle (setup, build, test, run, deploy)
- Document all critical commands and workflows
- Explain project-specific conventions

### 3. Accuracy
- Specify exact version numbers: `Python 3.11+`, not "Python 3"
- Use correct command syntax for the project's tools
- Verify all commands actually work
- Keep information up-to-date

### 4. Brevity
- Include only what AI agents need, not user tutorials
- Avoid marketing language or project history
- Skip obvious information
- Focus on actionable content

---

## Recommended Sections

### 1. Overview (REQUIRED)
**What to include**:
- Project name and purpose (1-2 sentences)
- Primary programming language(s)
- Key technologies/frameworks used

**Example**:
```markdown
## Overview

Friendev is an AI-powered development assistant written in Rust. It provides an interactive REPL interface for AI-powered coding assistance, file management, and development workflows.
```

### 2. Dev Environment (REQUIRED)
**What to include**:
- Prerequisites with specific versions
- Setup steps with exact commands
- Environment variables needed
- Platform-specific requirements

**Example**:
```markdown
## Dev Environment

### Prerequisites
- Rust 1.70+ (stable)
- Cargo (Rust package manager)
- Git

### Setup Steps
\```bash
git clone <repository-url>
cd project-name
npm install
\```
```

### 3. Project Structure (REQUIRED)
**What to include**:
- Directory tree showing key folders
- Brief description of each major component
- Location of important files (config, entry points)

**Example**:
```markdown
## Project Structure

\```
project/
├── src/           # Source code
├── tests/         # Test files
├── docs/          # Documentation
└── config/        # Configuration files
\```

### Key Components:
- **src/**: Main application code
- **tests/**: Unit and integration tests
```

### 4. Build & Compilation (if applicable)
**What to include**:
- Build commands for different targets
- Output locations
- Build options and flags
- Cross-compilation instructions

### 5. Testing (REQUIRED)
**What to include**:
- Command to run all tests
- Command to run specific tests
- Test patterns and organization
- Coverage tools and commands

**Example**:
```markdown
## Testing

\```bash
# Run all tests
npm test

# Run specific test file
npm test -- tests/auth.test.js

# Run with coverage
npm test -- --coverage
\```
```

### 6. Code Style & Standards (REQUIRED)
**What to include**:
- Linting commands
- Formatting commands
- Naming conventions
- Code organization patterns

### 7. Running the Application (REQUIRED)
**What to include**:
- Start command
- Environment variables
- Configuration files
- Command-line options

### 8. API & Dependencies (REQUIRED)
**What to include**:
- External dependencies with versions
- Internal module structure
- Package manager commands

### 9. Troubleshooting (RECOMMENDED)
**What to include**:
- Common build errors and solutions
- Configuration issues
- Platform-specific problems
- Debug commands

### 10. Contributing (RECOMMENDED)
**What to include**:
- Git workflow
- Commit message format
- PR guidelines
- Code review process

---

## Writing Guidelines

### Commands
- ✅ **DO**: `pytest tests/ --cov`
- ❌ **DON'T**: "run the test suite"

- ✅ **DO**: `cargo build --release`
- ❌ **DON'T**: "build the project"

### Versions
- ✅ **DO**: `Node.js 18.x or higher`
- ❌ **DON'T**: "recent Node.js"

- ✅ **DO**: `Python 3.11+`
- ❌ **DON'T**: "Python 3"

### Paths
- ✅ **DO**: `src/main.rs` (relative from root)
- ❌ **DON'T**: "the main file"

- ✅ **DO**: `target/release/app`
- ❌ **DON'T**: "the output directory"

### Code Blocks
Always specify language:
```markdown
\```bash
npm install
\```

\```python
import sys
\```

\```rust
fn main() {}
\```
```

### Tone
- ✅ **DO**: "Run `cargo test` to execute tests"
- ❌ **DON'T**: "You can run tests using cargo"

- ✅ **DO**: "Requires Rust 1.70+"
- ❌ **DON'T**: "Make sure you have a recent version of Rust"

---

## What NOT to Include

### ❌ Project History
- Founding story
- Contributor lists
- Changelog details
- Version history

### ❌ User Tutorials
- "Getting started" guides for end users
- Feature walkthroughs
- UI/UX documentation
- Marketing content

### ❌ Vague Language
- "or" / "either" (be specific)
- "usually" / "typically" (be definitive)
- "might" / "could" (be clear)
- "best" / "leading" (avoid marketing)

### ❌ Obvious Information
- How to use Git basics
- How to install common tools
- General programming concepts

---

## Detection Heuristics

Use the project structure to infer:

### Language Detection
- `package.json` → Node.js/JavaScript
- `Cargo.toml` → Rust
- `requirements.txt` or `pyproject.toml` → Python
- `pom.xml` or `build.gradle` → Java
- `go.mod` → Go

### Framework Detection
- `next.config.js` → Next.js
- `vite.config.js` → Vite
- `Cargo.toml` with `tokio` → Async Rust
- `package.json` with `react` → React

### Build System Detection
- `Makefile` → Make
- `CMakeLists.txt` → CMake
- `build.gradle` → Gradle
- `Cargo.toml` → Cargo

### Test Framework Detection
- `jest.config.js` → Jest
- `pytest.ini` → pytest
- `Cargo.toml` with `#[test]` → Rust built-in tests

---

## Quality Checklist

Before writing the file, verify:

- [ ] All commands are copy-pastable and correct
- [ ] Version numbers are specific (not "latest" or "recent")
- [ ] Paths are consistent (all relative from root)
- [ ] Code blocks have language tags
- [ ] No marketing language or superlatives
- [ ] No vague instructions ("or", "either", "usually")
- [ ] Sections are relevant to the project type
- [ ] Information is actionable for AI agents
- [ ] File will be written using file-editing tools

---

## Output Format

Generate the AGENTS.md content following this structure:

```markdown
# AGENTS.md

## Overview
[1-2 sentence project description]

## Dev Environment
[Prerequisites and setup]

## Project Structure
[Directory tree and component descriptions]

## Build & Compilation
[Build commands and options]

## Testing
[Test commands and patterns]

## Code Style & Standards
[Linting and formatting]

## Running the Application
[Start commands and configuration]

## API & Dependencies
[Dependencies and versions]

## Troubleshooting
[Common issues and solutions]

## Contributing
[Git workflow and guidelines]
```

---

## Project Structure to Analyze

{project_structure}

---

## Action Required

1. Analyze the project structure above
2. Identify the programming language, frameworks, and tools
3. Generate a comprehensive AGENTS.md file following all guidelines
4. **Use `file_write` tool to create AGENTS.md in the project root**
5. Ensure all commands are accurate and testable

Remember: The goal is to enable AI agents to work effectively with this project. Focus on actionable, specific, accurate information.
