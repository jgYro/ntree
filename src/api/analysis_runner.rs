use std::path::PathBuf;
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::analyzers::{ComplexityResult, ComplexityAnalyzer};
use crate::api::{CfgResult, BasicBlockResult, generate_cfg_ir};

/// Internal module for running individual analyses.
pub struct AnalysisRunner;

impl AnalysisRunner {
    /// Run complexity analysis on CFG IR data.
    pub fn run_complexity_analysis(file_path: &PathBuf) -> Result<Vec<ComplexityResult>, NTreeError> {
        // Generate CFG IR data if needed for complexity analysis
        let cfg_ir_results = match generate_cfg_ir(file_path) {
            Ok(results) => results,
            Err(e) => return Err(e),
        };

        let mut complexity_data = Vec::new();
        let analyzer = ComplexityAnalyzer::new();
        for cfg_ir in cfg_ir_results {
            match analyzer.analyze(&cfg_ir) {
                Ok(result) => complexity_data.push(result),
                Err(e) => return Err(NTreeError::ParseError(format!("Complexity analysis failed: {}", e))),
            }
        }

        Ok(complexity_data)
    }

    /// Run CFG generation if enabled.
    pub fn run_cfg_generation(file_path: &PathBuf) -> Result<Vec<CfgResult>, NTreeError> {
        match crate::api::generate_cfgs(file_path) {
            Ok(cfgs) => Ok(cfgs),
            Err(e) => Err(e),
        }
    }

    /// Run basic block generation if enabled.
    pub fn run_basic_block_generation(file_path: &PathBuf) -> Result<Vec<BasicBlockResult>, NTreeError> {
        match crate::api::generate_basic_blocks(file_path) {
            Ok(blocks) => Ok(blocks),
            Err(e) => Err(e),
        }
    }

    /// Run function extraction.
    pub fn run_function_extraction(file_path: &PathBuf) -> Result<Vec<FunctionSpan>, NTreeError> {
        match crate::api::list_functions(file_path) {
            Ok(functions) => Ok(functions),
            Err(e) => Err(e),
        }
    }
}