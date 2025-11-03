use crate::core::{read_file, NTreeError};
use crate::extractors::build_cfg_from_block;
use crate::language::LanguageConfig;
use std::path::Path;
use tree_sitter::{Node, Parser};

/// Result containing both Mermaid and JSON representations of a CFG.
#[derive(Debug)]
pub struct CfgResult {
    pub function_name: String,
    pub mermaid: String,
    pub jsonl: String,
}

/// Generates Control Flow Graphs for all functions in a Rust file.
///
/// # Arguments
/// * `path` - Path to the Rust source file
///
/// # Returns
/// * `Ok(Vec<CfgResult>)` - CFG results for each function
/// * `Err(NTreeError)` - If file cannot be read or parsed
pub fn generate_cfgs<P: AsRef<Path>>(path: P) -> Result<Vec<CfgResult>, NTreeError> {
    let content = read_file(&path)?;
    let config = LanguageConfig::rust();

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
    let mut results = Vec::new();

    // Find all functions and generate CFGs for their bodies
    extract_function_cfgs(root_node, &content, &config, &mut results);

    Ok(results)
}

/// Recursively extracts CFGs from function nodes.
fn extract_function_cfgs(
    node: Node,
    source: &str,
    config: &LanguageConfig,
    results: &mut Vec<CfgResult>,
) {
    if node.kind() == config.get_function_node_type() {
        // Extract function name
        let function_name = extract_function_name(node, source, config);

        // Find the body block
        if let Some(body_node) = find_body_node(node, config) {
            // Build CFG from the body
            let cfg = build_cfg_from_block(body_node, source);

            // Create result with both representations
            let result = CfgResult {
                function_name,
                mermaid: cfg.to_mermaid(),
                jsonl: cfg.to_jsonl(),
            };

            results.push(result);
        }
    }

    // Recurse to children
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        extract_function_cfgs(child, source, config, results);
    }
}

/// Extracts the function name from a function node.
fn extract_function_name(node: Node, source: &str, config: &LanguageConfig) -> String {
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if config.identifier_types.contains(&child.kind()) {
            let start = child.start_byte();
            let end = child.end_byte();
            return source[start..end].to_string();
        }
    }

    "anonymous".to_string()
}

/// Finds the body block node of a function.
fn find_body_node<'a>(node: Node<'a>, config: &LanguageConfig) -> Option<Node<'a>> {
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if child.kind() == config.get_body_node_type() {
            return Some(child);
        }
    }

    None
}