use crate::core::NTreeError;
use crate::storage::{ExportEdge, ImportEdge, SymbolStore};
use std::path::PathBuf;

/// Clean language-aware extraction dispatcher.
pub struct LanguageExtractors;

impl LanguageExtractors {
    /// Extract symbols using language-specific extractors.
    pub fn extract_symbols(
        file_path: &PathBuf,
        symbol_store: &mut SymbolStore,
    ) -> Result<(), NTreeError> {
        use super::symbol_extractors::SymbolExtractors;
        SymbolExtractors::extract_symbols(file_path, symbol_store)
    }

    /// Extract imports and exports from file.
    pub fn extract_dependencies(
        file_path: &PathBuf,
    ) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        use super::dependency_extractors::DependencyExtractors;
        DependencyExtractors::extract_dependencies(file_path)
    }
}
