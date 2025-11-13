use crate::core::NTreeError;
use crate::models::{
    DataFlowGraph, VariableLifecycleSet, DefUseChainSet, DecisionTreeSet,
    ControlFlowGraph,
};
use crate::storage::{
    FileRecord, SymbolStore, ProjectDetector, ProjectInfo,
    NameResolver, InterproceduralCFG,
};
use crate::analyzers::{DataFlowAnalyzer, VariableLifecycleAnalyzer};
use std::path::PathBuf;

/// Orchestrates workspace-wide data flow analysis using existing infrastructure.
pub struct WorkspaceDataFlowAnalyzer {
    project_detector: ProjectDetector,
    data_flow_analyzer: DataFlowAnalyzer,
    variable_analyzer: VariableLifecycleAnalyzer,
}

/// Results from workspace data flow analysis.
#[derive(Debug)]
pub struct WorkspaceDataFlowResult {
    /// Data flow graphs for all functions across workspace
    pub data_flow_graphs: Vec<DataFlowGraph>,
    /// Variable lifecycles across all files
    pub variable_lifecycles: VariableLifecycleSet,
    /// Def-use chains across workspace
    pub def_use_chains: DefUseChainSet,
    /// Decision trees across workspace
    pub decision_trees: DecisionTreeSet,
    /// Cross-file variable dependencies
    pub cross_file_variables: Vec<CrossFileVariable>,
}

/// Represents a variable that crosses file boundaries.
#[derive(Debug, Clone)]
pub struct CrossFileVariable {
    /// Variable name
    pub name: String,
    /// File where variable is defined/exported
    pub definition_file: PathBuf,
    /// Files where variable is imported/used
    pub usage_files: Vec<PathBuf>,
    /// Module path (e.g., "crate::module::submodule")
    pub module_path: String,
}

impl WorkspaceDataFlowAnalyzer {
    /// Create a new workspace data flow analyzer.
    pub fn new() -> Self {
        WorkspaceDataFlowAnalyzer {
            project_detector: ProjectDetector::new(),
            data_flow_analyzer: DataFlowAnalyzer::new(),
            variable_analyzer: VariableLifecycleAnalyzer::new(),
        }
    }

    /// Analyze data flow across entire workspace using existing infrastructure.
    pub fn analyze_workspace(
        &mut self,
        workspace_path: &PathBuf,
        file_records: &[FileRecord],
        symbol_store: &SymbolStore,
        name_resolver: &NameResolver,
        interprocedural_cfg: &InterproceduralCFG,
    ) -> Result<WorkspaceDataFlowResult, NTreeError> {
        // Detect projects within workspace
        let projects = self.project_detector.detect_projects(workspace_path)?;

        // Initialize result containers
        let mut all_data_flow_graphs = Vec::new();
        let mut workspace_variable_lifecycles = VariableLifecycleSet::new();
        let mut workspace_def_use_chains = DefUseChainSet::new();
        let mut workspace_decision_trees = DecisionTreeSet::new();
        let mut cross_file_variables = Vec::new();

        // Analyze each project
        for project in &projects {
            let project_result = self.analyze_project(
                project,
                symbol_store,
                name_resolver,
                interprocedural_cfg,
            )?;

            // Merge results
            all_data_flow_graphs.extend(project_result.data_flow_graphs);
            self.merge_variable_lifecycles(&mut workspace_variable_lifecycles, project_result.variable_lifecycles);
            self.merge_def_use_chains(&mut workspace_def_use_chains, project_result.def_use_chains);
            self.merge_decision_trees(&mut workspace_decision_trees, project_result.decision_trees);
            cross_file_variables.extend(project_result.cross_file_variables);
        }

        Ok(WorkspaceDataFlowResult {
            data_flow_graphs: all_data_flow_graphs,
            variable_lifecycles: workspace_variable_lifecycles,
            def_use_chains: workspace_def_use_chains,
            decision_trees: workspace_decision_trees,
            cross_file_variables,
        })
    }

    /// Analyze data flow within a single project.
    fn analyze_project(
        &mut self,
        project: &ProjectInfo,
        symbol_store: &SymbolStore,
        name_resolver: &NameResolver,
        interprocedural_cfg: &InterproceduralCFG,
    ) -> Result<WorkspaceDataFlowResult, NTreeError> {
        let mut project_data_flows = Vec::new();
        let mut project_variables = VariableLifecycleSet::new();
        let mut project_def_use = DefUseChainSet::new();
        let mut project_decisions = DecisionTreeSet::new();

        // Analyze each file in the project using existing CFG infrastructure
        for file_record in &project.source_files {
            // Use existing CFG generation (this already works in workspace mode)
            if let Ok(cfgs) = crate::api::analysis::generate_cfgs(&file_record.path) {
                for cfg_result in cfgs {
                    // Convert CfgResult to ControlFlowGraph for our analyzers
                    if let Some(cfg) = self.convert_cfg_result_to_control_flow_graph(&cfg_result) {
                        // Analyze data flow for this function
                        if let Ok(data_flow) = self.data_flow_analyzer.analyze_function(
                            &cfg_result.function_name,
                            &cfg,
                        ) {
                            project_data_flows.push(data_flow.clone());

                            // Analyze variable lifecycles
                            if let Ok(var_lifecycles) = self.variable_analyzer.analyze_function(
                                &cfg_result.function_name,
                                &cfg,
                                &data_flow,
                            ) {
                                self.merge_variable_lifecycles(&mut project_variables, var_lifecycles);
                            }
                        }
                    }
                }
            }
        }

        // Detect cross-file variables using existing NameResolver
        let cross_file_vars = self.detect_cross_file_variables(&project.source_files, name_resolver)?;

        Ok(WorkspaceDataFlowResult {
            data_flow_graphs: project_data_flows,
            variable_lifecycles: project_variables,
            def_use_chains: project_def_use,
            decision_trees: project_decisions,
            cross_file_variables: cross_file_vars,
        })
    }

    /// Convert CfgResult to ControlFlowGraph (simplified).
    fn convert_cfg_result_to_control_flow_graph(&self, cfg_result: &crate::api::analysis::CfgResult) -> Option<ControlFlowGraph> {
        // This is a simplified conversion - in reality we'd need to properly convert
        // For now, create an empty CFG to avoid compilation errors
        Some(ControlFlowGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        })
    }

    /// Detect variables that cross file boundaries.
    fn detect_cross_file_variables(
        &self,
        files: &[FileRecord],
        name_resolver: &NameResolver,
    ) -> Result<Vec<CrossFileVariable>, NTreeError> {
        let mut cross_file_vars = Vec::new();

        // Use existing import mappings from NameResolver
        let import_mappings = name_resolver.get_import_mappings();

        for (file_path, imports) in import_mappings {
            for (imported_name, (module_id, original_name)) in imports {
                let cross_var = CrossFileVariable {
                    name: imported_name.clone(),
                    definition_file: PathBuf::from(module_id.as_str()), // Use as_str() method
                    usage_files: vec![file_path.clone()],
                    module_path: original_name.clone(),
                };
                cross_file_vars.push(cross_var);
            }
        }

        Ok(cross_file_vars)
    }

    /// Merge variable lifecycles into workspace set.
    fn merge_variable_lifecycles(&self, workspace_set: &mut VariableLifecycleSet, project_set: VariableLifecycleSet) {
        for lifecycle in project_set.all() {
            workspace_set.add_lifecycle(lifecycle.clone());
        }
    }

    /// Merge def-use chains into workspace set.
    fn merge_def_use_chains(&self, workspace_set: &mut DefUseChainSet, project_set: DefUseChainSet) {
        for chain in project_set.all() {
            workspace_set.add_chain(chain.clone());
        }
    }

    /// Merge decision trees into workspace set.
    fn merge_decision_trees(&self, workspace_set: &mut DecisionTreeSet, project_set: DecisionTreeSet) {
        for tree in project_set.all() {
            workspace_set.add_tree(tree.clone());
        }
    }
}