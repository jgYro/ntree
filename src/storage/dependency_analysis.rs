use super::module_graph::ModuleId;

/// Analysis results for module dependencies.
#[derive(Debug, Clone)]
pub struct DependencyAnalysis {
    pub total_modules: usize,
    pub total_edges: usize,
    pub cycles: usize,
    pub has_cycles: bool,
    pub topo_order: Option<Vec<ModuleId>>,
    pub cycle_details: Vec<Vec<ModuleId>>,
}

impl DependencyAnalysis {
    /// Create new dependency analysis.
    pub fn new(
        total_modules: usize,
        total_edges: usize,
        cycles: Vec<Vec<ModuleId>>,
        topo_order: Option<Vec<ModuleId>>,
    ) -> Self {
        DependencyAnalysis {
            total_modules,
            total_edges,
            cycles: cycles.len(),
            has_cycles: !cycles.is_empty(),
            topo_order,
            cycle_details: cycles,
        }
    }

    /// Check if dependencies can be processed topologically.
    pub fn can_process_topologically(&self) -> bool {
        !self.has_cycles
    }

    /// Get processing order (None if cycles exist).
    pub fn processing_order(&self) -> Option<&Vec<ModuleId>> {
        self.topo_order.as_ref()
    }

    /// Get strongly connected components (cycles).
    pub fn get_cycles(&self) -> &Vec<Vec<ModuleId>> {
        &self.cycle_details
    }

    /// Summary statistics.
    pub fn summary(&self) -> String {
        if self.has_cycles {
            format!(
                "Module graph: {} modules, {} edges, {} cycles detected",
                self.total_modules, self.total_edges, self.cycles
            )
        } else {
            format!(
                "Module graph: {} modules, {} edges, topologically sortable",
                self.total_modules, self.total_edges
            )
        }
    }
}
