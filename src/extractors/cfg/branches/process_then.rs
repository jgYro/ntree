use super::super::core::{get_statement_text, is_statement_node, CfgContext};
use super::control_flow_handler::handle_control_flow_expression;
use super::nested_if_handler::{handle_expression_if, handle_nested_if};
use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
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

        // Handle control flow expressions
        if child.kind() == "if_expression" {
            handle_nested_if(cfg, ctx, child, source, cond_id, &mut current, &mut first);
            continue;
        }

        // Handle other control flow expressions
        if let Some((new_current, new_first)) =
            handle_control_flow_expression(cfg, ctx, child, source, cond_id, current, first, "true")
        {
            current = new_current;
            first = new_first;
            continue;
        } else if matches!(child.kind(), "break_expression" | "continue_expression") {
            return vec![]; // Terminated
        }

        // Check if expression_statement contains control flow
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
