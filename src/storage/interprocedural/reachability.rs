use super::types::{EntryPoint, InterproceduralEdge, InterproceduralEdgeKind, ReachabilityInfo};
use crate::core::NTreeError;
use crate::models::ControlFlowGraph;
use crate::storage::SymbolId;
use std::collections::{HashMap, HashSet, VecDeque};

/// Reachability analyzer for program-level CFG computation.
#[derive(Debug)]
pub struct ReachabilityAnalyzer {
    entry_points: Vec<EntryPoint>,
    reachability: HashMap<SymbolId, ReachabilityInfo>,
}

impl ReachabilityAnalyzer {
    /// Create new reachability analyzer.
    pub fn new() -> Self {
        ReachabilityAnalyzer {
            entry_points: Vec::new(),
            reachability: HashMap::new(),
        }
    }

    /// Initialize reachability info for a function.
    pub fn add_function(&mut self, symbol_id: SymbolId) {
        self.reachability
            .insert(symbol_id.clone(), ReachabilityInfo::new(symbol_id));
    }

    /// Add entry point for program-level analysis.
    pub fn add_entry_point(
        &mut self,
        sym_id: SymbolId,
        reason: String,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
    ) -> Result<(), NTreeError> {
        let entry_node = match self.get_function_entry_exit(&sym_id, function_cfgs) {
            Some((entry, _)) => entry,
            None => {
                return Err(NTreeError::InvalidInput(format!(
                    "No CFG found for entry function: {:?}",
                    sym_id
                )))
            }
        };

        let entry_point = EntryPoint::new(sym_id, reason, entry_node);
        self.entry_points.push(entry_point);
        Ok(())
    }

    /// Compute reachability from all entry points.
    pub fn compute_reachability(
        &mut self,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
        interprocedural_edges: &[InterproceduralEdge],
    ) -> Result<(), NTreeError> {
        for reachability in self.reachability.values_mut() {
            reachability.reachable = false;
            reachability.reachable_nodes.clear();
            reachability.reached_from.clear();
        }

        let entry_points = self.entry_points.clone();
        for entry in &entry_points {
            self.mark_reachable_from_entry(entry, function_cfgs, interprocedural_edges)?;
        }

        Ok(())
    }

    /// Mark all reachable functions and nodes from an entry point.
    fn mark_reachable_from_entry(
        &mut self,
        entry: &EntryPoint,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
        interprocedural_edges: &[InterproceduralEdge],
    ) -> Result<(), NTreeError> {
        let mut visited_functions: HashSet<SymbolId> = HashSet::new();
        let mut visited_nodes: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<(SymbolId, usize)> = VecDeque::new();

        queue.push_back((entry.sym_id.clone(), entry.entry_node));

        while let Some((current_sym, current_node)) = queue.pop_front() {
            if visited_nodes.contains(&current_node) {
                continue;
            }
            visited_nodes.insert(current_node);

            if !visited_functions.contains(&current_sym) {
                visited_functions.insert(current_sym.clone());
                if let Some(reachability) = self.reachability.get_mut(&current_sym) {
                    reachability.mark_reachable_from(entry.sym_id.clone());
                }
            }

            if let Some(reachability) = self.reachability.get_mut(&current_sym) {
                reachability.add_reachable_node(current_node);
            }

            if let Some(cfg) = function_cfgs.get(&current_sym) {
                for edge in &cfg.edges {
                    if edge.from == current_node {
                        queue.push_back((current_sym.clone(), edge.to));
                    }
                }
            }

            for interproc_edge in interprocedural_edges {
                if interproc_edge.from_node == current_node
                    && interproc_edge.kind == InterproceduralEdgeKind::Call
                {
                    if let Some(callee_sym) = &interproc_edge.callee_sym {
                        queue.push_back((callee_sym.clone(), interproc_edge.to_node));
                    }
                }
            }
        }

        Ok(())
    }

    fn get_function_entry_exit(
        &self,
        function_sym: &SymbolId,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
    ) -> Option<(usize, Vec<usize>)> {
        if let Some(cfg) = function_cfgs.get(function_sym) {
            if cfg.nodes.is_empty() {
                return None;
            }

            let entry = cfg.nodes[0].cfg_node;
            let mut exits = Vec::new();
            let has_outgoing: HashSet<usize> = cfg.edges.iter().map(|e| e.from).collect();

            for node in &cfg.nodes {
                let node_id = node.cfg_node;
                if !has_outgoing.contains(&node_id)
                    || node.label.contains("return")
                    || node.label.contains("exit")
                {
                    exits.push(node_id);
                }
            }

            if exits.is_empty() {
                exits.push(cfg.nodes.last().unwrap().cfg_node);
            }

            return Some((entry, exits));
        }
        None
    }

    pub fn get_entry_points(&self) -> &[EntryPoint] {
        &self.entry_points
    }

    pub fn get_reachability(&self) -> &HashMap<SymbolId, ReachabilityInfo> {
        &self.reachability
    }
}
