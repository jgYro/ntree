use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::{CfgContext, get_statement_text};
use super::super::processors::process_block;
use tree_sitter::Node;

/// Process a while expression and return exit points.
/// Implements CFG-06: while loops with condition + back edge.
pub fn process_while(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    while_node: Node,
    source: &str,
    entry: usize,
) -> Vec<usize> {
    // Get condition and body from while_expression
    let mut cursor = while_node.walk();
    let mut condition_node = None;
    let mut body_node = None;

    for child in while_node.named_children(&mut cursor) {
        match child.kind() {
            "block" => body_node = Some(child),
            _ => {
                // First non-block child is the condition
                if condition_node.is_none() {
                    condition_node = Some(child);
                }
            }
        }
    }

    let condition = match condition_node {
        Some(node) => node,
        None => return vec![entry], // No condition found, fallback
    };

    let body = match body_node {
        Some(node) => node,
        None => return vec![entry], // No body found, fallback
    };

    // Create condition node
    let condition_id = ctx.alloc_id();
    let condition_text = get_statement_text(condition, source);
    cfg.add_node(CfgNode::new(condition_id, format!("while {}", condition_text)));

    // Connect entry to condition
    cfg.add_edge(CfgEdge::new(entry, condition_id, "next".to_string()));

    // Create after-loop node (where false branch and breaks go)
    let after_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(after_id, "after_while".to_string()));

    // Push loop context for break/continue handling
    ctx.push_loop(condition_id, after_id);

    // Process body starting from a new ID
    let body_start_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(body_start_id, "while_body".to_string()));

    // Connect condition to body start (true branch)
    cfg.add_edge(CfgEdge::new(condition_id, body_start_id, "true".to_string()));

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