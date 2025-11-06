use std::path::PathBuf;
use tree_sitter::Node;
use crate::storage::TopLevelSymbol;
use crate::core::NTreeError;

/// TypeScript-specific symbol extractor.
pub struct TypeScriptSymbolExtractor;

impl TypeScriptSymbolExtractor {
    /// Extract symbols from TypeScript AST.
    pub fn extract_symbols(
        _root: Node,
        _source: &str,
        _file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        // Not implemented yet
        Ok(Vec::new())
    }
}