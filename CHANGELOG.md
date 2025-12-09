# Changelog
- all friendev update changelog on here.
---
## [0.2.6] - in dev
### Change
- **Welcome screen Change**: The original welcome screen, which prompted with '/model list', has been changed to '/model'.
- **System Notification**: Added a new notification system that sends a desktop notification and plays a sound alert when AI tasks are completed.
- **Enhanced `file_read` Tool**: Added support for reading specific line ranges.
  - New optional parameters: `start_line` and `end_line`.
  - Useful for reading large files in chunks to save tokens.
  - Displays `(Lines X-Y)` header when a range is selected.
- **Add LSP support for outline and index**(Other Branch)
  - Added LSP client implementation using async-lsp-client
  - Refactored file outline and indexer to prioritize Tree-sitter with LSP fallback
  - Added configuration support for custom LSP servers via lsp.json
  - Added --ts and --lsp flags to /index commands
  - Added LSP configuration documentation
- **Separate Shorekeeper model configuration**: Add the /model sk command to support setting up Shorekeeper models independently.
- **Added Jury mode (--jury)**: Introduces a three-model independent jury mechanism, requiring a 2/3 majority vote to execute sensitive operations.
- **Hooks Support**: Provides a hooks system that allows you to set up automated execution of custom scripts or commands at specific stages.
  - A built-in Lua interpreter was introduced.
  - See `docs\HOOKS.md` for details.
- **MCP**
  - Deep MCP Tool Integration  
    - Dynamic Tool Discovery: Friendev can now automatically discover and register all tools provided by connected MCP servers.  
    - Namespace Isolation: To prevent naming conflicts, MCP tools are automatically named in the format server/tool.  
    - AI-Native Support: AI assistants can now "see" and directly call these MCP tools to complete tasks without manual intervention.  
  - Resource Access System (Resources)  
    - Resource List: Added the mcp_resource_list tool to view all available resources.  
    - Resource Reading: Added the mcp_resource_read tool to support reading the content of resource URIs.  
  - Interactive Prompt System  
    - Command: Added the /prompt interactive command.  
    - Workflow: Implemented a 5-step fully automated workflow: Select Server → Select Prompt → Fill Input Parameters → Execute Prompt → Automatically Send Results to AI.  
  - Added the /mcp Command
- **Add todo management tools and command**
  - Implement `todo_write` and `todo_read` tools for AI task management
  - Add `/todo` command for users to view current task list
  - Support status visualization (pending, in_progress, completed)
- **/send.md**
  - The `/send.md` command has been added.
  - After using the `/send.md` command, the system will automatically read the send.md file in the project root directory and then send it to AI.
---
## [0.2.5] - 2025-12-02
### Added
- **Intelligent Code Outline**: Added `file_outline` tool powered by Tree-sitter to extract symbol definitions (functions, classes, etc.) from source files.
  - Supported languages: Rust, Python, JavaScript, TypeScript, Go, Java, C/C++, C#, PHP, Ruby.
- **Intelligent Code Indexing**: Introduced a SQLite-based code outline indexing system (`.friendev/index/outline.db`) for instant symbol search.
- **New Commands**: Added `/index outline` (incremental) and `/index outline all` (full) commands with progress bars and stats.
- **New Tools**:
  - `file_search_by_outline`: Query symbol definitions (functions, classes) instantly from the local index.
  - `index_file`: Manually update the index for specific files.
- **Automatic Indexing**: Integrated auto-hooks to update the index whenever files are modified via `file_write` `file_diff_edit` or `file_replace`.
- **Startup Diagnostics**: Added a smart check to warn users if the code index is stale (based on git commit history).
- **i18n Support**: Full internationalization for all new indexing features and search diagnostics (EN/CN).
### Fix
- Fixed the unexpected `1111` on the language
---
## [0.2.4] - 2025-12-01

### Bug Fixes
- Fixed an issue where the multi-line input box and already-printed content were incorrectly overwritten under certain conditions.

### New Features
- **File Search Capability**: Added `file_search` tool, enabling the AI to search across local files.
- **Search Query Visibility**: Enhanced the three network search components to display the actual search terms used, allowing users to see exactly what the AI is querying.(By @LikeEpieiKeia216 - #1)
---
## [0.2.3] - 2025-11-30

### New Features
- Added --shorekeeper mode: AI assesses, reviews, and approves "Approval Required" based on risk, replacing manual approval, making it safer than ally (yolo).
- **Alias Support**: Added `--yolo` as an alias for `--ally`
---
## [0.2.2] - 2025-11-30

### New Features
- **Prompt Optimization**: Added `!` prefix syntax to enable automatic prompt optimization.
- **Model Management**: Introduced the `/model` command, providing a visual interface for model switching.
- **Network Tool**: Integrated the `network_get_content` tool, allowing the AI to fetch web page content via URL.

### User Experience Improvements
- **Input System**: Replaced with the reedline library, supporting multi-line input and enhanced editing capabilities.
- **Interruption Mechanism**: Added the ESC key shortcut to interrupt AI output; improved Ctrl+C exit logic (now requires two consecutive presses).

### System Optimizations
- **Approval Mechanism**: Refactored the "Approval Required".
---
## [0.2.1] - 2025-11-22
- Roll back the codebase to the time before it was refactored into C/S (Only Client), as there are many issues
---
## [0.2.0] - 2025-11-22
- Refactoring for C/S architecture
- Split into multiple crates, becoming a multi-crate workspace
---
## [0.1.5] - 2025-11-20
#### Added
- Internationalization (i18n) support introduced for all UI components
- Optimized the prompt for generating AGENTS.md
---
## [0.1.4] - 2025-11-15

### Added
- **New `file_diff_edit` tool**: Enables precise file editing using diff-style hunks for batch line-level modifications
- **New `run_command` tool**: Grants LLM the ability to execute system commands (by Friendev)
- **New `--ally` parameter**: Automatically approves all "Approval Required" prompts
- **Enhanced setup process**: Added i18n support and language configuration
- **New `--setup` parameter**: Forces initial setup workflow when specified

### Changed
- **Updated AGENTS.md integration**: Modified context inclusion positioning
- **Improved language handling**: Updated language IDs and restricted UI language input
- **Optimized startup sequence**: Adjusted code execution order

### Enhanced
- **Automatic cleanup**: Removes isolated tool calls without corresponding responses
- **Session management**: Automatically deletes history files with zero messages

### Technical
- **Code refactoring**: Restructured core logic for improved maintainability
- **Workflow improvements**: Streamlined various operational processes
---
## [0.1.3] - 2025-11-14

### Added
- Added the `/agents.md` command to allow the AI to generate an `AGENTS.md` file.
- Implemented automatic context integration for `AGENTS.md`: its content is now automatically included in the conversation context.

### Enhanced
- **Expanded Approval Required functionality**: Added an `[i]nfo`, You can view the code generated by AI. (By Friendev)
- **Improved the `file_replace` tool**:
  - Added `normalize` (fuzzy matching) and `regex` (regular expression) parameters for greater flexibility;
  - Standardized line endings across files to ensure consistent cross-platform handling;
  - Addressed various edge cases to improve robustness.

### Fixed
- Resolved issues related to invalid tool invocations and parameter handling:
  - Strengthened validation to skip malformed tool calls and invalid JSON parameters;
  - Fixed errors caused by missing `tool_response` messages for `tool_call_id`s (e.g., `invalid_parameter_error`: "An assistant message with 'tool_calls' must be followed by tool messages..."). (By Friendev)

### UX Improvements
- The `/history list` command now automatically filters out sessions with zero messages. (By Friendev)
- On startup, the system now automatically deletes any history files containing zero messages, keeping the session list clean. (By Friendev)
---
## [0.1.2] - 2025-11-13
### Added
- **New internet search capabilities** with three search tools:
- `network_search_auto`: Automatically selects between search engines (defaults to DuckDuckGo, falls back to Bing)
- `network_search_duckduckgo`: Direct DuckDuckGo search integration
- `network_search_bing`: Direct Bing search integration
### Enhanced
- **Implemented timeout and retry mechanism** for AI requests to improve reliability
- **Refactored JSON streaming logic** with Server-Sent Events (SSE) implementation
- **Codebase improvements** through partial refactoring of core functionality
---
## [0.1.1] - 2025-11-09

### Added
- **New `file_replace` tool**: Enables AI to perform partial content replacements within files.

### Changed
- **Rewrote system prompts** from Chinese to English for improved international compatibility.

### Improved
- **Optimized JSON validation** and string processing logic for enhanced reliability.
