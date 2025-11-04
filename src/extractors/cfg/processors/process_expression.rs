use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::{CfgContext, get_statement_text};
use super::super::branches::process_if;
use super::super::statements::{process_while, process_for, process_break, process_continue, process_match};
use tree_sitter::Node;

/// Handle expression statements which may contain control flow expressions.
pub fn handle_expression_statement(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    stmt: Node,
    source: &str,
    current: usize,
) -> Option<usize> {
    let mut cursor = stmt.walk();

    // Check if it contains special expressions
    for child in stmt.named_children(&mut cursor) {
        match child.kind() {
            "if_expression" => {
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
            "while_expression" => {
                let exits = process_while(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return Some(usize::MAX); // Signal termination
                } else if !exits.is_empty() {
                    return Some(exits[0]);
                }
                return Some(current);
            }
            "for_expression" => {
                let (exits, _for_ir) = process_for(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return Some(usize::MAX); // Signal termination
                } else if !exits.is_empty() {
                    return Some(exits[0]);
                }
                return Some(current);
            }
            "match_expression" => {
                let exits = process_match(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return Some(usize::MAX); // Signal termination
                } else if !exits.is_empty() {
                    return Some(exits[0]);
                }
                return Some(current);
            }
            "break_expression" => {
                let _exits = process_break(cfg, ctx, child, source, current);
                return Some(usize::MAX); // Signal termination (break stops execution)
            }
            "continue_expression" => {
                let _exits = process_continue(cfg, ctx, child, source, current);
                return Some(usize::MAX); // Signal termination (continue stops execution)
            }
            _ => {}
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