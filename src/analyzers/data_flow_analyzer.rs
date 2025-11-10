use crate::core::NTreeError;
use crate::models::{
    ControlFlowGraph, DataFlowGraph, DataFlowNode, DataDependencyEdge,
    DependencyType, VariableDefinition,
};
use std::collections::{HashMap, HashSet};

/// Performs data flow analysis using reaching definitions algorithm.
pub struct DataFlowAnalyzer {
    /// Current function being analyzed
    current_function: String,
    /// Generated data flow graph
    data_flow_graph: DataFlowGraph,
    /// Reaching definitions for each node
    reaching_definitions: HashMap<String, HashSet<VariableDefinition>>,
}

impl DataFlowAnalyzer {
    /// Create a new data flow analyzer.
    pub fn new() -> Self {
        DataFlowAnalyzer {
            current_function: String::new(),
            data_flow_graph: DataFlowGraph::new(String::new()),
            reaching_definitions: HashMap::new(),
        }
    }

    /// Analyze data flow for a function CFG.
    pub fn analyze_function(&mut self, function_name: &str, cfg: &ControlFlowGraph) -> Result<DataFlowGraph, NTreeError> {
        self.current_function = function_name.to_string();
        self.data_flow_graph = DataFlowGraph::new(function_name.to_string());
        self.reaching_definitions.clear();

        // Build data flow nodes from CFG
        self.build_data_flow_nodes(cfg)?;

        // Compute reaching definitions
        self.compute_reaching_definitions(cfg)?;

        // Generate data dependency edges
        self.generate_dependency_edges()?;

        Ok(self.data_flow_graph.clone())
    }

    /// Build data flow nodes from CFG nodes.
    fn build_data_flow_nodes(&mut self, cfg: &ControlFlowGraph) -> Result<(), NTreeError> {
        for cfg_node in &cfg.nodes {
            let mut data_node = DataFlowNode::new(
                cfg_node.cfg_node.to_string(),
                cfg_node.label.clone(),
                format!("{}:{}-{}:{}", 1, 1, 1, 1), // Default span since CFG doesn't have spans
                1, // Default line number
            );

            // Extract variable definitions and uses from the statement
            let (definitions, uses) = self.extract_def_use_from_statement(&cfg_node.label);

            for def in definitions {
                data_node.add_definition(def);
            }

            for use_var in uses {
                data_node.add_use(use_var);
            }

            self.data_flow_graph.add_node(data_node);
        }

        Ok(())
    }

    /// Compute reaching definitions using iterative algorithm.
    fn compute_reaching_definitions(&mut self, cfg: &ControlFlowGraph) -> Result<(), NTreeError> {
        // Initialize reaching definitions
        for node in &cfg.nodes {
            self.reaching_definitions.insert(node.cfg_node.to_string(), HashSet::new());
        }

        // Find entry node
        let entry_node = cfg.nodes.iter()
            .find(|n| n.label.contains("ENTRY"))
            .or_else(|| cfg.nodes.first());

        let entry_node = match entry_node {
            Some(node) => node,
            None => return Ok(()), // Empty CFG
        };

        let mut changed = true;
        while changed {
            changed = false;

            for node in &cfg.nodes {
                let mut new_definitions = HashSet::new();

                // Collect definitions from predecessors
                for edge in &cfg.edges {
                    if edge.to == node.cfg_node {
                        if let Some(pred_defs) = self.reaching_definitions.get(&edge.from.to_string()) {
                            new_definitions.extend(pred_defs.clone());
                        }
                    }
                }

                // Add definitions from this node
                let node_id_str = node.cfg_node.to_string();
                if let Some(data_node) = self.data_flow_graph.nodes.get(&node_id_str) {
                    for def_var in &data_node.definitions {
                        let var_def = VariableDefinition {
                            variable: def_var.clone(),
                            definition_site: node_id_str.clone(),
                            span: format!("{}:{}-{}:{}", 1, 1, 1, 1), // Default span
                            is_initial: node.cfg_node == entry_node.cfg_node,
                        };
                        new_definitions.insert(var_def);
                    }
                }

                // Check if changed
                if let Some(current_defs) = self.reaching_definitions.get(&node_id_str) {
                    if &new_definitions != current_defs {
                        changed = true;
                    }
                }

                self.reaching_definitions.insert(node_id_str, new_definitions);
            }
        }

        // Set reaching definitions in data flow graph
        for (node_id, definitions) in &self.reaching_definitions {
            self.data_flow_graph.set_reaching_definitions(
                node_id.clone(),
                definitions.iter().cloned().collect(),
            );
        }

        Ok(())
    }

    /// Generate data dependency edges between nodes.
    fn generate_dependency_edges(&mut self) -> Result<(), NTreeError> {
        for (node_id, data_node) in &self.data_flow_graph.nodes.clone() {
            // For each variable use in this node
            for use_var in &data_node.uses {
                // Find reaching definitions for this variable
                if let Some(reaching_defs) = self.reaching_definitions.get(node_id) {
                    for def in reaching_defs {
                        if def.variable == *use_var && def.definition_site != *node_id {
                            let edge = DataDependencyEdge {
                                from: def.definition_site.clone(),
                                to: node_id.clone(),
                                variable: use_var.clone(),
                                dependency_type: DependencyType::TrueDependency,
                                span: data_node.span.clone(),
                            };
                            self.data_flow_graph.add_edge(edge);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract variable definitions and uses from a statement.
    fn extract_def_use_from_statement(&self, statement: &str) -> (Vec<String>, Vec<String>) {
        let mut definitions = Vec::new();
        let mut uses = Vec::new();

        // Simple pattern matching for common patterns
        if statement.contains(" = ") {
            // Assignment: "x = y + z"
            let parts: Vec<&str> = statement.split(" = ").collect();
            if parts.len() == 2 {
                let lhs = parts[0].trim();
                let rhs = parts[1].trim();

                // Left side is definition
                if let Some(var) = self.extract_variable_name(lhs) {
                    definitions.push(var);
                }

                // Right side contains uses
                uses.extend(self.extract_variables_from_expression(rhs));
            }
        } else if statement.contains("let ") {
            // Variable declaration: "let x = 5;"
            if let Some(var) = self.extract_let_variable(statement) {
                definitions.push(var.clone());

                // Check for initialization
                if statement.contains(" = ") {
                    let parts: Vec<&str> = statement.split(" = ").collect();
                    if parts.len() > 1 {
                        uses.extend(self.extract_variables_from_expression(parts[1]));
                    }
                }
            }
        } else {
            // Expression or other statement - extract all variables as uses
            uses.extend(self.extract_variables_from_expression(statement));
        }

        (definitions, uses)
    }

    /// Extract variable name from left-hand side of assignment.
    fn extract_variable_name(&self, lhs: &str) -> Option<String> {
        // Handle simple identifiers and basic patterns
        let cleaned = lhs.trim().replace(";", "");
        if cleaned.chars().all(|c| c.is_alphanumeric() || c == '_') && !cleaned.is_empty() {
            Some(cleaned)
        } else {
            None
        }
    }

    /// Extract variable from let statement.
    fn extract_let_variable(&self, statement: &str) -> Option<String> {
        if let Some(start) = statement.find("let ") {
            let after_let = &statement[start + 4..];
            if let Some(end) = after_let.find([' ', '=', ';']) {
                let var_name = after_let[..end].trim();
                if var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(var_name.to_string());
                }
            }
        }
        None
    }

    /// Extract variables from an expression.
    fn extract_variables_from_expression(&self, expression: &str) -> Vec<String> {
        let mut variables = Vec::new();
        let cleaned = expression.replace([';', '(', ')', '+', '-', '*', '/', '&', '|', '!'], " ");

        for word in cleaned.split_whitespace() {
            let word = word.trim();
            // Skip keywords, literals, and operators
            if !word.is_empty() &&
               !word.chars().all(|c| c.is_ascii_digit()) && // Not a number
               !matches!(word, "true" | "false" | "if" | "else" | "while" | "for" | "return") &&
               word.chars().all(|c| c.is_alphanumeric() || c == '_') {
                variables.push(word.to_string());
            }
        }

        variables
    }

    /// Extract line number from span string.
    fn extract_line_number(&self, span: &str) -> u32 {
        span.split(':').next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}