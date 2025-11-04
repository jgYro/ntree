use crate::models::ForLoopIR;
use tree_sitter::Node;

/// Rust-specific analyzer for for loop constructs.
pub struct RustForLoopAnalyzer;

impl RustForLoopAnalyzer {
    /// Analyze a Rust for expression and return normalized IR.
    pub fn analyze(for_node: Node, source: &str, loop_id: String) -> Option<ForLoopIR> {
        // Rust only has iterator-style for loops: for pattern in iterator
        Self::extract_iterator_loop(for_node, source, loop_id)
    }

    /// Extract Rust iterator-style for loop: for pattern in iterator
    fn extract_iterator_loop(for_node: Node, source: &str, loop_id: String) -> Option<ForLoopIR> {
        let mut cursor = for_node.walk();
        let mut pattern = None;
        let mut iter_expr = None;

        // Walk through for_expression children to find pattern and iterator
        for child in for_node.named_children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    // The pattern (variable being bound)
                    if pattern.is_none() {
                        pattern = Some(Self::extract_text(child, source));
                    }
                }
                "block" => {
                    // Skip the body block
                    continue;
                }
                _ => {
                    // The iterator expression (after "in")
                    if pattern.is_some() && iter_expr.is_none() {
                        iter_expr = Some(Self::extract_text(child, source));
                    }
                }
            }
        }

        match (pattern, iter_expr) {
            (Some(p), Some(i)) => Some(ForLoopIR::new_iterator(loop_id, p, i)),
            _ => None,
        }
    }

    /// Extract text from a tree-sitter node.
    fn extract_text(node: Node, source: &str) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        source[start..end].trim().to_string()
    }
}