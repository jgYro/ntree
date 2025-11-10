use std::path::PathBuf;
use crate::core::NTreeError;
use crate::language::SupportedLanguage;
use crate::storage::SymbolStore;
use crate::analyzers::language_specific::{
    python::PythonSymbolExtractor,
    rust::RustSymbolExtractor,
    javascript::JavaScriptSymbolExtractor,
    typescript::TypeScriptSymbolExtractor,
    java::JavaSymbolExtractor,
    c::CSymbolExtractor,
    cpp::CppSymbolExtractor,
};

/// Language-specific symbol extraction dispatcher.
pub struct SymbolExtractors;

impl SymbolExtractors {
    /// Extract symbols using language-specific extractors.
    pub fn extract_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        // Explicit language routing - no catch-all patterns
        match SupportedLanguage::from_path(file_path)? {
            SupportedLanguage::Rust => Self::extract_rust_symbols(file_path, symbol_store),
            SupportedLanguage::Python => Self::extract_python_symbols(file_path, symbol_store),
            SupportedLanguage::JavaScript => Self::extract_javascript_symbols(file_path, symbol_store),
            SupportedLanguage::TypeScript => Self::extract_typescript_symbols(file_path, symbol_store),
            SupportedLanguage::Java => Self::extract_java_symbols(file_path, symbol_store),
            SupportedLanguage::C => Self::extract_c_symbols(file_path, symbol_store),
            SupportedLanguage::Cpp => Self::extract_cpp_symbols(file_path, symbol_store),
        }
    }

    /// Extract Rust symbols including impl methods.
    fn extract_rust_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = RustSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()),
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
            Err(_) => Ok(()),
        }
    }

    /// Extract JavaScript symbols.
    fn extract_javascript_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = JavaScriptSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

    /// Extract TypeScript symbols.
    fn extract_typescript_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = TypeScriptSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

    /// Extract Java symbols.
    fn extract_java_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = JavaSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

    /// Extract C symbols.
    fn extract_c_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = CSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

    /// Extract C++ symbols.
    fn extract_cpp_symbols(file_path: &PathBuf, symbol_store: &mut SymbolStore) -> Result<(), NTreeError> {
        match crate::create_tree_from_file(file_path) {
            Ok(root) => {
                let source = std::fs::read_to_string(file_path)?;
                let symbols = CppSymbolExtractor::extract_symbols(root, &source, file_path)?;
                for symbol in symbols {
                    symbol_store.add_symbol(symbol);
                }
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

}