use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::cfg_context::CfgContext;
use super::cfg_utils::{get_statement_text, is_statement_node};
use super::cfg_branches::process_if;
use tree_sitter::Node;

/// Handle expression statements which may contain if expressions.
pub fn handle_expression_statement(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    stmt: Node,
    source: &str,
    current: usize,
) -> Option<usize> {
    let mut cursor = stmt.walk();

    // Check if it contains an if_expression
    for child in stmt.named_children(&mut cursor) {
        if child.kind() == "if_expression" {
            let exits = process_if(cfg, ctx, child, source, current);
            if exits.is_empty() {
                return Some(usize::MAX); // Signal termination
            }
            if exits.len() > 1 || (exits.len() == 1 && exits[0] != current) {
                let join_id = ctx.alloc_id();
                cfg.add_node(CfgNode::new(join_id, "join".to_string()));
                for exit in &exits {
                    if *exit != join_id {
                        cfg.add_edge(CfgEdge::new(*exit, join_id, "next".to_string()));
                    }
                }
                return Some(join_id);
            } else if !exits.is_empty() {
                return Some(exits[0]);
            }
            return Some(current);
        }
    }

    // Check for return
    let text = get_statement_text(stmt, source);
    if text.starts_with("return") {
        let node_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(node_id, text));
        cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
        cfg.add_edge(CfgEdge::new(node_id, ctx.exit_id, "exit".to_string()));
        return Some(usize::MAX); // Signal termination
    } else if text.starts_with("if") {
        // Already handled above
        return None;
    } else {
        // Regular expression statement
        let node_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(node_id, text));
        cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
        return Some(node_id);
    }
}