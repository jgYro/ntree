use super::ast_utils::RustAstUtils;
use crate::core::NTreeError;
use crate::storage::TopLevelSymbol;
use std::path::PathBuf;
use tree_sitter::Node;

/// Rust-specific symbol extractor for structs, impls, and functions.
pub struct RustSymbolExtractor;

impl RustSymbolExtractor {
    /// Extract symbols from Rust AST including impl methods.
    pub fn extract_symbols(
        root: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        let mut symbols = Vec::new();
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "function_item" => {
                    symbols.push(Self::create_symbol(
                        child, source, file_path, None, "function",
                    )?);
                }
                "struct_item" => {
                    symbols.push(Self::create_symbol(
                        child, source, file_path, None, "struct",
                    )?);
                }
                "impl_item" => {
                    symbols.extend(Self::extract_impl_symbols(child, source, file_path)?);
                }
                "enum_item" => {
                    symbols.push(Self::create_symbol(child, source, file_path, None, "enum")?);
                }
                _ => {}
            }
        }

        Ok(symbols)
    }

    /// Extract impl symbols (struct/enum + methods).
    fn extract_impl_symbols(
        impl_node: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Vec<TopLevelSymbol>, NTreeError> {
        let mut symbols = Vec::new();
        let impl_target = RustAstUtils::extract_impl_target(impl_node, source);

        // Add methods
        for method_node in RustAstUtils::find_functions_in_node(impl_node) {
            symbols.push(Self::create_symbol(
                method_node,
                source,
                file_path,
                Some(&impl_target),
                "method",
            )?);
        }

        Ok(symbols)
    }

    /// Create symbol (generic for all types).
    fn create_symbol(
        node: Node,
        source: &str,
        file_path: &PathBuf,
        parent: Option<&str>,
        default_kind: &str,
    ) -> Result<TopLevelSymbol, NTreeError> {
        let name = RustAstUtils::extract_name(node, source);
        let span = RustAstUtils::extract_span(node);

        let (kind, qualname) = match parent {
            Some(parent_name) => {
                let kind = RustAstUtils::get_method_type(&name, true);
                (
                    kind.to_string(),
                    format!("{}::{}::{}", file_path.display(), parent_name, name),
                )
            }
            None => (
                default_kind.to_string(),
                format!("{}::{}", file_path.display(), name),
            ),
        };

        Ok(TopLevelSymbol::new(
            file_path.clone(),
            name,
            kind,
            qualname,
            span,
        ))
    }
}
