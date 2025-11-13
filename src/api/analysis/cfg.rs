use crate::core::{read_file, NTreeError};
use crate::extractors::cfg::build_cfg_from_block;
use crate::extractors::cfg::ir_converter::CFGToIRConverter;
use crate::extractors::cfg::processors::build_basic_blocks_from_block;
use crate::language::{detect_language_config, LanguageConfig};
use crate::models::FunctionCFGIR;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tree_sitter::{Node, Parser};

/// Result containing both Mermaid and JSON representations of a CFG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgResult {
    pub function_name: String,
    pub mermaid: String,
    pub jsonl: String,
}

/// Result containing basic block representation of a function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlockResult {
    pub function_name: String,
    pub jsonl: String,
}

/// Generates Control Flow Graphs with if/else support for all functions in a source file.
///
/// # Arguments
/// * `path` - Path to the source file (.rs, .py, etc.)
///
/// # Returns
/// * `Ok(Vec<CfgResult>)` - CFG results for each function
/// * `Err(NTreeError)` - If file cannot be read, parsed, or language unsupported
pub fn generate_cfgs<P: AsRef<Path>>(path: P) -> Result<Vec<CfgResult>, NTreeError> {
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
    let mut results = Vec::new();

    // Find all functions and generate CFGs for their bodies
    extract_function_cfgs(root_node, &content, &config, &mut results);

    Ok(results)
}

/// Alias for generate_cfgs to maintain backward compatibility with v2 naming.
pub use self::generate_cfgs as generate_cfgs_v2;

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

/// Generate basic block representations for all functions in a source file.
/// Coalesces straight-line statements into basic blocks.
pub fn generate_basic_blocks<P: AsRef<Path>>(path: P) -> Result<Vec<BasicBlockResult>, NTreeError> {
    let path_ref = path.as_ref();
    let source = read_file(path_ref)?;
    let config = detect_language_config(path_ref)?;

    let mut parser = Parser::new();
    parser
        .set_language(&config.language)
        .map_err(|e| NTreeError::ParseError(format!("Failed to set language: {:?}", e)))?;

    let tree = match parser.parse(&source, None) {
        Some(tree) => tree,
        None => return Err(NTreeError::ParseError("Failed to parse file".to_string())),
    };

    let root_node = tree.root_node();
    let mut results = Vec::new();
    let mut cursor = root_node.walk();

    for node in root_node.named_children(&mut cursor) {
        if node.kind() == config.get_function_node_type() {
            let function_name = extract_function_name(node, &source, &config);

            if let Some(body_node) = find_body_node(node, &config) {
                let bb_graph = build_basic_blocks_from_block(body_node, &source);
                let jsonl = bb_graph.to_jsonl();

                results.push(BasicBlockResult {
                    function_name,
                    jsonl,
                });
            }
        }
    }

    Ok(results)
}

/// Generate language-neutral IR for all functions in a file.
/// Implements CFG-11: Serialize to a language-neutral IR.
pub fn generate_cfg_ir<P: AsRef<Path>>(path: P) -> Result<Vec<FunctionCFGIR>, NTreeError> {
    let path_ref = path.as_ref();
    let source = read_file(path_ref)?;
    let config = detect_language_config(path_ref)?;

    let mut parser = Parser::new();
    parser
        .set_language(&config.language)
        .map_err(|e| NTreeError::ParseError(format!("Failed to set language: {:?}", e)))?;

    let tree = match parser.parse(&source, None) {
        Some(tree) => tree,
        None => return Err(NTreeError::ParseError("Failed to parse file".to_string())),
    };

    let source_file = path.as_ref().to_string_lossy().to_string();
    let root_node = tree.root_node();
    let mut results = Vec::new();
    let mut cursor = root_node.walk();

    for node in root_node.named_children(&mut cursor) {
        if node.kind() == config.get_function_node_type() {
            let function_name = extract_function_name(node, &source, &config);

            if let Some(body_node) = find_body_node(node, &config) {
                let cfg = build_cfg_from_block(body_node, &source);
                let ir =
                    CFGToIRConverter::convert_to_ir(&cfg, function_name, Some(source_file.clone()));
                results.push(ir);
            }
        }
    }

    Ok(results)
}

/// Generate and serialize CFG IR to JSONL format.
pub fn generate_cfg_ir_jsonl<P: AsRef<Path>>(path: P) -> Result<String, NTreeError> {
    let function_irs = generate_cfg_ir(path)?;
    Ok(CFGToIRConverter::serialize_to_jsonl(&function_irs))
}
