use std::path::PathBuf;
use crate::core::NTreeError;
use crate::language::SupportedLanguage;
use crate::storage::{ImportEdge, ExportEdge};
use crate::analyzers::language_specific::{
    python::PythonImportExtractor,
    javascript::JavaScriptImportExtractor,
    typescript::TypeScriptImportExtractor,
    java::JavaImportExtractor,
    c::CImportExtractor,
    cpp::CppImportExtractor,
};

/// Language-specific dependency extraction utilities.
pub struct DependencyExtractors;

impl DependencyExtractors {
    /// Extract imports and exports from file.
    pub fn extract_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        // Explicit language routing for import/export extraction
        match SupportedLanguage::from_path(file_path)? {
            SupportedLanguage::Rust => Self::extract_rust_dependencies(file_path),
            SupportedLanguage::Python => Self::extract_python_dependencies(file_path),
            SupportedLanguage::JavaScript => Self::extract_javascript_dependencies(file_path),
            SupportedLanguage::TypeScript => Self::extract_typescript_dependencies(file_path),
            SupportedLanguage::Java => Self::extract_java_dependencies(file_path),
            SupportedLanguage::C => Self::extract_c_dependencies(file_path),
            SupportedLanguage::Cpp => Self::extract_cpp_dependencies(file_path),
        }
    }

    /// Extract Rust dependencies.
    fn extract_rust_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(_root) => {
                // TODO: Implement Rust import extraction (use statements, extern crate)
                Ok((Vec::new(), Vec::new()))
            }
            Err(_) => Ok((Vec::new(), Vec::new())),
        }
    }

    /// Extract Python dependencies.
    fn extract_python_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                PythonImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(e) => Err(e),
        }
    }

    /// Extract JavaScript dependencies.
    fn extract_javascript_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                JavaScriptImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(_) => Ok((Vec::new(), Vec::new())),
        }
    }

    /// Extract TypeScript dependencies.
    fn extract_typescript_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                TypeScriptImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(_) => Ok((Vec::new(), Vec::new())),
        }
    }

    /// Extract Java dependencies.
    fn extract_java_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                JavaImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(_) => Ok((Vec::new(), Vec::new())),
        }
    }

    /// Extract C dependencies.
    fn extract_c_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                CImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(_) => Ok((Vec::new(), Vec::new())),
        }
    }

    /// Extract C++ dependencies.
    fn extract_cpp_dependencies(file_path: &PathBuf) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                CppImportExtractor::extract_dependencies(root, &source, file_path)
            }
            Err(_) => Ok((Vec::new(), Vec::new())),
        }
    }
}