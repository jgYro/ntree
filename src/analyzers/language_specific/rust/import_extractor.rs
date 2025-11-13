use super::ast_utils::RustAstUtils;
use crate::core::NTreeError;
use crate::storage::{ExportEdge, ImportEdge, ImportType};
use std::path::PathBuf;
use tree_sitter::Node;

/// Rust-specific import/export extractor.
pub struct RustImportExtractor;

impl RustImportExtractor {
    /// Extract Rust import/export relationships.
    pub fn extract_dependencies(
        root: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<(Vec<ImportEdge>, Vec<ExportEdge>), NTreeError> {
        let mut imports = Vec::new();
        let exports = Vec::new();
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            match child.kind() {
                "use_declaration" => {
                    if let Some(import) = Self::extract_use_statement(child, source, file_path)? {
                        imports.push(import);
                    }
                }
                "extern_crate_declaration" => {
                    if let Some(import) = Self::extract_extern_crate(child, source, file_path)? {
                        imports.push(import);
                    }
                }
                _ => {}
            }
        }

        Ok((imports, exports))
    }

    /// Extract use statement.
    fn extract_use_statement(
        use_node: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Option<ImportEdge>, NTreeError> {
        let span = RustAstUtils::extract_span(use_node);
        let use_text = RustAstUtils::extract_text(use_node, source);

        let module_path = Self::extract_use_path(use_node, source);
        Ok(Some(ImportEdge::new(
            file_path.clone(),
            module_path,
            None,
            ImportType::Module,
            span,
            use_text,
        )))
    }

    /// Extract extern crate declaration.
    fn extract_extern_crate(
        extern_node: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Option<ImportEdge>, NTreeError> {
        let span = RustAstUtils::extract_span(extern_node);
        let extern_text = RustAstUtils::extract_text(extern_node, source);

        let crate_name = Self::extract_crate_name(extern_node, source);
        Ok(Some(ImportEdge::new(
            file_path.clone(),
            crate_name,
            None,
            ImportType::Module,
            span,
            extern_text,
        )))
    }

    /// Extract module path from use statement.
    fn extract_use_path(use_node: Node, source: &str) -> String {
        let mut cursor = use_node.walk();
        for child in use_node.children(&mut cursor) {
            if child.kind() == "scoped_use_list" || child.kind() == "use_list" {
                return RustAstUtils::extract_text(child, source);
            }
        }
        "unknown_module".to_string()
    }

    /// Extract crate name from extern crate.
    fn extract_crate_name(extern_node: Node, source: &str) -> String {
        let mut cursor = extern_node.walk();
        for child in extern_node.children(&mut cursor) {
            if child.kind() == "identifier" {
                return RustAstUtils::extract_text(child, source);
            }
        }
        "unknown_crate".to_string()
    }
}
