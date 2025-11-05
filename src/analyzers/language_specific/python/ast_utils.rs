use tree_sitter::Node;

/// Utilities for working with Python AST nodes.
pub struct PythonAstUtils;

impl PythonAstUtils {
    /// Extract identifier name from a node.
    pub fn extract_name(node: Node, source: &str) -> String {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                let start = child.start_byte();
                let end = child.end_byte();
                return source.get(start..end).unwrap_or("unknown").to_string();
            }
        }
        "unknown".to_string()
    }

    /// Extract span information from a node.
    pub fn extract_span(node: Node) -> String {
        let start_point = node.start_position();
        let end_point = node.end_position();
        format!(
            "{}:{}–{}:{}",
            start_point.row + 1,
            start_point.column + 1,
            end_point.row + 1,
            end_point.column + 1
        )
    }

    /// Find all function definitions in a block.
    pub fn find_functions_in_block(block: Node) -> Vec<Node> {
        let mut functions = Vec::new();
        let mut cursor = block.walk();

        for child in block.children(&mut cursor) {
            if child.kind() == "function_definition" {
                functions.push(child);
            }
        }

        functions
    }

    /// Find the class body block.
    pub fn find_class_body(class_node: Node) -> Option<Node> {
        let mut cursor = class_node.walk();
        for child in class_node.children(&mut cursor) {
            if child.kind() == "block" {
                return Some(child);
            }
        }
        None
    }

    /// Check if a function is a Python constructor.
    pub fn is_constructor(func_name: &str) -> bool {
        matches!(func_name, "__init__" | "__new__")
    }

    /// Get method type based on name.
    pub fn get_method_type(func_name: &str) -> &'static str {
        if Self::is_constructor(func_name) {
            "constructor"
        } else if func_name.starts_with('_') && func_name.ends_with('_') {
            "special_method"
        } else if func_name.starts_with('_') {
            "private_method"
        } else {
            "method"
        }
    }
}