use crate::models::EarlyExitIR;
use tree_sitter::Node;

/// Rust-specific analyzer for early-exit constructs.
pub struct RustEarlyExitAnalyzer;

impl RustEarlyExitAnalyzer {
    /// Analyze a Rust early-exit construct and return normalized IR.
    pub fn analyze(node: Node, source: &str, exit_id: String) -> Option<EarlyExitIR> {
        match node.kind() {
            "try_expression" => Self::extract_try_operator(node, source, exit_id),
            "macro_invocation" => Self::extract_panic_macro(node, source, exit_id),
            _ => {
                // Check for ? operator within statements
                if Self::contains_try_operator(node, source) {
                    return Self::extract_try_from_statement(node, source, exit_id);
                }
                // Check for panic! macro within statements
                if Self::is_panic_macro(node, source) {
                    return Self::extract_panic_from_statement(node, source, exit_id);
                }
                None
            }
        }
    }

    /// Check if a statement contains a try operator (?).
    pub fn contains_try_operator(stmt: Node, source: &str) -> bool {
        let text = Self::extract_text(stmt, source);
        text.contains("?") && !text.starts_with("if") && !text.starts_with("match")
    }

    /// Check if a statement is a panic! macro.
    pub fn is_panic_macro(stmt: Node, source: &str) -> bool {
        let text = Self::extract_text(stmt, source);
        text.starts_with("panic!")
    }

    /// Check if node contains any Rust early-exit construct.
    pub fn contains_early_exit(stmt: Node, source: &str) -> bool {
        Self::contains_try_operator(stmt, source) || Self::is_panic_macro(stmt, source)
    }

    /// Extract Rust try operator: expr?
    fn extract_try_operator(try_node: Node, source: &str, exit_id: String) -> Option<EarlyExitIR> {
        // Get the expression before the ?
        let mut cursor = try_node.walk();
        for child in try_node.named_children(&mut cursor) {
            if child.kind() != "?" {
                let expr_text = Self::extract_text(child, source);
                return Some(EarlyExitIR::new_try_operator(exit_id, expr_text));
            }
        }

        // Fallback to full text
        let full_text = Self::extract_text(try_node, source);
        Some(EarlyExitIR::new_try_operator(exit_id, full_text))
    }

    /// Extract Rust panic macro: panic!("message")
    fn extract_panic_macro(macro_node: Node, source: &str, exit_id: String) -> Option<EarlyExitIR> {
        let full_text = Self::extract_text(macro_node, source);

        // Check if it's a panic! macro
        if full_text.starts_with("panic!") {
            // Extract panic message if present
            let message = Self::extract_panic_message(macro_node, source);
            return Some(EarlyExitIR::new_panic(exit_id, full_text, message));
        }

        None
    }

    /// Extract panic message from panic! macro arguments.
    fn extract_panic_message(macro_node: Node, source: &str) -> Option<String> {
        let mut cursor = macro_node.walk();

        // Look for token_tree (macro arguments)
        for child in macro_node.named_children(&mut cursor) {
            if child.kind() == "token_tree" {
                let args_text = Self::extract_text(child, source);
                // Remove parentheses and extract message
                return Some(args_text.trim_start_matches('(').trim_end_matches(')').to_string());
            }
        }

        None
    }

    /// Extract try operator from a regular statement containing ?.
    fn extract_try_from_statement(stmt: Node, source: &str, exit_id: String) -> Option<EarlyExitIR> {
        let text = Self::extract_text(stmt, source);
        // For statements like "let result = foo()?;", extract "foo()?"
        if let Some(question_pos) = text.find('?') {
            let expr_part = &text[..=question_pos];
            if let Some(equals_pos) = expr_part.rfind('=') {
                let try_expr = &expr_part[equals_pos + 1..].trim();
                return Some(EarlyExitIR::new_try_operator(exit_id, try_expr.to_string()));
            }
            return Some(EarlyExitIR::new_try_operator(exit_id, expr_part.to_string()));
        }
        None
    }

    /// Extract panic macro from a regular statement.
    fn extract_panic_from_statement(stmt: Node, source: &str, exit_id: String) -> Option<EarlyExitIR> {
        let text = Self::extract_text(stmt, source);
        if text.starts_with("panic!") {
            let message = Self::extract_panic_message_from_text(&text);
            return Some(EarlyExitIR::new_panic(exit_id, text, message));
        }
        None
    }

    /// Extract panic message from panic! text.
    fn extract_panic_message_from_text(text: &str) -> Option<String> {
        if let Some(start) = text.find('(') {
            if let Some(end) = text.rfind(')') {
                let content = &text[start + 1..end];
                return Some(content.trim().to_string());
            }
        }
        None
    }

    /// Extract text from a tree-sitter node.
    fn extract_text(node: Node, source: &str) -> String {
        let start = node.start_byte();
        let end = node.end_byte();
        source[start..end].trim().to_string()
    }
}