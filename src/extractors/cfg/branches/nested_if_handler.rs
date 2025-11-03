use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::CfgContext;
use super::process_if::process_if_with_edge_kind;
use tree_sitter::Node;

/// Handle nested if expression in branches.
pub fn handle_nested_if(
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
pub fn handle_expression_if(
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