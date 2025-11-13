use super::call_edge::{CallConfidence, CallEdge};
use super::symbol_core::SymbolId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Call graph table mapping callers to their call sites.
#[derive(Debug, Serialize, Deserialize)]
pub struct CallGraph {
    /// caller_sym -> list of call edges
    call_edges: HashMap<SymbolId, Vec<CallEdge>>,
}

impl CallGraph {
    /// Create new call graph.
    pub fn new() -> Self {
        CallGraph {
            call_edges: HashMap::new(),
        }
    }

    /// Add a call edge to the graph.
    pub fn add_call_edge(&mut self, edge: CallEdge) {
        self.call_edges
            .entry(edge.caller_sym.clone())
            .or_insert_with(Vec::new)
            .push(edge);
    }

    /// Get all call sites for a function.
    pub fn get_call_sites(&self, caller: &SymbolId) -> Vec<&CallEdge> {
        match self.call_edges.get(caller) {
            Some(edges) => edges.iter().collect(),
            None => Vec::new(),
        }
    }

    /// Get all functions that call a specific target.
    pub fn get_callers(&self, target: &SymbolId) -> Vec<&CallEdge> {
        self.call_edges
            .values()
            .flat_map(|edges| edges.iter())
            .filter(|edge| edge.targets.contains(target))
            .collect()
    }

    /// Get call graph statistics.
    pub fn stats(&self) -> CallGraphStats {
        let mut stats = CallGraphStats {
            total_call_sites: 0,
            direct_calls: 0,
            virtual_calls: 0,
            dynamic_calls: 0,
            unresolved_calls: 0,
        };

        for edges in self.call_edges.values() {
            stats.total_call_sites += edges.len();
            for edge in edges {
                match edge.confidence {
                    CallConfidence::Direct => stats.direct_calls += 1,
                    CallConfidence::Virtual => stats.virtual_calls += 1,
                    CallConfidence::Dynamic => stats.dynamic_calls += 1,
                    CallConfidence::Unknown => stats.unresolved_calls += 1,
                }
            }
        }

        stats
    }

    /// Get all call edges as iterator.
    pub fn all_call_edges(&self) -> impl Iterator<Item = &CallEdge> {
        self.call_edges.values().flat_map(|edges| edges.iter())
    }

    /// Get all call edges as a vector.
    pub fn get_all_call_edges(&self) -> Vec<&CallEdge> {
        self.all_call_edges().collect()
    }
}

/// Call graph statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraphStats {
    pub total_call_sites: usize,
    pub direct_calls: usize,
    pub virtual_calls: usize,
    pub dynamic_calls: usize,
    pub unresolved_calls: usize,
}

impl Default for CallGraph {
    fn default() -> Self {
        Self::new()
    }
}
