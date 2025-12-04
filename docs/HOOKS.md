# Friendev Hooks System

Friendev 提供了一个强大的钩子（Hooks）系统，允许您在开发工作流的关键时刻自动执行自定义脚本或命令。这非常适合用于环境检查、审计日志、自动 lint、安全扫描等任务。

## 配置

配置文件位于项目根目录下的 `.friendev/hooks.json`。

如果该文件不存在，Hooks 系统将不会激活。

## 钩子类型 (Hook Types)

支持以下触发时机：

- **`startup`**: Friendev 启动时触发。
- **`pre_command`**: 用户输入命令（如 `/model`）或执行任务前触发。
- **`post_command`**: 命令执行完成后触发。
- **`pre_approval`**: 在请求用户（或 AI Shorekeeper/Jury）审批操作（如文件编辑、命令运行）**之前**触发。
- **`post_approval`**: 在审批完成并获得结果（批准或拒绝）**之后**触发。
- **`session_start`**: 新聊天会话开始时。
- **`session_end`**: 会话结束时。

## 配置语法

Hooks 配置支持类似 GitHub Actions 的灵活语法，您可以混合使用简单的 Shell 命令和详细的配置对象。

### 1. 简单模式 (Shell 命令)

最简单的形式是直接提供命令字符串列表。

```json
{
  "hooks": {
    "startup": [
      "echo 'Welcome to Friendev!'",
      "python --version"
    ]
  }
}
```

### 2. 结构化模式 (高级配置)

对于更复杂的场景，可以使用对象来定义步骤。支持 Shell 命令、Lua 脚本以及 Friendev 内部命令。

```json
{
  "hooks": {
    "pre_command": [
      // 1. Shell 步骤
      {
        "name": "Check Git Status",
        "run": "git status --porcelain",
        "continue_on_error": true
      },
      
      // 2. 内联 Lua 脚本
      {
        "name": "Security Audit",
        "lua": "if env.FRIENDEV_COMMAND:match('rm %-rf') then print('!!! DANGER: Deletion detected !!!') end"
      },

      // 3. 引用外部 Lua 文件
      {
        "name": "Run Compliance Check",
        "uses": "scripts/compliance.lua",
        "env": { "LEVEL": "strict" }
      },

      // 4. 执行 Friendev 内部 Slash 命令
      {
        "name": "Update Code Index",
        "command": "/index outline all",
        "continue_on_error": true
      }
    ]
  }
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | String (可选) | 步骤名称，执行时会显示。 |
| `run` | String (可选) | 要执行的 Shell 命令。 |
| `command` | String (可选) | 要执行的 Friendev Slash 命令 (如 `/index outline all`)。**仅限 `pre_command、post_command、startup` 钩子。** |
| `shell` | String (可选) | 显式指定 Shell (如 `powershell`, `bash`, `cmd`)。 |
| `lua` | String (可选) | 要执行的内联 Lua 代码。 |
| `uses` | String (可选) | 引用外部脚本文件路径（支持 `.lua`）。 |
| `env` | Map (可选) | 该步骤专属的环境变量。 |
| `continue_on_error` | Bool (默认 true) | 如果步骤失败，是否继续执行后续步骤。 |

## 支持的操作类型

您可以在一个 Hook 步骤中选择以下**一种**操作方式：

1.  **Shell 命令 (`run`)**:
    -   调用系统 Shell 执行外部命令。
    -   示例: `"run": "npm run lint"`

2.  **Lua 脚本 (`lua` / `uses`)**:
    -   使用内嵌的 Lua 引擎执行脚本，跨平台兼容性好。
    -   支持访问 Friendev 上下文变量。
    -   示例: `"uses": "hooks/check.lua"`

3.  **Friendev 命令 (`command`)**:
    -   直接调用 Friendev 的内部功能。
    -   支持的钩子类型：`startup`, `pre_command`, `post_command`。
    -   示例: `"command": "/index refresh"`

## Lua 脚本支持

Friendev 内嵌了 Lua 5.4 引擎 (基于 `mlua`)，允许您编写跨平台的钩子逻辑，无需依赖系统 Shell。

### Lua 环境 API

在 Lua 脚本中，您可以访问以下全局变量：

- **`env`**: (Table) 包含当前上下文的所有环境变量。
  - `env.FRIENDEV_COMMAND`: 当前正在处理的用户命令。
  - `env.FRIENDEV_ACTION`: 待审批的操作类型 (如 `file_write`)。
  - `env.FRIENDEV_SUBJECT`: 操作目标 (如文件名)。
  - `env.FRIENDEV_APPROVED`: 审批结果 ("true" 或 "false")。
  - *以及所有标准的系统环境变量*。

- **`working_dir`**: (String) 当前工作目录的绝对路径。

- **`print(msg)`**: (Function) 将消息输出到 Friendev 终端。

### 示例：审计日志脚本

```lua
-- scripts/audit.lua
local file = io.open(".friendev/audit.log", "a")
if file then
    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
    file:write(string.format("[%s] Command: %s\n", timestamp, env.FRIENDEV_COMMAND or "N/A"))
    file:close()
    print("Audit log updated.")
end
```

## 环境变量

Hooks 执行时会自动注入以下上下文变量：

| 变量名 | 适用 Hook | 说明 |
|--------|-----------|------|
| `FRIENDEV_COMMAND` | `pre_command`, `post_command` | 用户输入的完整命令行。 |
| `FRIENDEV_ACTION` | `pre_approval`, `post_approval` | 触发审批的工具动作 (如 `run_command`, `file_write`)。 |
| `FRIENDEV_SUBJECT` | `pre_approval`, `post_approval` | 动作的目标 (如文件路径或命令内容)。 |
| `FRIENDEV_APPROVED` | `post_approval` | 审批结果 (`true` 或 `false`)。 |
| `FRIENDEV_BG` | `post_command` | 命令是否在后台运行 (`true` 或 `false`)。 |

## 最佳实践

1. **轻量级**: 保持 Hooks 脚本执行迅速，避免阻塞主线程太久。
2. **错误处理**: 对于非关键步骤，设置 `"continue_on_error": true`。
3. **跨平台**: 尽量使用 Lua 脚本代替 Shell 命令，以确保在 Windows 和 Linux 上都能正常工作。
4. **路径引用**: `uses` 路径可以是相对于项目根目录的，也可以是相对于 `.friendev/` 目录的。建议将脚本放在 `.friendev/hooks/` 下以保持整洁。
