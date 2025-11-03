use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::cfg_utils::{get_statement_text, is_statement_node};
use super::cfg_context::CfgContext;
use super::process_if::process_if_with_edge_kind;
use tree_sitter::Node;

/// Process then branch.
pub fn process_then_branch(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    block: Node,
    source: &str,
    cond_id: usize,
) -> Vec<usize> {
    let mut cursor = block.walk();
    let mut first = true;
    let mut current = cond_id;

    for child in block.named_children(&mut cursor) {
        if !is_statement_node(child) {
            continue;
        }

        // Check for nested if expressions
        if child.kind() == "if_expression" {
            handle_nested_if(cfg, ctx, child, source, cond_id, &mut current, &mut first);
            continue;
        }

        // Check if expression_statement contains an if_expression
        if child.kind() == "expression_statement" {
            if handle_expression_if(cfg, ctx, child, source, cond_id, &mut current, &mut first) {
                continue;
            }
        }

        // Regular statement processing
        let text = get_statement_text(child, source);
        let node_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(node_id, text.clone()));

        if first {
            cfg.add_edge(CfgEdge::new(cond_id, node_id, "true".to_string()));
            first = false;
        } else {
            cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
        }

        // Check for return
        if child.kind() == "return_expression" || text.starts_with("return") {
            cfg.add_edge(CfgEdge::new(node_id, ctx.exit_id, "exit".to_string()));
            return vec![]; // Terminated
        }

        current = node_id;
    }

    if first {
        vec![cond_id] // Empty then block
    } else {
        vec![current]
    }
}

/// Handle nested if expression in then branch.
fn handle_nested_if(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    if_node: Node,
    source: &str,
    cond_id: usize,
    current: &mut usize,
    first: &mut bool,
) {
    let (entry, edge_kind) = if *first {
        *first = false;
        (cond_id, "true")
    } else {
        (*current, "next")
    };

    let exits = process_if_with_edge_kind(cfg, ctx, if_node, source, entry, edge_kind);

    if exits.len() > 1 {
        let join_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(join_id, "join".to_string()));
        for exit in &exits {
            if *exit != join_id {
                cfg.add_edge(CfgEdge::new(*exit, join_id, "next".to_string()));
            }
        }
        *current = join_id;
    } else if !exits.is_empty() {
        *current = exits[0];
    }
}

/// Handle if expression within expression_statement.
fn handle_expression_if(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    stmt: Node,
    source: &str,
    cond_id: usize,
    current: &mut usize,
    first: &mut bool,
) -> bool {
    let mut expr_cursor = stmt.walk();
    for expr in stmt.named_children(&mut expr_cursor) {
        if expr.kind() == "if_expression" {
            handle_nested_if(cfg, ctx, expr, source, cond_id, current, first);
            return true;
        }
    }
    false
}