use std::path::PathBuf;
use std::collections::HashMap;
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::analyzers::ComplexityResult;
use crate::api::analysis::{CfgResult, BasicBlockResult};
use crate::storage::{SymbolStore, FileRecord, CallGraph, NameResolver};
use crate::api::analysis::AnalysisOptions;
use crate::api::results::workspace_methods::{WorkspaceMethods, WorkspaceStats};
use crate::api::analysis::analysis_runner::AnalysisRunner;
/// Unified analysis results supporting both single file and workspace analysis.
#[derive(Debug)]
pub struct AnalysisResult {
    // Analysis data (always present) - accessible to result methods
    pub(crate) complexity_data: Vec<ComplexityResult>,
    pub(crate) cfg_data: Vec<CfgResult>,
    pub(crate) basic_block_data: Vec<BasicBlockResult>,
    pub(crate) function_data: Vec<FunctionSpan>,
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
        // Extract symbols using language-specific extractors
        use crate::api::extractors::language_extractors::LanguageExtractors;
        LanguageExtractors::extract_symbols(&file_path, &mut result.symbol_store)?;
        Ok(result)
    }
    /// Workspace analysis.
    fn from_workspace(workspace_path: PathBuf, options: AnalysisOptions) -> Result<Self, NTreeError> {
        let mut result = AnalysisResult {
            complexity_data: Vec::new(),
            cfg_data: Vec::new(),
            basic_block_data: Vec::new(),
            function_data: Vec::new(),
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
        Ok(result)
    }
}