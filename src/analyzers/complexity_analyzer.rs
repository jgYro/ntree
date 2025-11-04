use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::models::ir::FunctionCFGIR;

/// Complexity and reachability analysis result for a function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComplexityResult {
    /// Function name being analyzed
    pub function: String,
    /// Cyclomatic complexity (E - N + 2)
    pub cyclomatic: u32,
    /// List of unreachable node IDs
    pub unreachable: Vec<String>,
}

/// Analyzer for computing cyclomatic complexity and detecting unreachable nodes.
pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    /// Create a new complexity analyzer.
    pub fn new() -> Self {
        ComplexityAnalyzer
    }

    /// Analyze a function's CFG for complexity and reachability.
    pub fn analyze(&self, cfg: &FunctionCFGIR) -> Result<ComplexityResult, String> {
        let node_count = cfg.node_count() as u32;
        let edge_count = cfg.edge_count() as u32;

        if node_count == 0 {
            return Ok(ComplexityResult {
                function: cfg.function_name.clone(),
                cyclomatic: 1,
                unreachable: Vec::new(),
            });
        }

        // Calculate cyclomatic complexity: E - N + 2
        let cyclomatic = if edge_count >= node_count {
            edge_count - node_count + 2
        } else {
            1 // Minimum complexity for disconnected graphs
        };

        // Find unreachable nodes using DFS from ENTRY
        let unreachable = self.find_unreachable_nodes(cfg)?;

        Ok(ComplexityResult {
            function: cfg.function_name.clone(),
            cyclomatic,
            unreachable,
        })
    }

    /// Find unreachable nodes using DFS traversal from ENTRY node.
    fn find_unreachable_nodes(&self, cfg: &FunctionCFGIR) -> Result<Vec<String>, String> {
        // Build adjacency list from edges
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &cfg.edges {
            adjacency.entry(edge.from.clone()).or_insert_with(Vec::new).push(edge.to.clone());
        }

        // Find ENTRY node or the first node if no explicit ENTRY
        let entry_node = self.find_entry_node(cfg)?;

        // Perform DFS to find all reachable nodes
        let mut visited = HashSet::new();
        let mut stack = vec![entry_node];

        while let Some(current) = stack.pop() {
            if visited.insert(current.clone()) {
                if let Some(neighbors) = adjacency.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            stack.push(neighbor.clone());
                        }
                    }
                }
            }
        }

        // Collect unreachable nodes
        let mut unreachable = Vec::new();
        for node in &cfg.nodes {
            if !visited.contains(&node.id) {
                unreachable.push(node.id.clone());
            }
        }

        unreachable.sort();
        Ok(unreachable)
    }

    /// Find the ENTRY node or return the first node.
    fn find_entry_node(&self, cfg: &FunctionCFGIR) -> Result<String, String> {
        // Look for node with "ENTRY" label or similar
        for node in &cfg.nodes {
            if node.label.to_uppercase().contains("ENTRY") {
                return Ok(node.id.clone());
            }
        }

        // If no explicit ENTRY, use the first node
        match cfg.nodes.first() {
            Some(node) => Ok(node.id.clone()),
            None => Err("No nodes found in CFG".to_string()),
        }
    }
}

impl Default for ComplexityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}