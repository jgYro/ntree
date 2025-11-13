use crate::core::NTreeError;
use crate::storage::SymbolId;
use std::collections::{HashMap, HashSet};

/// Reverse dependency index for efficient invalidation.
#[derive(Debug)]
pub struct ReverseDependencyIndex {
    /// Maps function -> set of functions that call it
    reverse_calls: HashMap<SymbolId, HashSet<SymbolId>>,
    /// Maps function -> set of functions it calls
    forward_calls: HashMap<SymbolId, HashSet<SymbolId>>,
    /// Transitive closure cache
    transitive_cache: HashMap<SymbolId, HashSet<SymbolId>>,
}

impl ReverseDependencyIndex {
    /// Create new reverse dependency index.
    pub fn new() -> Self {
        ReverseDependencyIndex {
            reverse_calls: HashMap::new(),
            forward_calls: HashMap::new(),
            transitive_cache: HashMap::new(),
        }
    }

    /// Add a call relationship.
    pub fn add_call(&mut self, caller: SymbolId, callee: SymbolId) {
        // Forward: caller -> callee
        self.forward_calls
            .entry(caller.clone())
            .or_insert_with(HashSet::new)
            .insert(callee.clone());

        // Reverse: callee -> caller
        self.reverse_calls
            .entry(callee)
            .or_insert_with(HashSet::new)
            .insert(caller);

        // Invalidate transitive cache
        self.transitive_cache.clear();
    }

    /// Remove a call relationship.
    pub fn remove_call(&mut self, caller: &SymbolId, callee: &SymbolId) {
        if let Some(callees) = self.forward_calls.get_mut(caller) {
            callees.remove(callee);
            if callees.is_empty() {
                self.forward_calls.remove(caller);
            }
        }

        if let Some(callers) = self.reverse_calls.get_mut(callee) {
            callers.remove(caller);
            if callers.is_empty() {
                self.reverse_calls.remove(callee);
            }
        }

        self.transitive_cache.clear();
    }

    /// Get direct callers of a function.
    pub fn get_direct_callers(&self, function: &SymbolId) -> HashSet<SymbolId> {
        self.reverse_calls
            .get(function)
            .cloned()
            .unwrap_or_default()
    }

    /// Get direct callees of a function.
    pub fn get_direct_callees(&self, function: &SymbolId) -> HashSet<SymbolId> {
        self.forward_calls
            .get(function)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all transitive callers (functions that transitively depend on this function).
    pub fn get_transitive_callers(
        &mut self,
        function: &SymbolId,
    ) -> Result<HashSet<SymbolId>, NTreeError> {
        if let Some(cached) = self.transitive_cache.get(function) {
            return Ok(cached.clone());
        }

        let mut transitive_callers = HashSet::new();
        let mut visited = HashSet::new();
        let mut stack = vec![function.clone()];

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            let direct_callers = self.get_direct_callers(&current);
            for caller in direct_callers {
                if transitive_callers.insert(caller.clone()) {
                    stack.push(caller);
                }
            }
        }

        self.transitive_cache
            .insert(function.clone(), transitive_callers.clone());
        Ok(transitive_callers)
    }

    /// Get functions that need recomputation when target function changes.
    pub fn get_invalidation_set(
        &mut self,
        changed_function: &SymbolId,
    ) -> Result<HashSet<SymbolId>, NTreeError> {
        let mut invalidation_set = HashSet::new();

        // Include the changed function itself
        invalidation_set.insert(changed_function.clone());

        // Include all transitive callers
        let transitive_callers = self.get_transitive_callers(changed_function)?;
        invalidation_set.extend(transitive_callers);

        Ok(invalidation_set)
    }

    /// Update index from function summaries.
    pub fn rebuild_from_summaries<'a>(
        &mut self,
        summaries: impl Iterator<Item = &'a super::func_summary::FuncSummary>,
    ) {
        self.clear();

        for summary in summaries {
            for callee in summary.get_callees() {
                self.add_call(summary.sym_id.clone(), callee.clone());
            }
        }
    }

    /// Clear all dependency data.
    pub fn clear(&mut self) {
        self.reverse_calls.clear();
        self.forward_calls.clear();
        self.transitive_cache.clear();
    }

    /// Get dependency statistics.
    pub fn get_stats(&self) -> DependencyStats {
        let total_functions = self.forward_calls.len();
        let total_calls: usize = self.forward_calls.values().map(|s| s.len()).sum();
        let max_fanin = self
            .reverse_calls
            .values()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);
        let max_fanout = self
            .forward_calls
            .values()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        DependencyStats {
            total_functions,
            total_calls,
            max_fanin,
            max_fanout,
        }
    }
}

/// Dependency analysis statistics.
#[derive(Debug, Clone)]
pub struct DependencyStats {
    pub total_functions: usize,
    pub total_calls: usize,
    pub max_fanin: usize,
    pub max_fanout: usize,
}
