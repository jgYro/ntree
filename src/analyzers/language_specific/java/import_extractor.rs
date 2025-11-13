use crate::core::NTreeError;
use crate::storage::{ExportEdge, ImportEdge};
use std::path::PathBuf;
use tree_sitter::Node;

/// Java-specific import/export extractor.
pub struct JavaImportExtractor;

impl JavaImportExtractor {
    /// Extract Java import/export relationships.
    pub fn extract_dependencies(
        _root: Node,
        _source: &str,
        _file_path: &PathBuf,
    ) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        // Not implemented yet
        Ok((Vec::new(), Vec::new()))
    }
}
