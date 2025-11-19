use super::advanced_result_sets::{
    ExternalLibraryResultSet, IncrementalResultSet, InterproceduralResultSet,
};
use super::data_flow_result_sets::{
    CrossFileVariableResultSet, DataFlowResultSet, DecisionTreeResultSet, DefUseChainResultSet,
    VariableLifecycleResultSet,
};
use super::function_results::{BasicBlockResultSet, FunctionResultSet};
use super::result_sets::{CfgResultSet, ComplexityResultSet};
use super::symbol_methods::SymbolResultSet;
use super::workspace_methods::WorkspaceStats;
use crate::api::core::unified_analysis::AnalysisResult;
use crate::api::export::export_utils::ExportUtils;
use crate::core::NTreeError;
use crate::storage::{CallGraph, DependencyGraph, FileRecord, NameResolver};
use std::collections::HashMap;

/// Implementation of result access methods for unified AnalysisResult.
impl AnalysisResult {
    /// Get complexity analysis results.
    pub fn complexity(&self) -> ComplexityResultSet<'_> {
        ComplexityResultSet {
            data: &self.complexity_data,
        }
    }

    /// Get CFG analysis results.
    pub fn cfgs(&self) -> CfgResultSet<'_> {
        CfgResultSet {
            data: &self.cfg_data,
        }
    }

    /// Get function information.
    pub fn functions(&self) -> FunctionResultSet<'_> {
        FunctionResultSet {
            data: &self.function_data,
        }
    }

    /// Get basic block information.
    pub fn basic_blocks(&self) -> BasicBlockResultSet<'_> {
        BasicBlockResultSet {
            data: &self.basic_block_data,
        }
    }

    /// Get symbol search interface.
    pub fn symbols(&self) -> SymbolResultSet<'_> {
        SymbolResultSet::new(&self.symbol_store)
    }

    /// Get files grouped by language (workspace mode only).
    pub fn files_by_language(&self) -> Option<&HashMap<String, Vec<FileRecord>>> {
        if self.is_workspace_mode {
            Some(&self.files_by_language)
        } else {
            None
        }
    }

    /// Get workspace statistics (workspace mode only).
    pub fn workspace_stats(&self) -> Option<&WorkspaceStats> {
        self.workspace_stats.as_ref()
    }

    /// Export all results to JSONL format.
    pub fn to_jsonl(&self) -> Result<String, NTreeError> {
        ExportUtils::to_jsonl(
            &self.cfg_data,
            &self.basic_block_data,
            &self.complexity_data,
        )
    }

    /// Check if this is workspace analysis mode.
    pub fn is_workspace_mode(&self) -> bool {
        self.is_workspace_mode
    }

    /// Get total symbol count.
    pub fn symbol_count(&self) -> usize {
        self.symbol_store.stats().total_symbols
    }

    /// Get call graph for function call relationships.
    pub fn call_graph(&self) -> &CallGraph {
        &self.call_graph
    }

    /// Get dependency graph for module relationships.
    pub fn dependencies(&self) -> Result<DependencyGraph, NTreeError> {
        let dataset = self.export_dataset()?;
        DependencyGraph::from_import_edges(&dataset.import_edges)
    }

    /// Get name resolver for cross-file symbol resolution.
    pub fn name_bindings(&self) -> Option<&NameResolver> {
        self.name_resolver.as_ref()
    }

    /// Get symbol store for direct symbol access.
    pub fn symbol_store(&self) -> &crate::storage::SymbolStore {
        &self.symbol_store
    }

    /// Get CFG results.
    pub fn cfg_results(&self) -> &[crate::api::analysis::CfgResult] {
        &self.cfg_data
    }

    /// Get interprocedural analysis results (summary edges, reachability).
    pub fn interprocedural(&self) -> InterproceduralResultSet<'_> {
        InterproceduralResultSet::new(self)
    }

    /// Get incremental analysis interface for cache and performance.
    pub fn incremental(&self) -> IncrementalResultSet<'_> {
        IncrementalResultSet::new(self)
    }

    /// Get external library analysis results.
    pub fn external_libraries(&self) -> ExternalLibraryResultSet<'_> {
        ExternalLibraryResultSet::new(self)
    }

    /// Get data flow analysis results.
    pub fn data_flow(&self) -> DataFlowResultSet<'_> {
        DataFlowResultSet::new(&self.data_flow_graphs)
    }

    /// Get variable lifecycle tracking results.
    pub fn variables(&self) -> VariableLifecycleResultSet<'_> {
        VariableLifecycleResultSet::new(&self.variable_lifecycles)
    }

    /// Get def-use chain analysis results.
    pub fn def_use_chains(&self) -> DefUseChainResultSet<'_> {
        DefUseChainResultSet::new(&self.def_use_chains)
    }

    /// Get decision tree mapping results.
    pub fn decision_trees(&self) -> DecisionTreeResultSet<'_> {
        DecisionTreeResultSet::new(&self.decision_trees)
    }

    /// Get cross-file variable analysis results (workspace mode only).
    pub fn cross_file_variables(&self) -> CrossFileVariableResultSet<'_> {
        CrossFileVariableResultSet::new(&self.cross_file_variables)
    }
}
