use tree_sitter::Node;
use crate::core::NTreeError;
use crate::storage::{CallEdge, SymbolId, CallType};
use super::ast_utils::PythonAstUtils;

/// Python-specific call site extractor.
pub struct PythonCallExtractor;

impl PythonCallExtractor {
    /// Extract call sites from Python function body.
    pub fn extract_call_sites(
        function_body: Node,
        source: &str,
        caller_sym: &SymbolId,
    ) -> Result<Vec<CallEdge>, NTreeError> {
        let mut call_edges = Vec::new();
        Self::visit_node_for_calls(function_body, source, caller_sym, &mut call_edges);
        Ok(call_edges)
    }

    /// Recursively visit AST nodes looking for call expressions.
    fn visit_node_for_calls(
        node: Node,
        source: &str,
        caller_sym: &SymbolId,
        call_edges: &mut Vec<CallEdge>,
    ) {
        match node.kind() {
            "call" => {
                if let Some(call_edge) = Self::extract_call_expression(node, source, caller_sym) {
                    call_edges.push(call_edge);
                }
            }
            _ => {
                // Recursively visit children
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::visit_node_for_calls(child, source, caller_sym, call_edges);
                }
            }
        }
    }

    /// Extract call edge from Python call expression.
    fn extract_call_expression(
        call_node: Node,
        source: &str,
        caller_sym: &SymbolId,
    ) -> Option<CallEdge> {
        let span = PythonAstUtils::extract_span(call_node);
        let call_text = Self::extract_call_text(call_node, source);

        // Extract function name being called
        let function_name = Self::extract_function_name(call_node, source);

        Some(CallEdge::new(
            caller_sym.clone(),
            span,
            call_text,
        ).with_dynamic_hints(vec![function_name])) // Python calls are dynamic by default
    }

    /// Extract the function name from call expression.
    fn extract_function_name(call_node: Node, source: &str) -> String {
        let mut cursor = call_node.walk();
        for child in call_node.children(&mut cursor) {
            if child.kind() == "identifier" || child.kind() == "attribute" {
                return Self::extract_call_text(child, source);
            }
        }
        "unknown_function".to_string()
    }

    /// Extract call expression text.
    fn extract_call_text(node: Node, source: &str) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        source.get(start..end).unwrap_or("").trim().to_string()
    }

    /// Classify Python call type based on syntax.
    pub fn classify_call_type(call_text: &str) -> CallType {
        if call_text.contains("self.") {
            CallType::InstanceMethod
        } else if call_text.contains("::") || call_text.contains(".") {
            CallType::StaticMethod
        } else {
            CallType::FreeFunction
        }
    }
}