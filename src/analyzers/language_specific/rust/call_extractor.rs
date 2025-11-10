use tree_sitter::Node;
use crate::core::NTreeError;
use crate::storage::{CallEdge, SymbolId, CallType, CallConfidence};

/// Rust-specific call site extractor.
pub struct RustCallExtractor;

impl RustCallExtractor {
    /// Extract call sites from Rust function body.
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
            "call_expression" => {
                if let Some(call_edge) = Self::extract_call_expression(node, source, caller_sym) {
                    call_edges.push(call_edge);
                }
            }
            "macro_invocation" => {
                if let Some(call_edge) = Self::extract_macro_call(node, source, caller_sym) {
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

    /// Extract call edge from Rust call expression.
    fn extract_call_expression(
        call_node: Node,
        source: &str,
        caller_sym: &SymbolId,
    ) -> Option<CallEdge> {
        let span = Self::extract_span(call_node);
        let call_text = Self::extract_call_text(call_node, source);
        let function_name = Self::extract_function_name(call_node, source);

        // Classify call type based on Rust syntax
        let call_type = Self::classify_rust_call_type(&call_text);
        let confidence = match call_type {
            CallType::FreeFunction | CallType::StaticMethod => CallConfidence::Direct,
            CallType::InstanceMethod => CallConfidence::Virtual,
            _ => CallConfidence::Unknown,
        };

        let mut edge = CallEdge::new(caller_sym.clone(), span, call_text);
        edge.call_type = call_type;
        edge.confidence = confidence;
        edge.module_hints = vec![function_name];

        Some(edge)
    }

    /// Extract macro invocation as call.
    fn extract_macro_call(
        macro_node: Node,
        source: &str,
        caller_sym: &SymbolId,
    ) -> Option<CallEdge> {
        let span = Self::extract_span(macro_node);
        let call_text = Self::extract_call_text(macro_node, source);

        Some(CallEdge::new(caller_sym.clone(), span, call_text)
            .with_direct_target(caller_sym.clone())) // Placeholder for macro resolution
    }

    /// Classify Rust call type based on syntax.
    fn classify_rust_call_type(call_text: &str) -> CallType {
        if call_text.contains("::new") || call_text.contains("::default") {
            CallType::Constructor
        } else if call_text.contains("::") {
            CallType::StaticMethod
        } else if call_text.contains(".") {
            CallType::InstanceMethod
        } else {
            CallType::FreeFunction
        }
    }

    /// Extract span from node.
    fn extract_span(node: Node) -> String {
        let start = node.start_position();
        let end = node.end_position();
        format!("{}:{}–{}:{}", start.row + 1, start.column + 1, end.row + 1, end.column + 1)
    }

    /// Extract call text.
    fn extract_call_text(node: Node, source: &str) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        source.get(start..end).unwrap_or("").trim().to_string()
    }

    /// Extract function name being called.
    fn extract_function_name(call_node: Node, source: &str) -> String {
        let mut cursor = call_node.walk();
        for child in call_node.children(&mut cursor) {
            if child.kind() == "identifier" || child.kind() == "field_expression" {
                return Self::extract_call_text(child, source);
            }
        }
        "unknown_function".to_string()
    }
}