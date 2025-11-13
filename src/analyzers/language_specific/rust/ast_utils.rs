use tree_sitter::Node;

/// Utilities for working with Rust AST nodes.
pub struct RustAstUtils;

impl RustAstUtils {
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

    /// Extract impl target (what struct/trait is being implemented).
    pub fn extract_impl_target(impl_node: Node, source: &str) -> String {
        let mut cursor = impl_node.walk();
        for child in impl_node.children(&mut cursor) {
            if child.kind() == "type_identifier" || child.kind() == "generic_type" {
                let start = child.start_byte();
                let end = child.end_byte();
                return source.get(start..end).unwrap_or("unknown").to_string();
            }
        }
        "unknown_impl".to_string()
    }

    /// Find function items in a block or impl.
    pub fn find_functions_in_node(parent: Node) -> Vec<Node> {
        let mut functions = Vec::new();
        let mut cursor = parent.walk();

        for child in parent.children(&mut cursor) {
            if child.kind() == "function_item" {
                functions.push(child);
            }
        }

        functions
    }

    /// Check if function is a Rust constructor pattern.
    pub fn is_constructor(func_name: &str) -> bool {
        matches!(func_name, "new" | "default" | "from" | "with")
    }

    /// Get method type based on name and context.
    pub fn get_method_type(func_name: &str, is_in_impl: bool) -> &'static str {
        if !is_in_impl {
            "function"
        } else if Self::is_constructor(func_name) {
            "constructor"
        } else if func_name.starts_with('_') {
            "private_method"
        } else {
            "method"
        }
    }

    /// Extract text content from node.
    pub fn extract_text(node: Node, source: &str) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        source.get(start..end).unwrap_or("").trim().to_string()
    }
}
