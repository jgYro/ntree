use std::path::PathBuf;
use crate::core::NTreeError;
use crate::language::SupportedLanguage;
use crate::storage::{SymbolStore, TopLevelSymbol, ImportEdge, ExportEdge};
use crate::analyzers::language_specific::python::{PythonSymbolExtractor, PythonImportExtractor};

/// Language-aware symbol extraction utilities.
pub struct LanguageExtractors;

impl LanguageExtractors {
    /// Extract symbols using language-specific extractors.
    pub fn extract_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        // Detect language and use appropriate extractor
        match SupportedLanguage::from_path(file_path) {
            Ok(SupportedLanguage::Python) => {
                Self::extract_python_symbols(file_path, symbol_store)
            }
            _ => {
                // Use generic function extraction for other languages
                Self::extract_generic_symbols(file_path, symbol_store)
            }
        }
    }

    /// Extract imports and exports from file.
    pub fn extract_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match SupportedLanguage::from_path(file_path) {
            Ok(SupportedLanguage::Python) => {
                Self::extract_python_dependencies(file_path)
            }
            _ => {
                // No import extraction implemented for other languages yet
                Ok((Vec::new(), Vec::new()))
            }
        }
    }

    /// Extract Python dependencies (imports/exports).
    fn extract_python_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                PythonImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(e) => Err(e),
        }
    }

    /// Extract Python symbols including class methods.
    fn extract_python_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = PythonSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()), // Skip files that can't be parsed
        }
    }

    /// Extract symbols using generic function extraction.
    fn extract_generic_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::api::list_functions(file_path) {
            Ok(functions) => {
                for function in functions {
                    let symbol = TopLevelSymbol::new(
                        file_path.clone(),
                        function.function.clone(),
                        "function".to_string(),
                        format!("{}::{}", file_path.display(), function.function),
                        function.span.clone(),
                    );
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()), // Skip files that can't be analyzed
        }
    }
}