use crate::core::{read_file, NTreeError};
use crate::extractors::extract_functions;
use crate::language::detect_language_config;
use crate::models::FunctionSpan;
use std::path::Path;
use tree_sitter::Parser;

/// Extracts all functions with their spans from a source file.
///
/// # Arguments
/// * `path` - Path to the source file (.rs, .py, etc.)
///
/// # Returns
/// * `Ok(Vec<FunctionSpan>)` - List of functions with their full and body spans
/// * `Err(NTreeError)` - If file cannot be read, parsed, or language unsupported
pub fn list_functions<P: AsRef<Path>>(path: P) -> Result<Vec<FunctionSpan>, NTreeError> {
    let path_ref = path.as_ref();
    let content = read_file(path_ref)?;
    let config = detect_language_config(path_ref)?;

    let mut parser = Parser::new();
    match parser.set_language(&config.language) {
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
    let functions = extract_functions(root_node, &content, &config);

    Ok(functions)
}
