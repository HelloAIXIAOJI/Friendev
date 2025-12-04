use anyhow::Result;
use std::path::Path;
use async_lsp_client::LspServer;
use lsp_types::{
    InitializeParams, ClientCapabilities, Url,
    DocumentSymbolParams, TextDocumentIdentifier,
    DocumentSymbol,
    DidOpenTextDocumentParams, TextDocumentItem,
    request::DocumentSymbolRequest,
    notification::{DidOpenTextDocument, Initialized},
};

pub struct LspClient {
    server: LspServer,
}

impl LspClient {
    pub async fn new(cmd: &str, args: &[&str]) -> Result<Self> {
        // Collect args to vector of strings to satisfy interface
        let args_vec: Vec<&str> = args.iter().copied().collect();
        let (server, _) = LspServer::new(cmd, args_vec);
        // Removed .await, as new() is not async (returns tuple directly)
        
        Ok(Self { server })
    }

    pub async fn initialize(&mut self, root_path: &Path) -> Result<()> {
        let root_uri = Url::from_directory_path(root_path)
            .map_err(|_| anyhow::anyhow!("Invalid root path"))?;

        let params = InitializeParams {
            process_id: Some(std::process::id()),
            root_uri: Some(root_uri),
            capabilities: ClientCapabilities::default(),
            ..Default::default()
        };

        self.server.initialize(params).await
            .map_err(|e| anyhow::anyhow!("LSP initialization failed: {}", e))?;
            
        // send_notification likely returns () or Result<()> that we should handle if it errors?
        // If it returns (), we can't map_err.
        // Assuming it is fire-and-forget or panic-on-fail based on previous errors.
        self.server.send_notification::<Initialized>(lsp_types::InitializedParams {}).await;

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.server.shutdown().await
            .map_err(|e| anyhow::anyhow!("LSP shutdown failed: {}", e))?;
            
        self.server.exit().await;
            
        Ok(())
    }
    
    pub async fn document_symbol(&mut self, path: &Path) -> Result<Vec<DocumentSymbol>> {
        let uri = Url::from_file_path(path)
            .map_err(|_| anyhow::anyhow!("Invalid file path"))?;

        if let Ok(content) = tokio::fs::read_to_string(path).await {
            let item = TextDocumentItem {
                uri: uri.clone(),
                language_id: "unknown".to_string(),
                version: 1,
                text: content,
            };
            let params = DidOpenTextDocumentParams {
                text_document: item,
            };
            self.server.send_notification::<DidOpenTextDocument>(params).await;
        }

        let params = DocumentSymbolParams {
            text_document: TextDocumentIdentifier {
                uri,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        let result = self.server.send_request::<DocumentSymbolRequest>(params).await
            .map_err(|e| anyhow::anyhow!("Document symbol request failed: {}", e))?;
            
        let mut symbols = Vec::new();
        
        if let Some(response) = result {
            match response {
                lsp_types::DocumentSymbolResponse::Flat(si) => {
                     for info in si {
                        #[allow(deprecated)]
                        symbols.push(DocumentSymbol {
                            name: info.name,
                            detail: None,
                            kind: info.kind,
                            tags: None,
                            deprecated: info.deprecated,
                            range: info.location.range,
                            selection_range: info.location.range,
                            children: None,
                        });
                    }
                },
                lsp_types::DocumentSymbolResponse::Nested(ds) => {
                    fn flatten(symbols: Vec<DocumentSymbol>) -> Vec<DocumentSymbol> {
                        let mut result = Vec::new();
                        for sym in symbols {
                            result.push(sym.clone());
                            if let Some(children) = sym.children {
                                result.extend(flatten(children));
                            }
                        }
                        result
                    }
                    symbols = flatten(ds);
                }
            }
        }

        Ok(symbols)
    }
}
