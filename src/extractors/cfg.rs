use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use tree_sitter::Node;

/// Builds a Control Flow Graph from a function body block.
///
/// Walks through the block's statements sequentially, creating:
/// - An ENTRY node
/// - One node per statement
/// - An EXIT node
/// - Edges connecting them with proper control flow
///
/// Special handling:
/// - `return` statements connect directly to EXIT with "exit" edge
/// - No edges are created after a return statement
pub fn build_cfg_from_block(block_node: Node, source: &str) -> ControlFlowGraph {
    let mut cfg = ControlFlowGraph::new();
    let mut node_id = 0;

    // Add ENTRY node
    cfg.add_node(CfgNode::new(node_id, "ENTRY".to_string()));
    let entry_id = node_id;
    node_id += 1;

    // Reserve EXIT node ID
    let exit_id = count_statements(block_node) + 1;

    // Walk through statements in the block
    let mut cursor = block_node.walk();
    let mut prev_id = Some(entry_id);
    let mut terminated = false;

    for child in block_node.named_children(&mut cursor) {
        // Skip non-statement nodes
        if !is_statement_node(child) {
            continue;
        }

        // If we've hit a return, skip remaining statements (dead code)
        if terminated {
            continue;
        }

        // Extract statement text
        let statement_text = extract_statement_text(child, source);

        // Create node for this statement
        cfg.add_node(CfgNode::new(node_id, statement_text));

        // Connect to previous node if there is one
        if let Some(prev) = prev_id {
            cfg.add_edge(CfgEdge::new(prev, node_id, "next".to_string()));
        }

        // Check if this is a return statement
        if child.kind() == "return_expression" {
            // Connect return directly to EXIT with "exit" edge
            cfg.add_edge(CfgEdge::new(node_id, exit_id, "exit".to_string()));
            terminated = true;
            prev_id = None;  // No more edges after return
        } else if is_expression_statement_with_return(child, source) {
            // Handle return inside expression_statement
            cfg.add_edge(CfgEdge::new(node_id, exit_id, "exit".to_string()));
            terminated = true;
            prev_id = None;
        } else {
            prev_id = Some(node_id);
        }

        node_id += 1;
    }

    // Add EXIT node
    cfg.add_node(CfgNode::new(exit_id, "EXIT".to_string()));

    // Connect last statement to EXIT if not terminated by return
    if let Some(prev) = prev_id {
        if prev != entry_id || !has_any_statements(block_node) {
            // Either we have statements that need connection, or it's an empty function
            if prev == entry_id && !has_any_statements(block_node) {
                // Empty function: connect ENTRY directly to EXIT
                cfg.add_edge(CfgEdge::new(entry_id, exit_id, "next".to_string()));
            } else {
                // Normal case: connect last non-return statement to EXIT
                cfg.add_edge(CfgEdge::new(prev, exit_id, "next".to_string()));
            }
        }
    }

    cfg
}

/// Counts the number of statements in a block.
fn count_statements(block_node: Node) -> usize {
    let mut cursor = block_node.walk();
    let mut count = 0;

    for child in block_node.named_children(&mut cursor) {
        if is_statement_node(child) {
            count += 1;
        }
    }

    count
}

/// Checks if a block has any statements.
fn has_any_statements(block_node: Node) -> bool {
    let mut cursor = block_node.walk();

    for child in block_node.named_children(&mut cursor) {
        if is_statement_node(child) {
            return true;
        }
    }

    false
}

/// Checks if an expression statement contains a return.
fn is_expression_statement_with_return(node: Node, source: &str) -> bool {
    if node.kind() != "expression_statement" {
        return false;
    }

    // Check text for return keyword
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &source[start..end];
    text.trim().starts_with("return")
}

/// Checks if a node represents a statement.
fn is_statement_node(node: Node) -> bool {
    let kind = node.kind();
    // Common Rust statement types
    matches!(
        kind,
        "let_declaration"
            | "expression_statement"
            | "return_expression"
            | "if_expression"
            | "match_expression"
            | "while_expression"
            | "for_expression"
            | "loop_expression"
            | "macro_invocation"
            | "assignment_expression"
    )
}

/// Extracts the text of a statement, cleaning it up for display.
fn extract_statement_text(node: Node, source: &str) -> String {
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &source[start..end];

    // Clean up the text: remove excess whitespace, newlines
    let cleaned = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    // Ensure semicolon for clarity if not present (except for returns which look cleaner without)
    if !cleaned.ends_with(';') && !cleaned.starts_with("return") && !cleaned.contains("return ") {
        format!("{};", cleaned)
    } else {
        cleaned
    }
}