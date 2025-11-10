use crate::core::NTreeError;
use crate::models::{
    VariableLifecycle, VariableEvent, VariableEventType, VariableScope, VariableLifecycleSet,
    ControlFlowGraph, DataFlowGraph,
};
use std::collections::{HashMap, HashSet};

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
        self.enhance_with_data_flow(data_flow)?;

        // Determine liveness at function exit
        self.compute_liveness_at_exit(cfg)?;

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
    fn process_node_for_variables(&mut self, node_id: &str, statement: &str, span: &str) -> Result<(), NTreeError> {
        let line = self.extract_line_number(span);
        let column = self.extract_column_number(span);

        // Handle variable definitions
        if let Some((var_name, event_type)) = self.extract_definition(statement) {
            let definition_event = VariableEvent {
                span: span.to_string(),
                event_type,
                context: node_id.to_string(),
                line,
                column,
            };

            let current_scope = self.scope_stack.last().unwrap().clone();
            let lifecycle = VariableLifecycle::new(var_name.clone(), definition_event, current_scope);

            // Try to infer type
            let enhanced_lifecycle = match self.infer_variable_type(statement) {
                Some(var_type) => lifecycle.with_type(var_type),
                None => lifecycle,
            };

            self.lifecycles.insert(var_name, enhanced_lifecycle);
        }

        // Handle variable uses
        let used_vars = self.extract_variable_uses(statement);
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
        if let Some(mutated_var) = self.extract_mutation(statement) {
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

    /// Extract variable definition from statement.
    fn extract_definition(&self, statement: &str) -> Option<(String, VariableEventType)> {
        // Handle let declarations
        if statement.contains("let ") {
            if let Some(var_name) = self.extract_let_variable(statement) {
                return Some((var_name, VariableEventType::Definition));
            }
        }

        // Handle function parameters (simplified)
        if statement.contains("(") && statement.contains(":") {
            if let Some(param_name) = self.extract_parameter(statement) {
                return Some((param_name, VariableEventType::Definition));
            }
        }

        None
    }

    /// Extract variable uses from statement.
    fn extract_variable_uses(&self, statement: &str) -> Vec<String> {
        let mut uses = Vec::new();

        // Skip if this is a definition
        if statement.contains("let ") {
            // For "let x = y", y is a use
            if statement.contains(" = ") {
                let parts: Vec<&str> = statement.split(" = ").collect();
                if parts.len() > 1 {
                    uses.extend(self.extract_variables_from_expression(parts[1]));
                }
            }
        } else if statement.contains(" = ") {
            // For "x = y", y is a use
            let parts: Vec<&str> = statement.split(" = ").collect();
            if parts.len() > 1 {
                uses.extend(self.extract_variables_from_expression(parts[1]));
            }
        } else {
            // Other expressions - all variables are uses
            uses.extend(self.extract_variables_from_expression(statement));
        }

        uses
    }

    /// Extract variable mutation from statement.
    fn extract_mutation(&self, statement: &str) -> Option<String> {
        // Handle reassignments (not let declarations)
        if statement.contains(" = ") && !statement.contains("let ") {
            let parts: Vec<&str> = statement.split(" = ").collect();
            if !parts.is_empty() {
                let lhs = parts[0].trim().replace(";", "");
                if lhs.chars().all(|c| c.is_alphanumeric() || c == '_') && !lhs.is_empty() {
                    return Some(lhs);
                }
            }
        }

        None
    }

    /// Extract variable name from let statement.
    fn extract_let_variable(&self, statement: &str) -> Option<String> {
        if let Some(start) = statement.find("let ") {
            let after_let = &statement[start + 4..];
            if let Some(end) = after_let.find([' ', '=', ';', ':']) {
                let var_name = after_let[..end].trim();
                if var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(var_name.to_string());
                }
            }
        }
        None
    }

    /// Extract parameter name (simplified).
    fn extract_parameter(&self, statement: &str) -> Option<String> {
        // Very basic parameter extraction - would need language-specific implementation
        if statement.contains("(") && statement.contains(":") {
            // Look for patterns like "fn name(param: type)"
            if let Some(start) = statement.find('(') {
                if let Some(end) = statement.find(':') {
                    if end > start {
                        let param_text = &statement[start + 1..end].trim();
                        if param_text.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            return Some(param_text.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract variables from expression.
    fn extract_variables_from_expression(&self, expression: &str) -> Vec<String> {
        let mut variables = Vec::new();
        let cleaned = expression.replace([';', '(', ')', '+', '-', '*', '/', '&', '|', '!', '<', '>', '='], " ");

        for word in cleaned.split_whitespace() {
            let word = word.trim();
            if !word.is_empty() &&
               !word.chars().all(|c| c.is_ascii_digit()) &&
               !matches!(word, "true" | "false" | "if" | "else" | "while" | "for" | "return" | "mut") &&
               word.chars().all(|c| c.is_alphanumeric() || c == '_') {
                variables.push(word.to_string());
            }
        }

        variables
    }

    /// Enhance lifecycle analysis with data flow information.
    fn enhance_with_data_flow(&mut self, data_flow: &DataFlowGraph) -> Result<(), NTreeError> {
        for lifecycle in self.lifecycles.values_mut() {
            // Check if variable has data dependencies
            let deps = data_flow.get_variable_dependencies(&lifecycle.name);
            if !deps.is_empty() {
                // Variable has data dependencies - mark as used
                lifecycle.live_at_exit = true;
            }
        }

        Ok(())
    }

    /// Compute liveness at function exit.
    fn compute_liveness_at_exit(&mut self, cfg: &ControlFlowGraph) -> Result<(), NTreeError> {
        // Find exit nodes
        let mut exit_nodes = HashSet::new();
        for node in &cfg.nodes {
            if node.label.contains("EXIT") || node.label.contains("return") {
                exit_nodes.insert(node.cfg_node);
            }
        }

        // If no explicit exit nodes, use nodes with no outgoing edges
        if exit_nodes.is_empty() {
            for node in &cfg.nodes {
                let has_outgoing = cfg.edges.iter().any(|edge| edge.from == node.cfg_node);
                if !has_outgoing {
                    exit_nodes.insert(node.cfg_node);
                }
            }
        }

        // Mark variables as live if used in exit nodes
        for exit_node in exit_nodes {
            if let Some(node) = cfg.nodes.iter().find(|n| n.cfg_node == exit_node) {
                let used_vars = self.extract_variable_uses(&node.label);
                for var_name in used_vars {
                    if let Some(lifecycle) = self.lifecycles.get_mut(&var_name) {
                        lifecycle.live_at_exit = true;
                    }
                }
            }
        }

        Ok(())
    }

    /// Infer variable type from statement.
    fn infer_variable_type(&self, statement: &str) -> Option<String> {
        // Simple type inference
        if statement.contains(": i32") {
            Some("i32".to_string())
        } else if statement.contains(": String") {
            Some("String".to_string())
        } else if statement.contains(": bool") {
            Some("bool".to_string())
        } else if statement.contains(" = \"") {
            Some("String".to_string())
        } else if statement.contains(" = true") || statement.contains(" = false") {
            Some("bool".to_string())
        } else if statement.contains(" = ") {
            let parts: Vec<&str> = statement.split(" = ").collect();
            if parts.len() > 1 && parts[1].trim().chars().all(|c| c.is_ascii_digit()) {
                Some("i32".to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Extract line number from span.
    fn extract_line_number(&self, span: &str) -> u32 {
        span.split(':').next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    /// Extract column number from span.
    fn extract_column_number(&self, span: &str) -> u32 {
        span.split(':').nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}