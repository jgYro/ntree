use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::cfg_context::CfgContext;
use super::super::core::cfg_utils::get_statement_text;
use super::super::processors::process_block;
use tree_sitter::Node;

/// Process a single match arm and return exit points.
pub fn process_match_arm(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    arm_node: Node,
    source: &str,
    dispatch_id: usize,
    _join_id: usize,
) -> Vec<usize> {
    // Get pattern and body from match_arm
    let mut cursor = arm_node.walk();
    let mut pattern = None;
    let mut body = None;

    for child in arm_node.named_children(&mut cursor) {
        match child.kind() {
            "block" => body = Some(child),
            _ => {
                // First non-block child is the pattern
                if pattern.is_none() {
                    pattern = Some(child);
                }
            }
        }
    }

    let pattern_text = match pattern {
        Some(node) => get_statement_text(node, source),
        None => "default".to_string(),
    };

    match body {
        Some(body_node) => {
            // Create arm start node
            let arm_start_id = ctx.alloc_id();
            cfg.add_node(CfgNode::new(arm_start_id, format!("arm_start: {}", pattern_text)));
            cfg.add_edge(CfgEdge::new(dispatch_id, arm_start_id, pattern_text));

            // Process the arm body
            let exits = process_block(cfg, ctx, body_node, source, arm_start_id);
            exits
        }
        None => {
            // Expression arm (no block)
            let arm_id = ctx.alloc_id();
            cfg.add_node(CfgNode::new(arm_id, format!("arm: {}", pattern_text)));
            cfg.add_edge(CfgEdge::new(dispatch_id, arm_id, pattern_text));
            vec![arm_id]
        }
    }
}