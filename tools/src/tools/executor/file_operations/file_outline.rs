use anyhow::Result;
use std::path::Path;
use config::Config; // Changed from load_config as it is a static method on Config struct usually or we use Config::load()

use super::super::parser;
use super::super::lsp_client::LspClient;
use super::file_common::normalize_path;
use crate::tools::args::FileOutlineArgs;
use crate::types::ToolResult;

fn get_lsp_command(ext: &str) -> Option<(String, Vec<String>)> {
    // Load config to check for custom LSP servers
    if let Ok(Some(lsp_settings)) = Config::load_lsp() {
         if let Some(lsp_config) = lsp_settings.servers.get(ext) {
             return Some((lsp_config.command.clone(), lsp_config.args.clone()));
         }
    }

    // Fallback to defaults
    match ext {
        "rs" => Some(("rust-analyzer".to_string(), vec![])),
        "py" => Some(("pylsp".to_string(), vec![])),
        "go" => Some(("gopls".to_string(), vec![])),
        "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs" => 
            Some(("typescript-language-server".to_string(), vec!["--stdio".to_string()])),
        "c" | "cpp" | "h" | "hpp" | "cc" => Some(("clangd".to_string(), vec!["--stdio".to_string()])),
        _ => None,
    }
}

fn flatten_symbols(symbols: Vec<lsp_types::DocumentSymbol>) -> Vec<parser::Symbol> {
    let mut result = Vec::new();
    for sym in symbols {
        #[allow(deprecated)]
        let line = sym.range.start.line as usize + 1;
        let kind = format!("{:?}", sym.kind);
        result.push(parser::Symbol {
            name: sym.name,
            kind,
            line,
            content: String::new(),
        });
        if let Some(children) = sym.children {
             result.extend(flatten_symbols(children));
        }
    }
    result
}

async fn try_lsp_outline(path: &Path) -> Result<Vec<parser::Symbol>> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let (cmd, args) = get_lsp_command(ext).ok_or_else(|| anyhow::anyhow!("No LSP for extension"))?;
    
    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let mut client = LspClient::new(&cmd, &args_refs).await?;
    
    // Use parent directory as root
    let root = path.parent().unwrap_or(path);
    
    client.initialize(root).await?;
    let symbols = client.document_symbol(path).await?;
    client.shutdown().await?;
    
    let mut flat_symbols = flatten_symbols(symbols);
    
    // Sort by line number
    flat_symbols.sort_by_key(|k| k.line);
    
    Ok(flat_symbols)
}

pub enum SymbolSource {
    TreeSitter,
    Lsp,
}

pub async fn get_symbols_with_source(path: &Path, use_tree_sitter: bool, use_lsp: bool) -> Result<(Vec<parser::Symbol>, Option<SymbolSource>)> {
    // Default behavior (if neither flag is set): Try Tree-sitter, then LSP
    let (try_ts, try_lsp) = if !use_tree_sitter && !use_lsp {
        (true, true)
    } else {
        (use_tree_sitter, use_lsp)
    };

    // 1. Try Tree-sitter first (if enabled)
    if try_ts {
        if let Ok(symbols) = parser::extract_symbols(path) {
            if !symbols.is_empty() {
                return Ok((symbols, Some(SymbolSource::TreeSitter)));
            }
        }
    }

    // 2. Try LSP (if enabled)
    if try_lsp {
        let lsp_result = try_lsp_outline(path).await;
        match lsp_result {
            Ok(symbols) => {
                if !symbols.is_empty() {
                     return Ok((symbols, Some(SymbolSource::Lsp)));
                }
            },
            Err(e) => {
                if !try_ts {
                    return Err(e);
                }
            }
        }
    }
    
    if !try_ts && !try_lsp {
        return Err(anyhow::anyhow!("Both Tree-sitter and LSP are disabled."));
    }
    
    if try_ts {
         parser::extract_symbols(path).map(|s| (s, None)) // None because it's fallback/empty
    } else {
         Ok((vec![], None))
    }
}

pub async fn get_symbols(path: &Path, use_tree_sitter: bool, use_lsp: bool) -> Result<Vec<parser::Symbol>> {
    get_symbols_with_source(path, use_tree_sitter, use_lsp).await.map(|(s, _)| s)
}

pub async fn execute_file_outline(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileOutlineArgs = serde_json::from_str(arguments)?;
    let path = normalize_path(&args.path, working_dir);

    match get_symbols_with_source(&path, args.use_tree_sitter, args.use_lsp).await {
        Ok((symbols, source)) => {
            if symbols.is_empty() {
                return Ok(ToolResult::ok(
                    format!("No symbols found in {}", path.display()),
                    "No symbols found.".to_string()
                ));
            }

            let source_msg = match source {
                Some(SymbolSource::TreeSitter) => " (via Tree-sitter)",
                Some(SymbolSource::Lsp) => " (via LSP)",
                None => "",
            };

            let mut results = String::new();
            results.push_str(&format!("Outline for {}{}:\n", path.display(), source_msg));
            results.push_str("---\n");

            for symbol in &symbols {
                results.push_str(&format!("L{:<4} [{}] {}\n", symbol.line, symbol.kind, symbol.name));
            }

            let brief = format!("Found {} symbols in {}", symbols.len(), path.display());
            Ok(ToolResult::ok(brief, results))
        }
        Err(e) => Ok(ToolResult::error(format!("Failed to extract symbols: {}", e))),
    }
}
