use super::super::core::{CfgContext, LabelNormalizer};
use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use tree_sitter::Node;

/// Process a break expression and return empty exit points (path terminated).
/// Implements CFG-07: break support in loops.
pub fn process_break(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    _break_node: Node,
    _source: &str,
    entry: usize,
) -> Vec<usize> {
    // Create break node
    let break_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(break_id, LabelNormalizer::break_label()));
    cfg.add_edge(CfgEdge::new(entry, break_id, "next".to_string()));

    // Connect to current loop's after node
    if let Some(loop_ctx) = ctx.current_loop() {
        cfg.add_edge(CfgEdge::new(
            break_id,
            loop_ctx.after_id,
            "break".to_string(),
        ));
    }

    // Return empty vector as this path is terminated
    vec![]
}

/// Process a continue expression and return empty exit points (path terminated).
/// Implements CFG-07: continue support in loops.
pub fn process_continue(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    _continue_node: Node,
    _source: &str,
    entry: usize,
) -> Vec<usize> {
    // Create continue node
    let continue_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(continue_id, LabelNormalizer::continue_label()));
    cfg.add_edge(CfgEdge::new(entry, continue_id, "next".to_string()));

    // Connect to current loop's condition node
    if let Some(loop_ctx) = ctx.current_loop() {
        cfg.add_edge(CfgEdge::new(
            continue_id,
            loop_ctx.condition_id,
            "continue".to_string(),
        ));
    }

    // Return empty vector as this path is terminated
    vec![]
}
