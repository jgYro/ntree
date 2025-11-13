use super::super::branches::process_if;
use super::super::core::{get_statement_text, CfgContext};
use super::super::statements::{
    process_break, process_continue, process_for, process_match, process_panic_expression,
    process_try_expression, process_while,
};
use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
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
            "try_expression" => {
                let (exits, _early_exit_ir) =
                    process_try_expression(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return Some(usize::MAX); // Signal termination
                } else {
                    return Some(exits[0]);
                }
            }
            _ => {}
        }
    }

    // Check for special statement types
    let text = get_statement_text(stmt, source);
    if text.starts_with("return") {
        let node_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(node_id, text));
        cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
        cfg.add_edge(CfgEdge::new(node_id, ctx.exit_id, "exit".to_string()));
        return Some(usize::MAX); // Signal termination
    } else if text.starts_with("panic!") {
        // Handle panic! in expression statement
        let (_exits, _early_exit_ir) = process_panic_expression(cfg, ctx, stmt, source, current);
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
