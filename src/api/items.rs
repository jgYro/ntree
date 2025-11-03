use crate::core::{read_file, NTreeError};
use crate::extractors::extract_top_level_items;
use crate::models::TopLevelItem;
use std::path::Path;
use tree_sitter::Parser;

pub fn list_top_level_items<P: AsRef<Path>>(path: P) -> Result<Vec<TopLevelItem>, NTreeError> {
    let file_path = match path.as_ref().to_str() {
        Some(p) => p,
        None => {
            return Err(NTreeError::ParseError(
                "Invalid file path encoding".to_string(),
            ))
        }
    };

    let content = read_file(&path)?;

    let mut parser = Parser::new();
    match parser.set_language(&tree_sitter_rust::language().into()) {
        Ok(_) => {}
        Err(e) => {
            return Err(NTreeError::ParseError(format!(
                "Failed to set language: {:?}",
                e
            )))
        }
    }

    let tree = match parser.parse(&content, None) {
        Some(t) => t,
        None => return Err(NTreeError::ParseError("Failed to parse file".to_string())),
    };

    let root_node = tree.root_node();
    let items = extract_top_level_items(file_path, root_node, &content);

    Ok(items)
}