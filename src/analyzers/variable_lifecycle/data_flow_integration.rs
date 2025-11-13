use crate::core::NTreeError;
use crate::models::{ControlFlowGraph, DataFlowGraph, VariableLifecycle};
use std::collections::HashMap;

/// Handles integration of data flow information into variable lifecycle analysis.
pub struct DataFlowIntegrator;

impl DataFlowIntegrator {
    /// Enhance variable lifecycle with data flow information.
    pub fn enhance_with_data_flow(
        _lifecycles: &mut HashMap<String, VariableLifecycle>,
        _data_flow: &DataFlowGraph,
    ) -> Result<(), NTreeError> {
        // For now, this is a placeholder that doesn't modify lifecycles
        // In a full implementation, this would correlate data flow nodes with variable events
        Ok(())
    }

    /// Compute liveness at exit points.
    pub fn compute_liveness_at_exit(
        lifecycles: &mut HashMap<String, VariableLifecycle>,
        cfg: &ControlFlowGraph,
    ) -> Result<(), NTreeError> {
        // Find exit edges and mark variables that are live at function exit
        for edge in &cfg.edges {
            if edge.to == 9999 {
                // This is an exit edge - check if variables are used in the source node
                for (_var_name, lifecycle) in lifecycles.iter_mut() {
                    // If variable has uses, mark it as potentially live at exit
                    if !lifecycle.uses.is_empty() {
                        lifecycle.live_at_exit = true;
                    }
                }
            }
        }

        // Mark variables that are only defined but never used as dead
        for lifecycle in lifecycles.values_mut() {
            if lifecycle.uses.is_empty() && lifecycle.mutations.is_empty() {
                // Variable is defined but never used - not live at exit
                lifecycle.live_at_exit = false;
            }
        }

        Ok(())
    }
}