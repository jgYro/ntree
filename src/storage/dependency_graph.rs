use std::collections::HashMap;
use crate::core::NTreeError;
use super::module_graph::{Module, ModuleId, ModuleEdge};
use super::dependency_edges::ImportEdge;
use super::cycle_detector::CycleDetector;
use super::dependency_analysis::DependencyAnalysis;

/// Directed module dependency graph with cycle detection.
#[derive(Debug)]
pub struct DependencyGraph {
    modules: HashMap<ModuleId, Module>,
    edges: Vec<ModuleEdge>,
    adjacency: HashMap<ModuleId, Vec<ModuleId>>,
}

impl DependencyGraph {
    /// Create new empty dependency graph.
    pub fn new() -> Self {
        DependencyGraph {
            modules: HashMap::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Build graph from import edges.
    pub fn from_import_edges(imports: &[ImportEdge]) -> Result<Self, NTreeError> {
        let mut graph = Self::new();

        for import in imports {
            let from_id = ModuleId::from_language_path(
                &import.source_file.to_string_lossy(),
                "unknown", // TODO: Get language from file extension
            );
            let to_id = ModuleId::from_language_path(&import.target_module, "unknown");

            graph.add_edge(ModuleEdge::new(
                from_id.clone(),
                to_id.clone(),
                super::module_graph::EdgeKind::Import,
                import.span.clone(),
            ));
        }

        Ok(graph)
    }

    /// Add module to graph.
    pub fn add_module(&mut self, module: Module) {
        let id = module.id.clone();
        self.modules.insert(id.clone(), module);
        self.adjacency.entry(id).or_insert_with(Vec::new);
    }

    /// Add edge to graph.
    pub fn add_edge(&mut self, edge: ModuleEdge) {
        let from = edge.from.clone();
        let to = edge.to.clone();

        self.adjacency
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());

        // Ensure target module exists in adjacency
        self.adjacency.entry(to).or_insert_with(Vec::new);

        self.edges.push(edge);
    }

    /// Detect cycles in the module dependency graph.
    pub fn detect_cycles(&self) -> Vec<Vec<ModuleId>> {
        CycleDetector::detect_cycles(&self.adjacency, &self.modules)
    }

    /// Get topological ordering of modules.
    pub fn topological_sort(&self) -> Option<Vec<ModuleId>> {
        CycleDetector::simple_topo_sort(&self.adjacency, &self.modules)
    }

    /// Get dependency analysis results.
    pub fn analyze_dependencies(&self) -> DependencyAnalysis {
        let cycles = self.detect_cycles();
        let topo_order = self.topological_sort();
        DependencyAnalysis::new(self.modules.len(), self.edges.len(), cycles, topo_order)
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}