use super::error::NTreeError;
use super::reader::read_file;
use crate::language::detect_language_config;
use std::path::Path;
use tree_sitter::{Node, Parser};

/// Creates a Tree-sitter syntax tree from a source file with automatic language detection.
///
/// # Arguments
/// * `path` - Path to the source file (.rs, .py, etc.)
///
/// # Returns
/// * `Ok(Node)` - Root node of the parsed syntax tree
/// * `Err(NTreeError)` - If file cannot be read, parsed, or language unsupported
///
/// # Note
/// The returned node has a 'static lifetime by leaking the tree.
/// This is acceptable for the library's use case.
pub fn create_tree_from_file<P: AsRef<Path>>(path: P) -> Result<Node<'static>, NTreeError> {
    let path_ref = path.as_ref();

    // Read file content
    let content = read_file(path_ref)?;

    // Detect language configuration
    let language_config = detect_language_config(path_ref)?;

    // Initialize parser
    let mut parser = Parser::new();

    // Set language based on file extension
    match parser.set_language(&language_config.language) {
        Ok(_) => {}
        Err(e) => {
            return Err(NTreeError::ParseError(format!(
                "Failed to set language: {:?}",
                e
            )))
        }
    }

    // Parse the content
    let tree = match parser.parse(&content, None) {
        Some(t) => t,
        None => {
            return Err(NTreeError::ParseError(format!(
                "Failed to parse {} file",
                path_ref
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("unknown")
            )))
        }
    };

    // Leak tree to get 'static lifetime for the root node
    let tree = Box::leak(Box::new(tree));
    Ok(tree.root_node())
}
