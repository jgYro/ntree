use crate::error::NTreeError;
use crate::reader::read_file;
use std::path::Path;
use tree_sitter::{Node, Parser};

pub fn create_tree_from_file<P: AsRef<Path>>(path: P) -> Result<Node<'static>, NTreeError> {
    let content = read_file(path)?;

    let mut parser = Parser::new();

    match parser.set_language(&tree_sitter_rust::language().into()) {
        Ok(_) => {}
        Err(e) => return Err(NTreeError::ParseError(format!("Failed to set language: {:?}", e))),
    }

    let tree = match parser.parse(&content, None) {
        Some(t) => t,
        None => return Err(NTreeError::ParseError("Failed to parse file".to_string())),
    };

    let tree = Box::leak(Box::new(tree));
    Ok(tree.root_node())
}