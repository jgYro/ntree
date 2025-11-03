use super::error::NTreeError;
use super::reader::read_file;
use std::path::Path;
use tree_sitter::{Node, Parser};

/// Creates a Tree-sitter syntax tree from a Rust source file.
///
/// # Arguments
/// * `path` - Path to the Rust source file
///
/// # Returns
/// * `Ok(Node)` - Root node of the parsed syntax tree
/// * `Err(NTreeError)` - If file cannot be read or parsed
///
/// # Note
/// The returned node has a 'static lifetime by leaking the tree.
/// This is acceptable for the library's use case.
pub fn create_tree_from_file<P: AsRef<Path>>(path: P) -> Result<Node<'static>, NTreeError> {
    // Read file content
    let content = read_file(path)?;

    // Initialize parser
    let mut parser = Parser::new();

    // Set language to Rust
    match parser.set_language(&tree_sitter_rust::language().into()) {
        Ok(_) => {}
        Err(e) => return Err(NTreeError::ParseError(format!("Failed to set language: {:?}", e))),
    }

    // Parse the content
    let tree = match parser.parse(&content, None) {
        Some(t) => t,
        None => return Err(NTreeError::ParseError("Failed to parse file".to_string())),
    };

    // Leak tree to get 'static lifetime for the root node
    let tree = Box::leak(Box::new(tree));
    Ok(tree.root_node())
}