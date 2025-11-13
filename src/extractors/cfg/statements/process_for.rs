use super::super::core::CfgContext;
use super::for_cfg_builder::{build_for_loop_cfg, extract_for_body};
use crate::analyzers::ForLoopNormalizer;
use crate::models::{ControlFlowGraph, ForLoopIR};
use tree_sitter::Node;

/// Process a for expression and return exit points with IR normalization.
/// Implements CFG-FOR-AG-01: Normalize for kinds (counter vs iterator).
pub fn process_for(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    for_node: Node,
    source: &str,
    entry: usize,
) -> (Vec<usize>, Option<ForLoopIR>) {
    // Generate unique loop ID
    let loop_id = format!("L{}", ctx.next_id);

    // Normalize for loop using language-agnostic analyzer
    let for_ir = ForLoopNormalizer::auto_detect_and_normalize(for_node, source, loop_id);

    // Get loop body
    let body = extract_for_body(for_node);

    match (body, &for_ir) {
        (Some(body_node), Some(ir)) => {
            let exits = build_for_loop_cfg(cfg, ctx, for_node, body_node, source, entry, ir);
            (exits, for_ir)
        }
        _ => (vec![entry], for_ir), // No body or analysis failed
    }
}
