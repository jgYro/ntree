use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use tree_sitter::Node;

mod branches;
mod core;
pub mod ir_converter;
pub mod processors;
mod statements;

use core::CfgContext;
use processors::process_block;

/// Builds a Control Flow Graph from a function body block with if/else support.
pub fn build_cfg_from_block(block_node: Node, source: &str) -> ControlFlowGraph {
    let mut cfg = ControlFlowGraph::new();
    let mut ctx = CfgContext::new();

    // Add ENTRY node
    let entry_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(entry_id, "ENTRY".to_string()));

    // Process block and get exit points
    let exits = process_block(&mut cfg, &mut ctx, block_node, source, entry_id);

    // Add EXIT node
    cfg.add_node(CfgNode::new(ctx.exit_id, "EXIT".to_string()));

    // Connect exit points to EXIT
    for exit in exits {
        if exit != ctx.exit_id {
            cfg.add_edge(CfgEdge::new(exit, ctx.exit_id, "next".to_string()));
        }
    }

    cfg
}
