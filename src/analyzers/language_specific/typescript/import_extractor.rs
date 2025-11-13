use crate::core::NTreeError;
use crate::storage::{ExportEdge, ImportEdge};
use std::path::PathBuf;
use tree_sitter::Node;

/// TypeScript-specific import/export extractor.
pub struct TypeScriptImportExtractor;

impl TypeScriptImportExtractor {
    /// Extract TypeScript import/export relationships.
    pub fn extract_dependencies(
        _root: Node,
        _source: &str,
        _file_path: &PathBuf,
    ) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        // Not implemented yet
        Ok((Vec::new(), Vec::new()))
    }
}
