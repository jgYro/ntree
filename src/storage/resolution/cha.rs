use super::types::{CallSiteId, Resolution};
use crate::core::NTreeError;
use crate::storage::SymbolId;
use std::collections::{HashMap, HashSet};

/// Class Hierarchy Analysis for OO/trait call resolution.
#[derive(Debug)]
pub struct ClassHierarchyAnalyzer {
    /// Type hierarchy: child -> parent mappings
    type_hierarchy: HashMap<SymbolId, SymbolId>,
    /// Method implementations: (type, method_name) -> implementation
    method_implementations: HashMap<(SymbolId, String), SymbolId>,
    /// Virtual method overrides: base_method -> set of overrides
    method_overrides: HashMap<SymbolId, HashSet<SymbolId>>,
    /// Interface/trait implementations: trait -> implementing types
    trait_implementations: HashMap<SymbolId, HashSet<SymbolId>>,
}

impl ClassHierarchyAnalyzer {
    /// Create new CHA analyzer.
    pub fn new() -> Self {
        ClassHierarchyAnalyzer {
            type_hierarchy: HashMap::new(),
            method_implementations: HashMap::new(),
            method_overrides: HashMap::new(),
            trait_implementations: HashMap::new(),
        }
    }

    /// Add inheritance relationship.
    pub fn add_inheritance(&mut self, child_type: SymbolId, parent_type: SymbolId) {
        self.type_hierarchy.insert(child_type, parent_type);
    }

    /// Add method implementation.
    pub fn add_method(&mut self, type_id: SymbolId, method_name: String, implementation: SymbolId) {
        self.method_implementations
            .insert((type_id, method_name), implementation);
    }

    /// Add method override relationship.
    pub fn add_override(&mut self, base_method: SymbolId, override_method: SymbolId) {
        self.method_overrides
            .entry(base_method)
            .or_insert_with(HashSet::new)
            .insert(override_method);
    }

    /// Add trait implementation.
    pub fn add_trait_impl(&mut self, trait_id: SymbolId, implementing_type: SymbolId) {
        self.trait_implementations
            .entry(trait_id)
            .or_insert_with(HashSet::new)
            .insert(implementing_type);
    }

    /// Resolve virtual call using CHA.
    pub fn resolve_virtual_call(
        &self,
        callsite_id: CallSiteId,
        receiver_type: &SymbolId,
        method_name: &str,
        caller: SymbolId,
        call_expr: String,
    ) -> Result<Resolution, NTreeError> {
        let mut candidates = HashSet::new();

        // Find base implementation
        if let Some(base_impl) = self
            .method_implementations
            .get(&(receiver_type.clone(), method_name.to_string()))
        {
            candidates.insert(base_impl.clone());

            // Add all overrides
            if let Some(overrides) = self.method_overrides.get(base_impl) {
                candidates.extend(overrides.clone());
            }
        }

        // Check trait implementations
        for (trait_id, implementors) in &self.trait_implementations {
            if let Some(_trait_method) = self
                .method_implementations
                .get(&(trait_id.clone(), method_name.to_string()))
            {
                // Check if receiver type implements this trait
                if self.implements_trait(receiver_type, trait_id) {
                    for implementor in implementors {
                        if let Some(impl_method) = self
                            .method_implementations
                            .get(&(implementor.clone(), method_name.to_string()))
                        {
                            candidates.insert(impl_method.clone());
                        }
                    }
                }
            }
        }

        // Get all subtypes of receiver that might override the method
        let subtypes = self.get_all_subtypes(receiver_type);
        for subtype in subtypes {
            if let Some(override_impl) = self
                .method_implementations
                .get(&(subtype, method_name.to_string()))
            {
                candidates.insert(override_impl.clone());
            }
        }

        let targets: Vec<SymbolId> = candidates.into_iter().collect();
        Ok(Resolution::cha(callsite_id, targets, caller, call_expr))
    }

    /// Check if type implements trait.
    fn implements_trait(&self, type_id: &SymbolId, trait_id: &SymbolId) -> bool {
        if let Some(implementors) = self.trait_implementations.get(trait_id) {
            return implementors.contains(type_id);
        }

        // Check inheritance chain
        let mut current = type_id;
        while let Some(parent) = self.type_hierarchy.get(current) {
            if let Some(implementors) = self.trait_implementations.get(trait_id) {
                if implementors.contains(parent) {
                    return true;
                }
            }
            current = parent;
        }

        false
    }

    /// Get all subtypes of a type.
    fn get_all_subtypes(&self, type_id: &SymbolId) -> HashSet<SymbolId> {
        let mut subtypes = HashSet::new();

        for (child, parent) in &self.type_hierarchy {
            if parent == type_id {
                subtypes.insert(child.clone());
                // Recursively get subtypes of child
                let child_subtypes = self.get_all_subtypes(child);
                subtypes.extend(child_subtypes);
            }
        }

        subtypes
    }

    /// Get hierarchy statistics.
    pub fn get_stats(&self) -> CHAStats {
        CHAStats {
            total_types: self.type_hierarchy.len(),
            total_methods: self.method_implementations.len(),
            total_overrides: self.method_overrides.values().map(|s| s.len()).sum(),
            total_traits: self.trait_implementations.len(),
        }
    }
}

/// CHA analysis statistics.
#[derive(Debug, Clone)]
pub struct CHAStats {
    pub total_types: usize,
    pub total_methods: usize,
    pub total_overrides: usize,
    pub total_traits: usize,
}
