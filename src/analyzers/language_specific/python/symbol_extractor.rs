use std::path::PathBuf;
use tree_sitter::Node;
use crate::storage::TopLevelSymbol;
use crate::core::NTreeError;
use super::ast_utils::PythonAstUtils;

/// Python-specific symbol extractor for classes and methods.
pub struct PythonSymbolExtractor;

impl PythonSymbolExtractor {
    /// Extract all symbols from Python AST including class methods.
    pub fn extract_symbols(
        root: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        let mut symbols = Vec::new();
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "function_definition" => {
                    // Top-level function
                    symbols.push(Self::create_function_symbol(child, source, file_path, None)?);
                }
                "class_definition" => {
                    // Class and its methods
                    symbols.extend(Self::extract_class_symbols(child, source, file_path)?);
                }
                _ => {}
            }
        }

        Ok(symbols)
    }

    /// Extract class and its methods.
    fn extract_class_symbols(
        class_node: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        let mut symbols = Vec::new();
        let class_name = PythonAstUtils::extract_name(class_node, source);

        // Add class symbol
        symbols.push(TopLevelSymbol::new(
            file_path.clone(),
            class_name.clone(),
            "class".to_string(),
            format!("{}::{}", file_path.display(), class_name),
            PythonAstUtils::extract_span(class_node),
        ));

        // Add methods
        if let Some(body) = PythonAstUtils::find_class_body(class_node) {
            for method_node in PythonAstUtils::find_functions_in_block(body) {
                symbols.push(Self::create_function_symbol(
                    method_node,
                    source,
                    file_path,
                    Some(&class_name),
                )?);
            }
        }

        Ok(symbols)
    }

    /// Create function or method symbol.
    fn create_function_symbol(
        func_node: Node,
        source: &str,
        file_path: &PathBuf,
        class_name: Option<&str>,
    ) -> Result<TopLevelSymbol, NTreeError> {
        let func_name = PythonAstUtils::extract_name(func_node, source);
        let span = PythonAstUtils::extract_span(func_node);

        let (kind, qualname) = match class_name {
            Some(class) => (
                PythonAstUtils::get_method_type(&func_name).to_string(),
                format!("{}::{}::{}", file_path.display(), class, func_name),
            ),
            None => (
                "function".to_string(),
                format!("{}::{}", file_path.display(), func_name),
            ),
        };

        Ok(TopLevelSymbol::new(
            file_path.clone(),
            func_name,
            kind,
            qualname,
            span,
        ))
    }
}