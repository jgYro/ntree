use std::path::PathBuf;
use std::collections::HashMap;
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::analyzers::ComplexityResult;
use crate::api::{CfgResult, BasicBlockResult};
use crate::storage::{SymbolStore, FileRecord};
use super::options::AnalysisOptions;
use super::workspace_methods::{WorkspaceMethods, WorkspaceStats};
use super::analysis_runner::AnalysisRunner;
/// Unified analysis results supporting both single file and workspace analysis.
#[derive(Debug)]
pub struct AnalysisResult {
    // Analysis data (always present) - accessible to result methods
    pub(super) complexity_data: Vec<ComplexityResult>,
    pub(super) cfg_data: Vec<CfgResult>,
    pub(super) basic_block_data: Vec<BasicBlockResult>,
    pub(super) function_data: Vec<FunctionSpan>,
    // Symbol and workspace data
    pub(super) symbol_store: SymbolStore,
    pub(super) file_records: Vec<FileRecord>,
    pub(super) files_by_language: HashMap<String, Vec<FileRecord>>,
    pub(super) workspace_stats: Option<WorkspaceStats>,
    pub(super) is_workspace_mode: bool,
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
        use super::language_extractors::LanguageExtractors;
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