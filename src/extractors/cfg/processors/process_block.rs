use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::super::core::{CfgContext, get_statement_text, is_statement_node};
use super::super::branches::process_if;
use super::process_expression::handle_expression_statement;
use super::super::statements::{process_while, process_break, process_continue, process_match};
use tree_sitter::Node;

/// Process a block and return exit points.
pub fn process_block(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    block: Node,
    source: &str,
    entry: usize,
) -> Vec<usize> {
    let mut current = entry;
    let mut cursor = block.walk();

    for child in block.named_children(&mut cursor) {
        if !is_statement_node(child) {
            continue;
        }

        match child.kind() {
            "if_expression" => {
                let exits = process_if(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return vec![];
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
                    current = join_id;
                } else if !exits.is_empty() {
                    current = exits[0];
                }
            }
            "while_expression" => {
                let exits = process_while(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return vec![];
                } else if !exits.is_empty() {
                    current = exits[0];
                }
            }
            "match_expression" => {
                let exits = process_match(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return vec![];
                } else if !exits.is_empty() {
                    current = exits[0];
                }
            }
            "break_expression" => {
                let exits = process_break(cfg, ctx, child, source, current);
                return exits; // Path terminated
            }
            "continue_expression" => {
                let exits = process_continue(cfg, ctx, child, source, current);
                return exits; // Path terminated
            }
            "return_expression" => {
                let text = get_statement_text(child, source);
                let node_id = ctx.alloc_id();
                cfg.add_node(CfgNode::new(node_id, text));
                cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
                cfg.add_edge(CfgEdge::new(node_id, ctx.exit_id, "exit".to_string()));
                return vec![]; // Path terminated
            }
            _ => {
                // Check for special cases in expression_statement
                if child.kind() == "expression_statement" {
                    if let Some(new_current) = handle_expression_statement(cfg, ctx, child, source, current) {
                        if new_current == usize::MAX {
                            return vec![];
                        }
                        current = new_current;
                    }
                } else {
                    // Regular statement
                    let text = get_statement_text(child, source);
                    let node_id = ctx.alloc_id();
                    cfg.add_node(CfgNode::new(node_id, text));
                    cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
                    current = node_id;
                }
            }
        }
    }

    vec![current]
}