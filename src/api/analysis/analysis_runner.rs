use std::path::PathBuf;
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::analyzers::{ComplexityResult, ComplexityAnalyzer, DataFlowAnalyzer, VariableLifecycleAnalyzer};
use crate::api::analysis::{CfgResult, BasicBlockResult, generate_cfg_ir};
use crate::models::{DataFlowGraph, VariableLifecycleSet, DefUseChainSet, DecisionTreeSet, ControlFlowGraph};

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
        match crate::api::analysis::generate_cfgs(file_path) {
            Ok(cfgs) => Ok(cfgs),
            Err(e) => Err(e),
        }
    }

    /// Run basic block generation if enabled.
    pub fn run_basic_block_generation(file_path: &PathBuf) -> Result<Vec<BasicBlockResult>, NTreeError> {
        match crate::api::analysis::generate_basic_blocks(file_path) {
            Ok(blocks) => Ok(blocks),
            Err(e) => Err(e),
        }
    }

    /// Run function extraction.
    pub fn run_function_extraction(file_path: &PathBuf) -> Result<Vec<FunctionSpan>, NTreeError> {
        match crate::api::results::list_functions(file_path) {
            Ok(functions) => Ok(functions),
            Err(e) => Err(e),
        }
    }

    /// Run data flow analysis on a single file.
    pub fn run_data_flow_analysis(file_path: &PathBuf) -> Result<Vec<DataFlowGraph>, NTreeError> {
        let mut data_flow_graphs = Vec::new();
        let mut analyzer = DataFlowAnalyzer::new();

        // Get CFG data for the file
        let cfg_results = Self::run_cfg_generation(file_path)?;

        for cfg_result in cfg_results {
            // Create empty CFG for now (would need proper conversion)
            let cfg = ControlFlowGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            };

            match analyzer.analyze_function(&cfg_result.function_name, &cfg) {
                Ok(data_flow) => data_flow_graphs.push(data_flow),
                Err(e) => return Err(e),
            }
        }

        Ok(data_flow_graphs)
    }

    /// Run variable lifecycle analysis on a single file.
    pub fn run_variable_lifecycle_analysis(
        file_path: &PathBuf,
        data_flow_graphs: &[DataFlowGraph],
    ) -> Result<VariableLifecycleSet, NTreeError> {
        let mut analyzer = VariableLifecycleAnalyzer::new();
        let mut combined_lifecycles = VariableLifecycleSet::new();

        let cfg_results = Self::run_cfg_generation(file_path)?;

        for (cfg_result, data_flow) in cfg_results.iter().zip(data_flow_graphs.iter()) {
            // Create empty CFG for now
            let cfg = ControlFlowGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            };

            match analyzer.analyze_function(&cfg_result.function_name, &cfg, data_flow) {
                Ok(lifecycles) => {
                    for lifecycle in lifecycles.all() {
                        combined_lifecycles.add_lifecycle(lifecycle.clone());
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(combined_lifecycles)
    }
}