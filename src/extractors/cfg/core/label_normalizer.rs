use crate::models::{ForLoopIR, LoopKind};

/// Generate language-agnostic CFG node labels for control flow constructs.
pub struct LabelNormalizer;

impl LabelNormalizer {
    /// Generate normalized label for a for loop based on its IR.
    pub fn for_loop_label(for_ir: &ForLoopIR) -> String {
        match for_ir.kind {
            LoopKind::ForCounter => {
                let init = for_ir.init.as_deref().unwrap_or("init");
                let condition = for_ir.condition.as_deref().unwrap_or("condition");
                let update = for_ir.update.as_deref().unwrap_or("update");
                format!(
                    "for_loop(init: {}, cond: {}, update: {})",
                    init, condition, update
                )
            }
            LoopKind::ForIterator => {
                let pattern = for_ir.pattern.as_deref().unwrap_or("item");
                let iter_expr = for_ir.iter_expr.as_deref().unwrap_or("iterator");
                format!(
                    "for_loop(cond: {}.has_next, pattern: {})",
                    iter_expr, pattern
                )
            }
        }
    }

    /// Generate normalized label for a while loop.
    pub fn while_loop_label(condition: &str) -> String {
        format!("while_loop(cond: {})", condition)
    }

    /// Generate normalized label for a match expression.
    pub fn match_label(expr: &str) -> String {
        format!("match_expr(value: {})", expr)
    }

    /// Generate normalized label for loop control statements.
    pub fn break_label() -> String {
        "break_stmt".to_string()
    }

    pub fn continue_label() -> String {
        "continue_stmt".to_string()
    }

    /// Generate normalized labels for loop structural nodes.
    pub fn loop_body_label(loop_type: &str) -> String {
        format!("{}_body", loop_type)
    }

    pub fn loop_after_label(loop_type: &str) -> String {
        format!("after_{}", loop_type)
    }

    /// Generate normalized label for match arms.
    pub fn match_arm_label(pattern: &str) -> String {
        format!("match_arm(pattern: {})", pattern)
    }

    /// Generate normalized label for control flow joins.
    pub fn join_label(join_type: &str) -> String {
        format!("{}_join", join_type)
    }
}
