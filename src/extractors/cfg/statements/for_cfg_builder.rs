use super::super::core::{CfgContext, LabelNormalizer};
use super::super::processors::process_block;
use crate::models::{CfgEdge, CfgNode, ControlFlowGraph, ForLoopIR};
use tree_sitter::Node;

/// Build CFG structure for a for loop.
pub fn build_for_loop_cfg(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    _for_node: Node,
    body: Node,
    source: &str,
    entry: usize,
    for_ir: &ForLoopIR,
) -> Vec<usize> {
    // Create for loop condition node with normalized label
    let condition_id = ctx.alloc_id();
    let condition_text = LabelNormalizer::for_loop_label(for_ir);

    cfg.add_node(CfgNode::new(condition_id, condition_text));
    cfg.add_edge(CfgEdge::new(entry, condition_id, "next".to_string()));

    // Create after-loop node
    let after_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(
        after_id,
        LabelNormalizer::loop_after_label("for_loop"),
    ));

    // Push loop context for break/continue handling
    ctx.push_loop(condition_id, after_id);

    // Create body start node
    let body_start_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(
        body_start_id,
        LabelNormalizer::loop_body_label("for_loop"),
    ));

    // Connect condition to body start (true branch)
    cfg.add_edge(CfgEdge::new(
        condition_id,
        body_start_id,
        "true".to_string(),
    ));

    // Process body
    let body_exits = process_block(cfg, ctx, body, source, body_start_id);

    // Pop loop context
    ctx.pop_loop();

    // Add back edges from body exits to condition
    for &exit in &body_exits {
        if exit != after_id {
            cfg.add_edge(CfgEdge::new(exit, condition_id, "back".to_string()));
        }
    }

    // Connect condition to after_loop (false branch)
    cfg.add_edge(CfgEdge::new(condition_id, after_id, "false".to_string()));

    vec![after_id]
}

/// Extract the body block from a for loop node.
pub fn extract_for_body(for_node: Node) -> Option<Node> {
    let mut cursor = for_node.walk();

    for child in for_node.named_children(&mut cursor) {
        if child.kind() == "block" {
            return Some(child);
        }
    }
    None
}
