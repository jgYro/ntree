use tree_sitter::Node;

/// Get if condition text.
pub fn get_if_condition(if_node: Node, source: &str) -> String {
    // Look for a condition child node or extract from text
    let mut cursor = if_node.walk();

    // Skip 'if' keyword and look for the condition expression
    for child in if_node.children(&mut cursor) {
        // The condition is typically a binary expression or identifier after 'if'
        if child.kind() != "if" && child.kind() != "block" && child.kind() != "else_clause" {
            let start = child.start_byte();
            let end = child.end_byte();
            return source[start..end].trim().to_string();
        }
    }

    // Fallback: extract from text
    let start = if_node.start_byte();
    let end = if_node.end_byte();
    let text = &source[start..end];

    if let Some(if_pos) = text.find("if") {
        if let Some(brace_pos) = text.find('{') {
            let cond = &text[if_pos + 2..brace_pos];
            return cond.trim().trim_start_matches('(').trim_end_matches(')').to_string();
        }
    }

    "condition".to_string()
}

/// Get then and else parts of if expression.
pub fn get_if_parts(if_node: Node) -> (Option<Node>, Option<Node>) {
    let mut cursor = if_node.walk();
    let mut then_block = None;
    let mut else_part = None;

    for child in if_node.named_children(&mut cursor) {
        match child.kind() {
            "block" => {
                if then_block.is_none() {
                    then_block = Some(child);
                }
            }
            "else_clause" => {
                else_part = Some(child);
            }
            _ => {}
        }
    }

    (then_block, else_part)
}

/// Get statement text.
pub fn get_statement_text(node: Node, source: &str) -> String {
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &source[start..end];

    // Clean up
    let cleaned = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    // Add semicolon if needed
    if !cleaned.ends_with(';')
        && !cleaned.starts_with("if")
        && !cleaned.starts_with("return")
        && !cleaned.contains("return ") {
        format!("{};", cleaned)
    } else {
        cleaned
    }
}

/// Check if node is a statement (language-agnostic).
pub fn is_statement_node(node: Node) -> bool {
    matches!(
        node.kind(),
        // Rust statement types
        "let_declaration"
            | "expression_statement"
            | "return_expression"
            | "if_expression"
            | "match_expression"
            | "while_expression"
            | "for_expression"
            | "loop_expression"
            | "macro_invocation"
            | "try_expression"
            | "assignment_expression"
        // Python statement types
            | "if_statement"
            | "try_statement"
            | "while_statement"
            | "for_statement"
            | "with_statement"
            | "import_statement"
            | "return_statement"
        // JavaScript/TypeScript statement types
            | "function_declaration"
            | "variable_declaration"
        // Java statement types
            | "method_declaration"
        // C/C++ statement types
            | "function_definition"
    )
}