# LSP 配置指南 (LSP Configuration Guide)

Friendev 支持通过 Language Server Protocol (LSP) 和 Tree-sitter 来提供代码大纲 (Outline) 和索引 (Index) 功能。

## 默认行为

Friendev 采用混合解析策略来确保最佳性能和兼容性：

1.  **优先使用 Tree-sitter**：Tree-sitter 是一个高性能的增量解析器，速度极快且不需要外部进程。对于大多数支持的语言，系统会首选 Tree-sitter。
2.  **回退到 LSP**：如果 Tree-sitter 解析失败、返回空结果，或者当前语言没有内置的 Tree-sitter 支持，系统会自动尝试启动配置的 LSP 服务器进行解析。

## 控制解析策略

在使用 `/index` 或 `/outline` 命令时，您可以通过参数手动控制解析行为：

- **默认** (不加参数): 先尝试 Tree-sitter，后备 LSP。
- **`--ts`**: 强制**只使用** Tree-sitter。如果 Tree-sitter 不支持或解析失败，则不尝试 LSP。
- **`--lsp`**: 强制**只使用** LSP。跳过 Tree-sitter 解析。

例如：
```bash
/index outline all --ts   # 仅使用 Tree-sitter 重建索引
/index outline all --lsp  # 仅使用 LSP 重建索引
```

## 配置文件位置

LSP 配置文件名为 `lsp.json`，位于 Friendev 的配置目录中：

- **Windows**: `C:\Users\<用户名>\AppData\Roaming\friendev\lsp.json`
- **Linux/macOS**: `~/.config/friendev/lsp.json`

如果文件不存在，您可以手动创建它。

## 配置格式

`lsp.json` 是一个标准的 JSON 文件，包含一个 `servers` 对象。`servers` 对象的键是文件扩展名（不带点），值是 LSP 服务器的启动命令配置。

### 示例

```json
{
  "servers": {
    "rs": {
      "command": "rust-analyzer",
      "args": []
    },
    "py": {
      "command": "pylsp",
      "args": []
    },
    "js": {
      "command": "typescript-language-server",
      "args": ["--stdio"]
    },
    "my_lang": {
      "command": "my-language-server",
      "args": ["--stdio"]
    }
  }
}
```

### 字段说明

- **Key (例如 "rs", "py")**: 匹配的文件扩展名。当 Friendev 处理该后缀的文件时，会使用对应的配置。
- **command**: 启动 LSP 服务器的可执行文件名称或绝对路径。该程序必须在系统的 PATH 环境变量中，或者提供完整路径。
- **args**: 传递给 LSP 服务器的命令行参数数组。通常需要指定 `--stdio` 以启用标准输入输出通信模式（这是 Friendev 目前支持的通信方式）。

## 内置支持 (默认 LSP)

如果您没有配置 `lsp.json`，Friendev 为以下语言提供了默认的 LSP 映射（仅在 Tree-sitter 回退时使用）：

| 语言 | 扩展名 | 默认命令 | 参数 |
|------|--------|----------|------|
| Rust | `rs` | `rust-analyzer` | `[]` |
| Python | `py` | `pylsp` | `[]` |
| Go | `go` | `gopls` | `[]` |
| JS/TS | `js`, `ts`, `jsx`, `tsx`, `mjs`, `cjs` | `typescript-language-server` | `["--stdio"]` |
| C/C++ | `c`, `cpp`, `h`, `hpp`, `cc` | `clangd` | `["--stdio"]` |

## 常见问题

1.  **索引时显示 [use tree-sitter] 是什么意思？**
    表示该文件成功使用内置的 Tree-sitter 解析器提取了符号信息，没有启动 LSP。这是预期的默认高效行为。

2.  **索引时显示 [use LSP] 是什么意思？**
    表示 Tree-sitter 解析未返回结果（或被禁用），系统成功回退并使用了 LSP 服务器解析了文件。

3.  **如何添加对新语言的支持？**
    只需在 `lsp.json` 中添加该语言扩展名和对应的 LSP Server 启动命令即可。无需修改程序代码。

## 安装 LSP Server

要使用 LSP 功能，您需要在系统中安装对应的 Language Server。例如：

- **Rust**: `rustup component add rust-analyzer`
- **Python**: `pip install python-lsp-server`
- **JS/TS**: `npm install -g typescript-language-server typescript`
- **Go**: `go install golang.org/x/tools/gopls@latest`
- **C/C++**: 安装 LLVM/Clang 工具链 (包含 `clangd`)
