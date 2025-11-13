use super::super::core::CfgContext;
use crate::analyzers::EarlyExitNormalizer;
use crate::models::{CfgEdge, CfgNode, ControlFlowGraph, EarlyExitIR};
use tree_sitter::Node;

/// Process early-exit constructs (try operators and panic statements).
/// Implements CFG-10: Early-exit sugar (?, panic!).
pub fn process_try_expression(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    try_node: Node,
    source: &str,
    entry: usize,
) -> (Vec<usize>, Option<EarlyExitIR>) {
    // Generate unique exit ID
    let exit_id = format!("E{}", ctx.next_id);

    // Normalize early-exit using language-agnostic analyzer
    let early_exit_ir = EarlyExitNormalizer::auto_detect_and_normalize(try_node, source, exit_id);

    if let Some(ref ir) = early_exit_ir {
        let exits = process_try_expression_cfg(cfg, ctx, try_node, source, entry, ir);
        (exits, early_exit_ir)
    } else {
        // Fallback: treat as regular expression
        (vec![entry], None)
    }
}

/// Process panic expressions (unconditional early exit).
pub fn process_panic_expression(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    panic_node: Node,
    source: &str,
    entry: usize,
) -> (Vec<usize>, Option<EarlyExitIR>) {
    // Generate unique exit ID
    let exit_id = format!("E{}", ctx.next_id);

    // Normalize panic using language-agnostic analyzer
    let early_exit_ir = EarlyExitNormalizer::auto_detect_and_normalize(panic_node, source, exit_id);

    if let Some(ref ir) = early_exit_ir {
        let exits = process_panic_expression_cfg(cfg, ctx, panic_node, source, entry, ir);
        (exits, early_exit_ir)
    } else {
        // Fallback: treat as regular expression
        (vec![entry], None)
    }
}

/// Process CFG for try expression: expr? (conditional early return).
fn process_try_expression_cfg(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    _try_node: Node,
    _source: &str,
    entry: usize,
    early_exit_ir: &EarlyExitIR,
) -> Vec<usize> {
    // Create try expression node
    let try_id = ctx.alloc_id();
    let try_label = format!("try_expr({})", early_exit_ir.trigger_expr);
    cfg.add_node(CfgNode::new(try_id, try_label));
    cfg.add_edge(CfgEdge::new(entry, try_id, "next".to_string()));

    // Create continuation block for the "ok" path
    let ok_block_id = ctx.alloc_id();
    cfg.add_node(CfgNode::new(ok_block_id, "try_ok".to_string()));

    // Add two edges from try expression:
    // 1. Conditional edge to EXIT (error path)
    cfg.add_edge(CfgEdge::new(try_id, ctx.exit_id, "error".to_string()));

    // 2. Edge to next block (ok path)
    cfg.add_edge(CfgEdge::new(try_id, ok_block_id, "ok".to_string()));

    vec![ok_block_id]
}

/// Process CFG for panic expression (unconditional exit).
fn process_panic_expression_cfg(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    _panic_node: Node,
    _source: &str,
    entry: usize,
    early_exit_ir: &EarlyExitIR,
) -> Vec<usize> {
    // Create panic node
    let panic_id = ctx.alloc_id();
    let panic_label = format!("panic_expr({})", early_exit_ir.trigger_expr);
    cfg.add_node(CfgNode::new(panic_id, panic_label));
    cfg.add_edge(CfgEdge::new(entry, panic_id, "next".to_string()));

    // Add exceptional edge to EXIT (or synthetic unwind)
    cfg.add_edge(CfgEdge::new(panic_id, ctx.exit_id, "exception".to_string()));

    // No continuation - panic terminates execution
    vec![]
}
