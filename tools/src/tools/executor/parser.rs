use anyhow::Result;
use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: String,
    pub line: usize,
    pub content: String, // The full line content or signature
}

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

pub fn extract_symbols(path: &Path) -> Result<Vec<Symbol>> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", path.display()));
    }

    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let lang = match Language::from_extension(ext) {
        Some(l) => l,
        None => return Err(anyhow::anyhow!("Unsupported language for file extension: .{}", ext)),
    };

    let source_code = fs::read_to_string(path)?;
    let mut parser = Parser::new();
    parser.set_language(lang.get_grammar())?;

    let tree = parser.parse(&source_code, None)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

    let query = Query::new(lang.get_grammar(), lang.get_query_source())?;
    let mut cursor = QueryCursor::new();
    
    let mut symbols = Vec::new();

    for m in cursor.matches(&query, tree.root_node(), source_code.as_bytes()) {
        for capture in m.captures {
            let node = capture.node;
            let start_line = node.start_position().row + 1;
            let name = match node.utf8_text(source_code.as_bytes()) {
                Ok(t) => t.to_string(),
                Err(_) => continue,
            };
            
            // Try to get the type of the definition from the parent node
            let kind = node.parent().map(|p| p.kind()).unwrap_or("unknown").to_string();
            
            // Get the full line content for context
            let line_content = source_code.lines().nth(start_line - 1).unwrap_or("").trim().to_string();

            // Clean up kind names
            let pretty_kind = kind.replace("_item", "")
                .replace("_definition", "")
                .replace("_declaration", "")
                .replace("impl", "Impl");

            symbols.push(Symbol {
                name,
                kind: pretty_kind,
                line: start_line,
                content: line_content,
            });
        }
    }

    // Sort by line number and deduplicate
    symbols.sort_by_key(|k| k.line);
    symbols.dedup_by(|a, b| a.line == b.line && a.name == b.name);

    Ok(symbols)
}

pub fn is_supported_extension(ext: &str) -> bool {
    Language::from_extension(ext).is_some()
}
