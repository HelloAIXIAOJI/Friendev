use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

use crate::tools::executor::file_operations::file_outline::{self, SymbolSource};

pub struct Indexer {
    db_path: std::path::PathBuf,
}

impl Indexer {
    pub fn new(project_root: &Path) -> Result<Self> {
        let index_dir = project_root.join(".friendev").join("index");
        if !index_dir.exists() {
            std::fs::create_dir_all(&index_dir)?;
        }
        let db_path = index_dir.join("outline.db");
        let indexer = Self { db_path };
        indexer.init_db()?;
        Ok(indexer)
    }

    fn get_connection(&self) -> Result<Connection> {
        Ok(Connection::open(&self.db_path)?)
    }

    fn init_db(&self) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                last_modified INTEGER NOT NULL,
                indexed_at INTEGER NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS symbols (
                id INTEGER PRIMARY KEY,
                file_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                kind TEXT NOT NULL,
                line INTEGER NOT NULL,
                content TEXT NOT NULL,
                FOREIGN KEY(file_id) REFERENCES files(id) ON DELETE CASCADE
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub async fn index_file(&self, path: &Path, project_root: &Path, use_tree_sitter: bool, use_lsp: bool) -> Result<String> {
        let relative_path = path.strip_prefix(project_root).unwrap_or(path);
        let relative_path_str = relative_path.to_string_lossy().to_string();

        // 1. Parse file
        let (symbols, source) = file_outline::get_symbols_with_source(path, use_tree_sitter, use_lsp).await?;

        // 2. Update DB
        let mut conn = self.get_connection()?;
        let tx = conn.transaction()?;

        // Get file metadata
        let metadata = std::fs::metadata(path)?;
        let last_modified = metadata
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;

        // Insert or replace file entry
        tx.execute(
            "INSERT OR REPLACE INTO files (path, last_modified, indexed_at) VALUES (?1, ?2, ?3)",
            params![relative_path_str, last_modified, now],
        )?;
        let file_id: i64 = tx.query_row(
            "SELECT id FROM files WHERE path = ?1",
            params![relative_path_str],
            |row| row.get(0),
        )?;

        // Clear old symbols
        tx.execute("DELETE FROM symbols WHERE file_id = ?1", params![file_id])?;

        // Insert new symbols
        let mut stmt = tx.prepare(
            "INSERT INTO symbols (file_id, name, kind, line, content) VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        for symbol in symbols {
            stmt.execute(params![
                file_id,
                symbol.name,
                symbol.kind,
                symbol.line,
                symbol.content
            ])?;
        }
        drop(stmt);

        tx.commit()?;
        
        let source_str = match source {
            Some(SymbolSource::TreeSitter) => "use tree-sitter",
            Some(SymbolSource::Lsp) => "use LSP",
            None => "no symbols",
        };
        
        Ok(source_str.to_string())
    }

    pub fn search_symbols(&self, pattern: &str) -> Result<Vec<(String, String, String, usize)>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT f.path, s.name, s.kind, s.line 
             FROM symbols s 
             JOIN files f ON s.file_id = f.id 
             WHERE s.name LIKE ?1
             ORDER BY f.path, s.line
             LIMIT 100",
        )?;
        
        // Simple SQL LIKE pattern for now. Could use REGEXP if we load extension, 
        // but for now simple substring match via %pattern%
        let sql_pattern = format!("%{}%", pattern); 
        
        let rows = stmt.query_map(params![sql_pattern], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_last_commit(&self) -> Result<Option<String>> {
        let conn = self.get_connection()?;
        let res: Option<String> = conn.query_row(
            "SELECT value FROM meta WHERE key = 'last_commit_hash'",
            [],
            |row| row.get(0),
        ).optional()?;
        Ok(res)
    }

    pub fn set_last_commit(&self, hash: &str) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('last_commit_hash', ?1)",
            params![hash],
        )?;
        Ok(())
    }
}
