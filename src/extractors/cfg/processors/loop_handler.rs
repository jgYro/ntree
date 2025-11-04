use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::CfgContext;
use super::super::statements::{process_while, process_for};
use tree_sitter::Node;

/// Handle different types of loop expressions in blocks.
pub fn handle_loop_expression(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    loop_node: Node,
    source: &str,
    current: usize,
) -> Option<usize> {
    match loop_node.kind() {
        "while_expression" => {
            let exits = process_while(cfg, ctx, loop_node, source, current);
            handle_loop_exits(exits)
        }
        "for_expression" => {
            let (exits, _for_ir) = process_for(cfg, ctx, loop_node, source, current);
            handle_loop_exits(exits)
        }
        _ => None,
    }
}

/// Handle if expression with joining logic.
pub fn handle_if_with_join(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    exits: Vec<usize>,
    current: usize,
) -> usize {
    if exits.is_empty() {
        return current; // Should signal termination upstream
    }

    // Create join node if we have branches that need joining
    if exits.len() > 1 || (exits.len() == 1 && exits[0] != current) {
        let join_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(join_id, "join".to_string()));

        for exit in &exits {
            if *exit != join_id {
                cfg.add_edge(CfgEdge::new(*exit, join_id, "next".to_string()));
            }
        }
        join_id
    } else if !exits.is_empty() {
        exits[0]
    } else {
        current
    }
}

/// Handle loop exit points.
fn handle_loop_exits(exits: Vec<usize>) -> Option<usize> {
    if exits.is_empty() {
        None // Signal termination
    } else if !exits.is_empty() {
        Some(exits[0])
    } else {
        None
    }
}