use crate::analyzers::ComplexityResult;
use crate::api::analysis::analysis_runner::AnalysisRunner;
use crate::api::analysis::AnalysisOptions;
use crate::api::analysis::{BasicBlockResult, CfgResult};
use crate::api::results::workspace_methods::{WorkspaceMethods, WorkspaceStats};
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::storage::{CallGraph, FileRecord, NameResolver, SymbolStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
/// Unified analysis results supporting both single file and workspace analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    // Analysis data (always present) - accessible to result methods
    pub(crate) complexity_data: Vec<ComplexityResult>,
    pub(crate) cfg_data: Vec<CfgResult>,
    pub(crate) basic_block_data: Vec<BasicBlockResult>,
    pub(crate) function_data: Vec<FunctionSpan>,
    // Data flow analysis results
    pub(crate) data_flow_graphs: Vec<crate::models::DataFlowGraph>,
    pub(crate) variable_lifecycles: crate::models::VariableLifecycleSet,
    pub(crate) def_use_chains: crate::models::DefUseChainSet,
    pub(crate) decision_trees: crate::models::DecisionTreeSet,
    pub(crate) cross_file_variables: Vec<crate::analyzers::CrossFileVariable>,
    // Symbol and workspace data
    pub(crate) symbol_store: SymbolStore,
    pub(crate) file_records: Vec<FileRecord>,
    pub(crate) files_by_language: HashMap<String, Vec<FileRecord>>,
    pub(crate) workspace_stats: Option<WorkspaceStats>,
    pub(crate) is_workspace_mode: bool,
    // Call graph and resolution data
    pub(crate) call_graph: CallGraph,
    pub(crate) name_resolver: Option<NameResolver>,
}

impl AnalysisResult {
    /// Create analysis result by running configured analyses.
    pub fn from_source_code(
        path: PathBuf,
        options: AnalysisOptions,
        is_workspace: bool,
    ) -> Result<Self, NTreeError> {
        if is_workspace || options.workspace_search {
            Self::from_workspace(path, options)
        } else {
            Self::from_single_file(path, options)
        }
    }

    /// Create result from single file analysis.
    fn from_single_file(file_path: PathBuf, options: AnalysisOptions) -> Result<Self, NTreeError> {
        let mut result = AnalysisResult {
            complexity_data: Vec::new(),
            cfg_data: Vec::new(),
            basic_block_data: Vec::new(),
            function_data: Vec::new(),
            data_flow_graphs: Vec::new(),
            variable_lifecycles: crate::models::VariableLifecycleSet::new(),
            def_use_chains: crate::models::DefUseChainSet::new(),
            decision_trees: crate::models::DecisionTreeSet::new(),
            cross_file_variables: Vec::new(),
            symbol_store: SymbolStore::new(),
            file_records: Vec::new(),
            files_by_language: HashMap::new(),
            workspace_stats: None,
            is_workspace_mode: false,
            call_graph: CallGraph::new(),
            name_resolver: None,
        };

        // Run single file analyses
        result.function_data = AnalysisRunner::run_function_extraction(&file_path)?;

        if options.cfg_generation {
            result.cfg_data = AnalysisRunner::run_cfg_generation(&file_path)?;
        }

        if options.complexity_analysis {
            result.complexity_data = AnalysisRunner::run_complexity_analysis(&file_path)?;
        }

        if options.basic_blocks {
            result.basic_block_data = AnalysisRunner::run_basic_block_generation(&file_path)?;
        }

        // Data flow analyses for single file
        if options.data_flow_analysis {
            result.data_flow_graphs = AnalysisRunner::run_data_flow_analysis(&file_path)?;
        }

        if options.variable_lifecycle_tracking {
            result.variable_lifecycles = AnalysisRunner::run_variable_lifecycle_analysis(
                &file_path,
                &result.data_flow_graphs,
            )?;
        }

        // Extract symbols using language-specific extractors
        use crate::api::extractors::language_extractors::LanguageExtractors;
        LanguageExtractors::extract_symbols(&file_path, &mut result.symbol_store)?;
        Ok(result)
    }
    /// Workspace analysis.
    fn from_workspace(
        workspace_path: PathBuf,
        options: AnalysisOptions,
    ) -> Result<Self, NTreeError> {
        let mut result = AnalysisResult {
            complexity_data: Vec::new(),
            cfg_data: Vec::new(),
            basic_block_data: Vec::new(),
            function_data: Vec::new(),
            data_flow_graphs: Vec::new(),
            variable_lifecycles: crate::models::VariableLifecycleSet::new(),
            def_use_chains: crate::models::DefUseChainSet::new(),
            decision_trees: crate::models::DecisionTreeSet::new(),
            cross_file_variables: Vec::new(),
            symbol_store: SymbolStore::new(),
            file_records: Vec::new(),
            files_by_language: HashMap::new(),
            workspace_stats: None,
            is_workspace_mode: true,
            call_graph: CallGraph::new(),
            name_resolver: None,
        };

        // Workspace analysis
        let (files, by_lang) = WorkspaceMethods::populate_workspace_data(
            &workspace_path,
            &options,
            &mut result.symbol_store,
        )?;
        result.file_records = files;
        result.files_by_language = by_lang;
        result.workspace_stats = Some(WorkspaceMethods::get_workspace_stats(&result.file_records));

        // Run CFG generation for workspace (if enabled)
        if options.cfg_generation {
            for file_record in &result.file_records {
                if let Ok(mut file_cfgs) = AnalysisRunner::run_cfg_generation(&file_record.path) {
                    result.cfg_data.append(&mut file_cfgs);
                }
            }
        }

        // Run complexity analysis for workspace (if enabled)
        if options.complexity_analysis {
            for file_record in &result.file_records {
                if let Ok(mut file_complexity) =
                    AnalysisRunner::run_complexity_analysis(&file_record.path)
                {
                    result.complexity_data.append(&mut file_complexity);
                }
            }
        }

        // Run basic block generation for workspace (if enabled)
        if options.basic_blocks {
            for file_record in &result.file_records {
                if let Ok(mut file_blocks) =
                    AnalysisRunner::run_basic_block_generation(&file_record.path)
                {
                    result.basic_block_data.append(&mut file_blocks);
                }
            }
        }

        // Workspace data flow analysis (if enabled)
        if let Some(workspace_data_flow) = WorkspaceMethods::analyze_workspace_data_flow(
            &workspace_path,
            &options,
            &result.file_records,
            &result.symbol_store,
        )? {
            result.data_flow_graphs = workspace_data_flow.data_flow_graphs;
            result.variable_lifecycles = workspace_data_flow.variable_lifecycles;
            result.def_use_chains = workspace_data_flow.def_use_chains;
            result.decision_trees = workspace_data_flow.decision_trees;
            result.cross_file_variables = workspace_data_flow.cross_file_variables;
        }

        Ok(result)
    }
}

impl AnalysisResult {
    /// Export the entire analysis result to JSON.
    pub fn to_json(&self) -> Result<String, NTreeError> {
        serde_json::to_string_pretty(self).map_err(|e| {
            NTreeError::ParseError(format!("Failed to serialize AnalysisResult to JSON: {}", e))
        })
    }

    /// Import an analysis result from JSON.
    pub fn from_json(json: &str) -> Result<Self, NTreeError> {
        serde_json::from_str(json).map_err(|e| {
            NTreeError::ParseError(format!("Failed to deserialize AnalysisResult from JSON: {}", e))
        })
    }

    /// Save analysis result to a JSON file.
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), NTreeError> {
        let json = self.to_json()?;
        std::fs::write(path, json).map_err(|e| {
            NTreeError::ParseError(format!("Failed to write analysis result to file: {}", e))
        })
    }

    /// Load analysis result from a JSON file.
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, NTreeError> {
        let json = std::fs::read_to_string(path).map_err(|e| {
            NTreeError::ParseError(format!("Failed to read analysis result file: {}", e))
        })?;
        Self::from_json(&json)
    }
}
