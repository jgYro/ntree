use crate::core::NTreeError;
use crate::storage::TopLevelSymbol;
use std::path::PathBuf;
use tree_sitter::Node;

/// JavaScript-specific symbol extractor.
pub struct JavaScriptSymbolExtractor;

impl JavaScriptSymbolExtractor {
    /// Extract symbols from JavaScript AST.
    pub fn extract_symbols(
        _root: Node,
        _source: &str,
        _file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        // JavaScript symbol extraction not implemented yet
        Ok(Vec::new())
    }
}
