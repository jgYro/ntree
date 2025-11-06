use std::collections::{HashMap, HashSet};
use super::module_graph::ModuleId;

/// Cycle detection and topological sorting utilities.
pub struct CycleDetector;

impl CycleDetector {
    /// Detect cycles using DFS.
    pub fn detect_cycles(
        adjacency: &HashMap<ModuleId, Vec<ModuleId>>,
        modules: &HashMap<ModuleId, super::module_graph::Module>,
    ) -> Vec<Vec<ModuleId>> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut cycles = Vec::new();

        for module_id in modules.keys() {
            if !visited.contains(module_id) {
                Self::dfs_cycle_detection(
                    module_id,
                    adjacency,
                    &mut visited,
                    &mut rec_stack,
                    &mut cycles,
                    &mut Vec::new(),
                );
            }
        }

        cycles
    }

    /// DFS helper for cycle detection.
    fn dfs_cycle_detection(
        node: &ModuleId,
        adjacency: &HashMap<ModuleId, Vec<ModuleId>>,
        visited: &mut HashSet<ModuleId>,
        rec_stack: &mut HashSet<ModuleId>,
        cycles: &mut Vec<Vec<ModuleId>>,
        path: &mut Vec<ModuleId>,
    ) {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());

        if let Some(neighbors) = adjacency.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    Self::dfs_cycle_detection(neighbor, adjacency, visited, rec_stack, cycles, path);
                } else if rec_stack.contains(neighbor) {
                    // Found cycle
                    if let Some(cycle_start) = path.iter().position(|n| n == neighbor) {
                        cycles.push(path[cycle_start..].to_vec());
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
    }

    /// Simple topological sort (returns None if cycles detected).
    pub fn simple_topo_sort(
        adjacency: &HashMap<ModuleId, Vec<ModuleId>>,
        modules: &HashMap<ModuleId, super::module_graph::Module>,
    ) -> Option<Vec<ModuleId>> {
        if !Self::detect_cycles(adjacency, modules).is_empty() {
            return None; // Cycles found
        }

        // Simplified topological sort
        let mut result = Vec::new();
        let mut remaining: HashSet<ModuleId> = modules.keys().cloned().collect();

        while !remaining.is_empty() {
            // Find a module with no dependencies in remaining set
            let next = remaining.iter().find(|&&ref module| {
                adjacency
                    .get(module)
                    .map(|deps| deps.iter().all(|dep| !remaining.contains(dep)))
                    .unwrap_or(true)
            }).cloned();

            match next {
                Some(module) => {
                    result.push(module.clone());
                    remaining.remove(&module);
                }
                None => return None, // Circular dependency
            }
        }

        Some(result)
    }
}