use crate::core::NTreeError;
use crate::models::{
    ControlFlowGraph, DataFlowGraph, VariableEvent, VariableEventType, VariableLifecycle,
    VariableLifecycleSet, VariableScope,
};
use std::collections::HashMap;

use super::variable_lifecycle::{
    DataFlowIntegrator, LifecycleUtils, VariableExtractor,
};

/// Analyzes variable lifecycles through their scopes.
pub struct VariableLifecycleAnalyzer {
    /// Current function being analyzed
    current_function: String,
    /// Variable lifecycles being tracked
    lifecycles: HashMap<String, VariableLifecycle>,
    /// Variable scope stack
    scope_stack: Vec<VariableScope>,
}

impl VariableLifecycleAnalyzer {
    /// Create a new variable lifecycle analyzer.
    pub fn new() -> Self {
        VariableLifecycleAnalyzer {
            current_function: String::new(),
            lifecycles: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }

    /// Analyze variable lifecycles for a function.
    pub fn analyze_function(
        &mut self,
        function_name: &str,
        cfg: &ControlFlowGraph,
        data_flow: &DataFlowGraph,
    ) -> Result<VariableLifecycleSet, NTreeError> {
        self.current_function = function_name.to_string();
        self.lifecycles.clear();
        self.scope_stack.clear();

        // Initialize function scope
        let function_scope = VariableScope {
            function_name: function_name.to_string(),
            scope_level: 0,
            scope_start: "0:0".to_string(),
            scope_end: "999:999".to_string(),
            captured: false,
        };
        self.scope_stack.push(function_scope);

        // Process CFG nodes to track variable events
        self.process_cfg_nodes(cfg)?;

        // Use data flow information to enhance lifecycle analysis
        DataFlowIntegrator::enhance_with_data_flow(&mut self.lifecycles, data_flow)?;

        // Determine liveness at function exit
        DataFlowIntegrator::compute_liveness_at_exit(&mut self.lifecycles, cfg)?;

        // Build result set
        let mut result_set = VariableLifecycleSet::new();
        for lifecycle in self.lifecycles.values() {
            result_set.add_lifecycle(lifecycle.clone());
        }

        Ok(result_set)
    }

    /// Process CFG nodes to identify variable events.
    fn process_cfg_nodes(&mut self, cfg: &ControlFlowGraph) -> Result<(), NTreeError> {
        for node in &cfg.nodes {
            let span = format!("{}:{}-{}:{}", 1, 1, 1, 1); // Default span
            self.process_node_for_variables(&node.cfg_node.to_string(), &node.label, &span)?;
        }

        Ok(())
    }

    /// Process a single CFG node for variable events.
    fn process_node_for_variables(
        &mut self,
        node_id: &str,
        statement: &str,
        span: &str,
    ) -> Result<(), NTreeError> {
        let line = LifecycleUtils::extract_line_number(span);
        let column = LifecycleUtils::extract_column_number(span);

        // Handle variable definitions
        if let Some((var_name, event_type)) = VariableExtractor::extract_definition(statement) {
            let definition_event = VariableEvent {
                span: span.to_string(),
                event_type,
                context: node_id.to_string(),
                line,
                column,
            };

            let current_scope = self.scope_stack.last().unwrap().clone();
            let lifecycle =
                VariableLifecycle::new(var_name.clone(), definition_event, current_scope);

            // Try to infer type
            let enhanced_lifecycle = match LifecycleUtils::infer_variable_type(statement) {
                Some(var_type) => lifecycle.with_type(var_type),
                None => lifecycle,
            };

            self.lifecycles.insert(var_name, enhanced_lifecycle);
        }

        // Handle variable uses
        let used_vars = VariableExtractor::extract_variable_uses(statement);
        for var_name in used_vars {
            if let Some(lifecycle) = self.lifecycles.get_mut(&var_name) {
                let use_event = VariableEvent {
                    span: span.to_string(),
                    event_type: VariableEventType::Use,
                    context: node_id.to_string(),
                    line,
                    column,
                };
                lifecycle.add_use(use_event);
            }
        }

        // Handle variable mutations
        if let Some(mutated_var) = VariableExtractor::extract_mutation(statement) {
            if let Some(lifecycle) = self.lifecycles.get_mut(&mutated_var) {
                let mutation_event = VariableEvent {
                    span: span.to_string(),
                    event_type: VariableEventType::Mutation,
                    context: node_id.to_string(),
                    line,
                    column,
                };
                lifecycle.add_mutation(mutation_event);
            }
        }

        Ok(())
    }
}