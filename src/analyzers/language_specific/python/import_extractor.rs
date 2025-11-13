use super::ast_utils::PythonAstUtils;
use crate::core::NTreeError;
use crate::storage::{ExportEdge, ImportEdge, ImportType};
use std::path::PathBuf;
use tree_sitter::Node;

/// Python-specific import/export extractor.
pub struct PythonImportExtractor;

impl PythonImportExtractor {
    /// Extract Python import/export relationships.
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
                "import_statement" => {
                    if let Some(import) = Self::extract_import_statement(child, source, file_path)?
                    {
                        imports.push(import);
                    }
                }
                "import_from_statement" => {
                    if let Some(import) = Self::extract_from_import(child, source, file_path)? {
                        imports.push(import);
                    }
                }
                _ => {}
            }
        }

        Ok((imports, exports))
    }

    /// Extract simple import statement (import module).
    fn extract_import_statement(
        import_node: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Option<ImportEdge>, NTreeError> {
        let mut cursor = import_node.walk();

        for child in import_node.children(&mut cursor) {
            if child.kind() == "dotted_name" || child.kind() == "aliased_import" {
                let module_name = PythonAstUtils::extract_name(child, source);
                return Ok(Some(ImportEdge::new(
                    file_path.clone(),
                    module_name,
                    None,
                    ImportType::Module,
                    PythonAstUtils::extract_span(import_node),
                    Self::extract_import_text(import_node, source),
                )));
            }
        }

        Ok(None)
    }

    /// Extract from-import statement (from module import symbol).
    fn extract_from_import(
        import_node: Node,
        source: &str,
        file_path: &PathBuf,
    ) -> Result<Option<ImportEdge>, NTreeError> {
        let mut cursor = import_node.walk();
        let mut module_name = String::new();
        let mut import_type = ImportType::Symbol;

        for child in import_node.children(&mut cursor) {
            if child.kind() == "dotted_name" && module_name.is_empty() {
                module_name = PythonAstUtils::extract_name(child, source);
            } else if child.kind() == "wildcard_import" {
                import_type = ImportType::Wildcard;
                break;
            }
        }

        if !module_name.is_empty() {
            Ok(Some(ImportEdge::new(
                file_path.clone(),
                module_name,
                None,
                import_type,
                PythonAstUtils::extract_span(import_node),
                Self::extract_import_text(import_node, source),
            )))
        } else {
            Ok(None)
        }
    }

    /// Extract import statement text.
    fn extract_import_text(node: Node, source: &str) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        source.get(start..end).unwrap_or("").trim().to_string()
    }
}
