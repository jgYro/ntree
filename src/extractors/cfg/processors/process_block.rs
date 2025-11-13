use super::super::branches::process_if;
use super::super::core::{get_statement_text, is_statement_node, CfgContext};
use super::super::statements::{
    process_break, process_continue, process_match, process_panic_expression,
    process_try_expression,
};
use super::loop_handler::{handle_if_with_join, handle_loop_expression};
use super::process_expression::handle_expression_statement;
use crate::analyzers::language_specific::RustEarlyExitAnalyzer;
use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
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
            // Rust control flow
            "if_expression" => {
                let exits = process_if(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return vec![];
                }
                current = handle_if_with_join(cfg, ctx, exits, current);
            }
            "while_expression" | "for_expression" => {
                if let Some(new_current) = handle_loop_expression(cfg, ctx, child, source, current)
                {
                    current = new_current;
                } else {
                    return vec![]; // Loop terminated
                }
            }
            // Python control flow
            "if_statement" => {
                let exits = process_if(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return vec![];
                }
                current = handle_if_with_join(cfg, ctx, exits, current);
            }
            "try_statement" => {
                // Process try/except as branching control flow
                let try_id = ctx.alloc_id();
                cfg.add_node(CfgNode::new(try_id, "try".to_string()));
                cfg.add_edge(CfgEdge::new(current, try_id, "next".to_string()));

                let except_id = ctx.alloc_id();
                cfg.add_node(CfgNode::new(except_id, "except".to_string()));
                cfg.add_edge(CfgEdge::new(try_id, except_id, "exception".to_string()));

                // Both try and except paths merge after
                let join_id = ctx.alloc_id();
                cfg.add_node(CfgNode::new(join_id, "join".to_string()));
                cfg.add_edge(CfgEdge::new(try_id, join_id, "success".to_string()));
                cfg.add_edge(CfgEdge::new(except_id, join_id, "next".to_string()));

                current = join_id;
            }
            "while_statement" | "for_statement" => {
                if let Some(new_current) = handle_loop_expression(cfg, ctx, child, source, current)
                {
                    current = new_current;
                } else {
                    return vec![]; // Loop terminated
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
            "try_expression" => {
                let (exits, _early_exit_ir) =
                    process_try_expression(cfg, ctx, child, source, current);
                if exits.is_empty() {
                    return vec![]; // Path terminated
                } else if !exits.is_empty() {
                    current = exits[0];
                }
            }
            "macro_invocation" => {
                // Check if it's a panic! macro
                let text = get_statement_text(child, source);
                if text.starts_with("panic!") {
                    let (exits, _early_exit_ir) =
                        process_panic_expression(cfg, ctx, child, source, current);
                    return exits; // Path terminated by panic
                } else {
                    // Regular macro - treat as statement
                    let node_id = ctx.alloc_id();
                    cfg.add_node(CfgNode::new(node_id, text));
                    cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
                    current = node_id;
                }
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
                // Check for Rust early-exit constructs first
                if RustEarlyExitAnalyzer::contains_early_exit(child, source) {
                    if RustEarlyExitAnalyzer::contains_try_operator(child, source) {
                        let (exits, _early_exit_ir) =
                            process_try_expression(cfg, ctx, child, source, current);
                        if exits.is_empty() {
                            return vec![]; // Path terminated
                        } else if !exits.is_empty() {
                            current = exits[0];
                        }
                    } else if RustEarlyExitAnalyzer::is_panic_macro(child, source) {
                        let (_exits, _early_exit_ir) =
                            process_panic_expression(cfg, ctx, child, source, current);
                        return vec![]; // Path terminated by panic
                    }
                } else if child.kind() == "expression_statement" {
                    if let Some(new_current) =
                        handle_expression_statement(cfg, ctx, child, source, current)
                    {
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
