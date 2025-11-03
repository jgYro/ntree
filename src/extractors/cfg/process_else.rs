use crate::models::{CfgEdge, CfgNode, ControlFlowGraph};
use super::cfg_utils::{get_statement_text, is_statement_node};
use super::cfg_context::CfgContext;
use super::process_if::process_if;
use tree_sitter::Node;

/// Process else branch.
pub fn process_else_branch(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    else_node: Node,
    source: &str,
    cond_id: usize,
) -> Vec<usize> {
    let mut cursor = else_node.walk();

    for child in else_node.named_children(&mut cursor) {
        match child.kind() {
            "block" => {
                return process_else_block(cfg, ctx, child, source, cond_id);
            }
            "if_expression" => {
                return process_else_if(cfg, ctx, child, source, cond_id);
            }
            _ => {}
        }
    }

    vec![]
}

/// Process regular else block.
fn process_else_block(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    block: Node,
    source: &str,
    cond_id: usize,
) -> Vec<usize> {
    let mut block_cursor = block.walk();
    let mut first = true;
    let mut current = cond_id;

    for stmt in block.named_children(&mut block_cursor) {
        if !is_statement_node(stmt) {
            continue;
        }

        let text = get_statement_text(stmt, source);
        let node_id = ctx.alloc_id();
        cfg.add_node(CfgNode::new(node_id, text.clone()));

        if first {
            cfg.add_edge(CfgEdge::new(cond_id, node_id, "false".to_string()));
            first = false;
        } else {
            cfg.add_edge(CfgEdge::new(current, node_id, "next".to_string()));
        }

        // Check for return
        if stmt.kind() == "return_expression" || text.starts_with("return") {
            cfg.add_edge(CfgEdge::new(node_id, ctx.exit_id, "exit".to_string()));
            return vec![]; // Terminated
        }

        current = node_id;
    }

    if first {
        vec![cond_id] // Empty else
    } else {
        vec![current]
    }
}

/// Process else-if.
fn process_else_if(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    if_node: Node,
    source: &str,
    cond_id: usize,
) -> Vec<usize> {
    // else if - treat as nested if
    let else_entry = ctx.alloc_id();
    cfg.add_node(CfgNode::new(else_entry, "else-entry".to_string()));
    cfg.add_edge(CfgEdge::new(cond_id, else_entry, "false".to_string()));

    process_if(cfg, ctx, if_node, source, else_entry)
}