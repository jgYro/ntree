use super::super::core::CfgContext;
use super::super::statements::{process_break, process_continue, process_match, process_while};
use crate::models::ControlFlowGraph;
use tree_sitter::Node;

/// Handle control flow expressions in branches with proper edge connections.
pub fn handle_control_flow_expression(
    cfg: &mut ControlFlowGraph,
    ctx: &mut CfgContext,
    stmt: Node,
    source: &str,
    cond_id: usize,
    current: usize,
    first: bool,
    _edge_type: &str, // "true" for then branch, "false" for else branch
) -> Option<(usize, bool)> {
    let entry = if first { cond_id } else { current };

    match stmt.kind() {
        "break_expression" => {
            let _exits = process_break(cfg, ctx, stmt, source, entry);
            None // Terminated by break
        }
        "continue_expression" => {
            let _exits = process_continue(cfg, ctx, stmt, source, entry);
            None // Terminated by continue
        }
        "while_expression" => {
            let exits = process_while(cfg, ctx, stmt, source, entry);
            if exits.is_empty() {
                None // Terminated
            } else {
                Some((exits[0], false))
            }
        }
        "match_expression" => {
            let exits = process_match(cfg, ctx, stmt, source, entry);
            if exits.is_empty() {
                None // Terminated
            } else {
                Some((exits[0], false))
            }
        }
        _ => None, // Not a control flow expression
    }
}
