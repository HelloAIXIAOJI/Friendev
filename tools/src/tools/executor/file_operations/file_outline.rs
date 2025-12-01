use anyhow::Result;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};

use super::file_common::normalize_path;
use crate::tools::args::FileOutlineArgs;
use crate::types::ToolResult;

enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    Cpp,
    C,
    CSharp,
    Php,
    Ruby,
}

impl Language {
    fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            "js" | "jsx" | "mjs" | "cjs" => Some(Language::JavaScript),
            "ts" | "tsx" | "mts" | "cts" => Some(Language::TypeScript),
            "go" => Some(Language::Go),
            "java" => Some(Language::Java),
            "cpp" | "cc" | "cxx" | "hpp" | "h" => Some(Language::Cpp),
            "c" => Some(Language::C),
            "cs" => Some(Language::CSharp),
            "php" => Some(Language::Php),
            "rb" => Some(Language::Ruby),
            _ => None,
        }
    }

    fn get_grammar(&self) -> tree_sitter::Language {
        match self {
            Language::Rust => tree_sitter_rust::language(),
            Language::Python => tree_sitter_python::language(),
            Language::JavaScript => tree_sitter_javascript::language(),
            Language::TypeScript => tree_sitter_typescript::language_typescript(),
            Language::Go => tree_sitter_go::language(),
            Language::Java => tree_sitter_java::language(),
            Language::Cpp => tree_sitter_cpp::language(),
            Language::C => tree_sitter_c::language(),
            Language::CSharp => tree_sitter_c_sharp::language(),
            Language::Php => tree_sitter_php::language(),
            Language::Ruby => tree_sitter_ruby::language(),
        }
    }

    fn get_query_source(&self) -> &'static str {
        match self {
            Language::Rust => r#"
                (function_item name: (identifier) @name)
                (struct_item name: (type_identifier) @name)
                (enum_item name: (type_identifier) @name)
                (trait_item name: (type_identifier) @name)
                (impl_item type: (type_identifier) @name)
                (mod_item name: (identifier) @name)
                (macro_definition name: (identifier) @name)
            "#,
            Language::Python => r#"
                (function_definition name: (identifier) @name)
                (class_definition name: (identifier) @name)
            "#,
            Language::JavaScript => r#"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (identifier) @name)
                (method_definition name: (property_identifier) @name)
                (variable_declarator name: (identifier) @name value: [(arrow_function) (function)])
            "#,
            Language::TypeScript => r#"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (type_identifier) @name)
                (interface_declaration name: (type_identifier) @name)
                (enum_declaration name: (identifier) @name)
                (method_definition name: (property_identifier) @name)
                (variable_declarator name: (identifier) @name value: [(arrow_function) (function)])
            "#,
            Language::Go => r#"
                (function_declaration name: (identifier) @name)
                (method_declaration name: (field_identifier) @name)
                (type_declaration (type_spec name: (type_identifier) @name))
            "#,
            Language::Java => r#"
                (class_declaration name: (identifier) @name)
                (interface_declaration name: (identifier) @name)
                (enum_declaration name: (identifier) @name)
                (method_declaration name: (identifier) @name)
                (constructor_declaration name: (identifier) @name)
            "#,
            Language::Cpp => r#"
                (function_definition declarator: (function_declarator declarator: (identifier) @name))
                (class_specifier name: (type_identifier) @name)
                (struct_specifier name: (type_identifier) @name)
                (namespace_definition name: (identifier) @name)
            "#,
            Language::C => r#"
                (function_definition declarator: (function_declarator declarator: (identifier) @name))
                (struct_specifier name: (type_identifier) @name)
                (enum_specifier name: (type_identifier) @name)
                (typedef_declaration declarator: (type_identifier) @name)
            "#,
            Language::CSharp => r#"
                (class_declaration name: (identifier) @name)
                (interface_declaration name: (identifier) @name)
                (struct_declaration name: (identifier) @name)
                (enum_declaration name: (identifier) @name)
                (method_declaration name: (identifier) @name)
                (constructor_declaration name: (identifier) @name)
                (namespace_declaration name: (identifier) @name)
            "#,
            Language::Php => r#"
                (function_definition name: (name) @name)
                (class_declaration name: (name) @name)
                (interface_declaration name: (name) @name)
                (trait_declaration name: (name) @name)
                (method_declaration name: (name) @name)
            "#,
            Language::Ruby => r#"
                (method name: (identifier) @name)
                (class name: (constant) @name)
                (module name: (constant) @name)
            "#,
        }
    }
}

pub async fn execute_file_outline(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileOutlineArgs = serde_json::from_str(arguments)?;
    let path = normalize_path(&args.path, working_dir);

    if !path.exists() {
        return Ok(ToolResult::error(format!("File not found: {}", path.display())));
    }

    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let lang = match Language::from_extension(ext) {
        Some(l) => l,
        None => return Ok(ToolResult::error(format!("Unsupported language for file extension: .{}", ext))),
    };

    let source_code = fs::read_to_string(&path)?;
    let mut parser = Parser::new();
    // Note: In older tree-sitter versions (0.20.x), set_language takes 'Language', not '&Language' in some bindings,
    // but the error message said "expected `&Language`, found `Language`" for 0.22.
    // Since we downgraded to 0.20.10, let's check the API.
    // Usually 0.20.10 takes `Language` (by value) because Language is a Copy/Clone wrapper around a pointer.
    // BUT if the error persists, we might need to adjust.
    // The error message from user was based on 0.22.
    // With 0.20.10, let's try passing by value first (as it was).
    // Wait, I am reverting the change I WAS GOING TO MAKE because I downgraded.
    // Let's stick to the original code for now, assuming 0.20.10 uses by-value.
    // IF IT FAILS again, I will add &.
    
    // Actually, let's just make it compile. The error said "expected &Language", implying the method signature wanted a reference.
    // That was for 0.22.
    // For 0.20.10, set_language signature is `fn set_language(&mut self, language: Language)`.
    // So passing by value `lang.get_grammar()` is CORRECT for 0.20.10.
    // So I don't need to add `&`.
    
    parser.set_language(lang.get_grammar())?;

    let tree = parser.parse(&source_code, None)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

    let query = Query::new(lang.get_grammar(), lang.get_query_source())?;
    let mut cursor = QueryCursor::new();
    
    let mut results = String::new();
    results.push_str(&format!("Outline for {}:\n", path.display()));
    results.push_str("---\n");

    let mut matches: Vec<(usize, String, String)> = Vec::new();

    for m in cursor.matches(&query, tree.root_node(), source_code.as_bytes()) {
        for capture in m.captures {
            let node = capture.node;
            let start_line = node.start_position().row + 1;
            let text = match node.utf8_text(source_code.as_bytes()) {
                Ok(t) => t.to_string(),
                Err(_) => continue,
            };
            
            // Try to get the type of the definition from the parent node if possible
            let kind = node.parent().map(|p| p.kind()).unwrap_or("unknown");
            
            matches.push((start_line, kind.to_string(), text));
        }
    }

    // Sort by line number
    matches.sort_by_key(|k| k.0);
    matches.dedup(); // Remove duplicates if any

    if matches.is_empty() {
        return Ok(ToolResult::ok(
            format!("No symbols found in {}", path.display()),
            "No symbols found.".to_string()
        ));
    }

    for (line, kind, name) in matches {
        // Clean up kind names (e.g., "function_item" -> "Function")
        let pretty_kind = kind.replace("_item", "")
            .replace("_definition", "")
            .replace("_declaration", "")
            .replace("impl", "Impl");
            
        results.push_str(&format!("L{:<4} [{}] {}\n", line, pretty_kind, name));
    }

    let brief = format!("Found {} symbols in {}", results.lines().count() - 2, path.display());
    Ok(ToolResult::ok(brief, results))
}
