use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::cfg_utils::{get_if_condition, get_if_parts};
use super::cfg_context::CfgContext;
use super::process_then::process_then_branch;
use super::process_else::process_else_branch;
use tree_sitter::Node;

/// Process an if expression.
pub fn process_if(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    if_node: Node,
    source: &str,
    entry: usize,
) -> Vec<usize> {
    process_if_with_edge_kind(cfg, ctx, if_node, source, entry, "next")
}

/// Process an if expression with specific edge kind.
pub fn process_if_with_edge_kind(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    if_node: Node,
    source: &str,
    entry: usize,
    edge_kind: &str,
) -> Vec<usize> {
    // Create condition node
    let condition = get_if_condition(if_node, source);

    let cond_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(cond_id, format!("if ({})", condition)));
    cfg.add_edge(CfgEdge::new(entry, cond_id, edge_kind.to_string()));

    let mut exits = Vec::new();

    // Find then and else parts
    let (then_block, else_part) = get_if_parts(if_node);

    // Process then branch
    if let Some(then) = then_block {
        let then_exits = process_then_branch(cfg, ctx, then, source, cond_id);
        exits.extend(then_exits);
    }

    // Process else branch
    if let Some(else_node) = else_part {
        let else_exits = process_else_branch(cfg, ctx, else_node, source, cond_id);
        exits.extend(else_exits);
    } else {
        // No else - false edge becomes an exit point
        exits.push(cond_id);
    }

    exits
}