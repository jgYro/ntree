use std::path::PathBuf;
use tree_sitter::Node;
use crate::storage::TopLevelSymbol;
use crate::core::NTreeError;

/// C++-specific symbol extractor.
pub struct CppSymbolExtractor;

impl CppSymbolExtractor {
    /// Extract symbols from C++ AST.
    pub fn extract_symbols(
        _root: Node,
        _source: &str,
        _file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        // Not implemented yet
        Ok(Vec::new())
    }
}