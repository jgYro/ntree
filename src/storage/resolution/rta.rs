use super::cha::ClassHierarchyAnalyzer;
use super::types::{CallSiteId, Resolution, TypeInstantiated};
use crate::core::NTreeError;
use crate::storage::SymbolId;
use std::collections::{HashMap, HashSet};

/// Rapid Type Analysis for precise OO call resolution.
#[derive(Debug)]
pub struct RapidTypeAnalyzer {
    /// CHA analyzer for hierarchy information
    cha_analyzer: ClassHierarchyAnalyzer,
    /// Type instantiation sites
    instantiated_types: HashMap<SymbolId, Vec<TypeInstantiated>>,
    /// Reachable instantiations
    reachable_instantiations: HashSet<SymbolId>,
}

impl RapidTypeAnalyzer {
    /// Create new RTA analyzer.
    pub fn new(cha_analyzer: ClassHierarchyAnalyzer) -> Self {
        RapidTypeAnalyzer {
            cha_analyzer,
            instantiated_types: HashMap::new(),
            reachable_instantiations: HashSet::new(),
        }
    }

    /// Add type instantiation.
    pub fn add_instantiation(&mut self, instantiation: TypeInstantiated) {
        self.instantiated_types
            .entry(instantiation.type_id.clone())
            .or_insert_with(Vec::new)
            .push(instantiation);
    }

    /// Mark type instantiation as reachable.
    pub fn mark_reachable(&mut self, type_id: SymbolId) {
        self.reachable_instantiations.insert(type_id.clone());

        if let Some(instantiations) = self.instantiated_types.get_mut(&type_id) {
            for instantiation in instantiations {
                instantiation.mark_reachable();
            }
        }
    }

    /// Resolve virtual call using RTA (refined from CHA).
    pub fn resolve_virtual_call(
        &self,
        callsite_id: CallSiteId,
        receiver_type: &SymbolId,
        method_name: &str,
        caller: SymbolId,
        call_expr: String,
    ) -> Result<Resolution, NTreeError> {
        // Start with CHA resolution
        let cha_resolution = self.cha_analyzer.resolve_virtual_call(
            callsite_id,
            receiver_type,
            method_name,
            caller.clone(),
            call_expr.clone(),
        )?;

        // Filter candidates by instantiated types
        let mut rta_targets = Vec::new();

        for target in &cha_resolution.targets {
            // Check if the type owning this method is instantiated and reachable
            if let Some(target_type) = self.get_method_owner_type(target) {
                if self.is_type_reachable(&target_type) {
                    rta_targets.push(target.clone());
                }
            } else {
                // If we can't determine ownership, include conservatively
                rta_targets.push(target.clone());
            }
        }

        // If no targets found, fall back to CHA
        if rta_targets.is_empty() {
            rta_targets = cha_resolution.targets;
        }

        Ok(Resolution::rta(callsite_id, rta_targets, caller, call_expr))
    }

    /// Check if type is instantiated and reachable.
    fn is_type_reachable(&self, type_id: &SymbolId) -> bool {
        self.reachable_instantiations.contains(type_id)
    }

    /// Get the type that owns a method (simplified implementation).
    fn get_method_owner_type(&self, _method_symbol: &SymbolId) -> Option<SymbolId> {
        // In a real implementation, this would parse the symbol ID
        // to extract the owning type
        None
    }

    /// Update reachability based on program analysis.
    pub fn update_reachability(&mut self, reachable_functions: &HashSet<SymbolId>) {
        // Mark types as reachable if any of their methods are reachable
        for function in reachable_functions {
            if let Some(owner_type) = self.get_method_owner_type(function) {
                self.mark_reachable(owner_type);
            }
        }
    }

    /// Get RTA statistics.
    pub fn get_stats(&self) -> RTAStats {
        let total_instantiations: usize = self.instantiated_types.values().map(|v| v.len()).sum();
        let reachable_instantiations = self.reachable_instantiations.len();

        RTAStats {
            total_types: self.instantiated_types.len(),
            total_instantiations,
            reachable_types: reachable_instantiations,
            cha_stats: self.cha_analyzer.get_stats(),
        }
    }

    /// Get all instantiated types.
    pub fn get_instantiated_types(&self) -> Vec<&SymbolId> {
        self.instantiated_types.keys().collect()
    }

    /// Get reachable instantiated types.
    pub fn get_reachable_types(&self) -> &HashSet<SymbolId> {
        &self.reachable_instantiations
    }
}

/// RTA analysis statistics.
#[derive(Debug, Clone)]
pub struct RTAStats {
    pub total_types: usize,
    pub total_instantiations: usize,
    pub reachable_types: usize,
    pub cha_stats: super::cha::CHAStats,
}
