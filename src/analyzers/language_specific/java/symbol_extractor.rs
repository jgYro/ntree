use crate::core::NTreeError;
use crate::storage::TopLevelSymbol;
use std::path::PathBuf;
use tree_sitter::Node;

/// Java-specific symbol extractor.
pub struct JavaSymbolExtractor;

impl JavaSymbolExtractor {
    /// Extract symbols from Java AST.
    pub fn extract_symbols(
        _root: Node,
        _source: &str,
        _file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        // Not implemented yet
        Ok(Vec::new())
    }
}
