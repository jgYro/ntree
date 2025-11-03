use crate::models::FunctionSpan;
use crate::language::LanguageConfig;
use tree_sitter::Node;

/// Extracts all functions from a syntax tree.
///
/// Handles edge cases including:
/// - Functions with attributes (#[test], #[derive], etc.)
/// - Functions with where clauses
/// - Functions with braces on new lines
/// - Async, const, and unsafe functions
///
/// The body span always starts at the opening `{` and ends at the closing `}`.
pub fn extract_functions(root_node: Node, source: &str, config: &LanguageConfig) -> Vec<FunctionSpan> {
    let mut functions = Vec::new();
    extract_functions_recursive(root_node, source, config, &mut functions);
    functions
}

fn extract_functions_recursive(
    node: Node,
    source: &str,
    config: &LanguageConfig,
    functions: &mut Vec<FunctionSpan>,
) {
    if node.kind() == config.get_function_node_type() {
        let function_name = extract_function_name(node, source, config);
        let function_span = FunctionSpan::format_span(
            node.start_position().row,
            node.start_position().column,
            node.end_position().row,
            node.end_position().column,
        );

        let body_span = find_body_span(node, config);

        functions.push(FunctionSpan::new(function_name, function_span, body_span));
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        extract_functions_recursive(child, source, config, functions);
    }
}

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

/// Finds the body block of a function.
///
/// The body span includes the opening `{` and closing `}` braces,
/// regardless of where they appear (same line or new line after where clause).
fn find_body_span(node: Node, config: &LanguageConfig) -> Option<String> {
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if child.kind() == config.get_body_node_type() {
            return Some(FunctionSpan::format_span(
                child.start_position().row,
                child.start_position().column,
                child.end_position().row,
                child.end_position().column,
            ));
        }
    }

    None
}