use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::cfg_context::CfgContext;
use super::super::core::cfg_utils::get_statement_text;
use super::process_match_arm::process_match_arm;
use tree_sitter::Node;

/// Process a match expression and return exit points.
/// Implements CFG-08: match expressions with n-way branch + join.
pub fn process_match(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    match_node: Node,
    source: &str,
    entry: usize,
) -> Vec<usize> {
    // Get match expression and body
    let mut cursor = match_node.walk();
    let mut match_expr = None;
    let mut match_body = None;

    for child in match_node.named_children(&mut cursor) {
        match child.kind() {
            "match_block" => match_body = Some(child),
            _ => {
                // First non-match_block child is the expression
                if match_expr.is_none() {
                    match_expr = Some(child);
                }
            }
        }
    }

    let expr = match match_expr {
        Some(node) => node,
        None => return vec![entry], // No expression found, fallback
    };

    let body = match match_body {
        Some(node) => node,
        None => return vec![entry], // No body found, fallback
    };

    // Create match dispatch node
    let dispatch_id = ctx.alloc_id();
    let expr_text = get_statement_text(expr, source);
    cfg.add_node(CfgNode::new(dispatch_id, format!("match {}", expr_text)));
    cfg.add_edge(CfgEdge::new(entry, dispatch_id, "next".to_string()));

    // Create join node
    let join_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(join_id, "match_join".to_string()));

    // Process each match arm
    let mut arm_exits = Vec::new();
    let mut cursor = body.walk();

    for child in body.named_children(&mut cursor) {
        if child.kind() == "match_arm" {
            let arm_exits_for_this_arm = process_match_arm(cfg, ctx, child, source, dispatch_id, join_id);
            arm_exits.extend(arm_exits_for_this_arm);
        }
    }

    // Connect all arm exits to join node
    for &exit in &arm_exits {
        if exit != join_id {
            cfg.add_edge(CfgEdge::new(exit, join_id, "next".to_string()));
        }
    }

    // If no arms or all arms return/break, the join node might be unreachable
    // but we still return it as a potential exit point
    vec![join_id]
}