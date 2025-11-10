use std::path::Path;
use std::collections::HashMap;
use crate::core::NTreeError;
use crate::storage::{
    InterproceduralCFG, SymbolStore, SymbolId,
    InterproceduralEdge, CallSiteSummary, EntryPoint, ReachabilityInfo,
    ExceptionalEdge, FunctionExit
};
use crate::models::ControlFlowGraph;
use crate::api::analysis::AnalysisOptions;
use crate::api::core::AnalysisResult;

/// Results from interprocedural control flow analysis.
#[derive(Debug)]
pub struct InterproceduralResult {
    /// The interprocedural CFG
    pub interprocedural_cfg: InterproceduralCFG,
    /// Individual function CFGs included
    pub function_cfgs: HashMap<SymbolId, ControlFlowGraph>,
    /// Program entry points
    pub entry_points: Vec<EntryPoint>,
    /// Reachability analysis results
    pub reachability: HashMap<SymbolId, ReachabilityInfo>,
    /// Call site summaries
    pub call_sites: HashMap<usize, CallSiteSummary>,
    /// Exceptional control flow edges
    pub exceptional_edges: Vec<ExceptionalEdge>,
    /// Function exit information
    pub function_exits: HashMap<SymbolId, FunctionExit>,
}

impl InterproceduralResult {
    /// Get summary edges (call and return edges).
    pub fn get_summary_edges(&self) -> Vec<&InterproceduralEdge> {
        self.interprocedural_cfg.get_interprocedural_edges()
            .iter()
            .collect()
    }

    /// Get unreachable functions.
    pub fn get_unreachable_functions(&self) -> Vec<SymbolId> {
        self.reachability
            .iter()
            .filter_map(|(sym_id, info)| {
                if !info.reachable {
                    Some(sym_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get reachable functions from a specific entry point.
    pub fn get_functions_reachable_from(&self, entry_sym: SymbolId) -> Vec<SymbolId> {
        self.reachability
            .iter()
            .filter_map(|(sym_id, info)| {
                if info.reached_from.contains(&entry_sym) {
                    Some(sym_id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get call graph statistics.
    pub fn get_call_graph_stats(&self) -> InterproceduralStats {
        let total_functions = self.function_cfgs.len();
        let total_call_sites = self.call_sites.len();
        let total_edges = self.interprocedural_cfg.get_interprocedural_edges().len();
        let unreachable_count = self.get_unreachable_functions().len();
        let exceptional_edges_count = self.exceptional_edges.len();

        InterproceduralStats {
            total_functions,
            total_call_sites,
            total_interprocedural_edges: total_edges,
            unreachable_functions: unreachable_count,
            exceptional_edges: exceptional_edges_count,
            entry_points: self.entry_points.len(),
        }
    }
}

/// Statistics for interprocedural analysis.
#[derive(Debug, Clone)]
pub struct InterproceduralStats {
    /// Total number of functions analyzed
    pub total_functions: usize,
    /// Total number of call sites
    pub total_call_sites: usize,
    /// Total number of interprocedural edges
    pub total_interprocedural_edges: usize,
    /// Number of unreachable functions
    pub unreachable_functions: usize,
    /// Number of exceptional control flow edges
    pub exceptional_edges: usize,
    /// Number of entry points
    pub entry_points: usize,
}

/// Interprocedural analysis configuration.
#[derive(Debug, Clone)]
pub struct InterproceduralOptions {
    /// Enable summary edge generation
    pub summary_edges: bool,
    /// Enable reachability analysis
    pub reachability_analysis: bool,
    /// Enable exceptional control flow analysis
    pub exceptional_control_flow: bool,
    /// Auto-detect entry points (main, test functions, etc.)
    pub auto_detect_entries: bool,
    /// Manual entry point symbols
    pub manual_entry_points: Vec<String>,
}

impl Default for InterproceduralOptions {
    fn default() -> Self {
        InterproceduralOptions {
            summary_edges: true,
            reachability_analysis: true,
            exceptional_control_flow: true,
            auto_detect_entries: true,
            manual_entry_points: Vec::new(),
        }
    }
}

impl InterproceduralOptions {
    /// Create options with all features enabled.
    pub fn all_enabled() -> Self {
        Self::default()
    }

    /// Create options with only summary edges.
    pub fn summary_only() -> Self {
        InterproceduralOptions {
            summary_edges: true,
            reachability_analysis: false,
            exceptional_control_flow: false,
            auto_detect_entries: false,
            manual_entry_points: Vec::new(),
        }
    }

    /// Add manual entry point.
    pub fn with_entry_point(mut self, entry_symbol: String) -> Self {
        self.manual_entry_points.push(entry_symbol);
        self
    }
}

/// Perform interprocedural control flow analysis on a workspace.
pub fn analyze_interprocedural_cfg<P: AsRef<Path>>(
    workspace_path: P,
    options: InterproceduralOptions,
) -> Result<InterproceduralResult, NTreeError> {
    // Get workspace analysis
    let analysis_opts = AnalysisOptions {
        cfg_generation: true,
        complexity_analysis: false,
        basic_blocks: false,
        workspace_search: true,
        ..Default::default()
    };

    let workspace_result = AnalysisResult::from_source_code(
        workspace_path.as_ref().to_path_buf(),
        analysis_opts,
        true,
    )?;

    // Build interprocedural CFG
    build_interprocedural_cfg(workspace_result, options)
}

/// Build interprocedural CFG from workspace analysis.
fn build_interprocedural_cfg(
    workspace_result: AnalysisResult,
    options: InterproceduralOptions,
) -> Result<InterproceduralResult, NTreeError> {
    let mut interprocedural_cfg = InterproceduralCFG::new();
    let mut function_cfgs = HashMap::new();

    // For now, create a simplified implementation since CfgResult doesn't contain the actual CFG
    // This is a minimal implementation that demonstrates the API structure
    let symbol_store = workspace_result.symbol_store();

    // We would need to rebuild CFGs from the analysis results or store them differently
    // For this minimal implementation, we'll create placeholder CFGs
    for function_span in workspace_result.functions().all() {
        if let Ok(symbol_id) = symbol_store.find_by_name(&function_span.function) {
            // Create a placeholder CFG - in a real implementation, this would be reconstructed
            // from the analysis data or stored separately
            let placeholder_cfg = create_placeholder_cfg(&function_span.function);
            interprocedural_cfg.add_function_cfg(symbol_id.clone(), placeholder_cfg.clone());
            function_cfgs.insert(symbol_id, placeholder_cfg);
        }
    }

    // Generate summary edges if enabled
    if options.summary_edges {
        let call_edges: Vec<_> = workspace_result.call_graph().get_all_call_edges()
            .into_iter().cloned().collect();
        interprocedural_cfg.generate_summary_edges(&call_edges, symbol_store)?;
    }

    // Add entry points
    if options.auto_detect_entries {
        add_auto_detected_entries(&mut interprocedural_cfg, symbol_store)?;
    }

    for entry_name in &options.manual_entry_points {
        if let Ok(symbol_id) = symbol_store.find_by_name(entry_name) {
            interprocedural_cfg.add_entry_point(
                symbol_id,
                format!("Manual entry point: {}", entry_name),
            )?;
        }
    }

    // Compute reachability if enabled
    if options.reachability_analysis {
        interprocedural_cfg.compute_reachability()?;
    }

    // Generate exceptional edges if enabled
    if options.exceptional_control_flow {
        interprocedural_cfg.generate_exceptional_edges()?;
    }

    // Build result
    Ok(InterproceduralResult {
        entry_points: interprocedural_cfg.get_entry_points().to_vec(),
        reachability: interprocedural_cfg.get_reachability().clone(),
        call_sites: interprocedural_cfg.get_call_sites().clone(),
        exceptional_edges: interprocedural_cfg.get_exceptional_edges().to_vec(),
        function_exits: interprocedural_cfg.get_function_exits().clone(),
        function_cfgs,
        interprocedural_cfg,
    })
}

/// Create a placeholder CFG for demonstration.
fn create_placeholder_cfg(function_name: &str) -> ControlFlowGraph {
    use crate::models::{CfgNode, CfgEdge};

    let entry = CfgNode::new(0, format!("entry: {}", function_name));
    let exit = CfgNode::new(1, format!("exit: {}", function_name));
    let edge = CfgEdge::new(0, 1, "flow".to_string());

    ControlFlowGraph {
        nodes: vec![entry, exit],
        edges: vec![edge],
    }
}



/// Auto-detect common entry points.
fn add_auto_detected_entries(
    interprocedural_cfg: &mut InterproceduralCFG,
    symbol_store: &SymbolStore,
) -> Result<(), NTreeError> {
    // Common entry point names
    let entry_names = vec![
        "main",
        "test_",  // Rust test prefix
        "bench_", // Rust benchmark prefix
    ];

    for entry_pattern in entry_names {
        let symbols = symbol_store.find_symbols_matching(entry_pattern)?;
        for symbol_id in symbols {
            let reason = match entry_pattern {
                "main" => "Main function".to_string(),
                "test_" => "Test function".to_string(),
                "bench_" => "Benchmark function".to_string(),
                _ => format!("Entry point pattern: {}", entry_pattern),
            };

            match interprocedural_cfg.add_entry_point(symbol_id, reason) {
                Ok(_) => {},
                Err(_) => continue, // Skip if CFG not found for this symbol
            }
        }
    }

    Ok(())
}

/// Generate summary edges only (minimal interprocedural analysis).
pub fn generate_summary_edges<P: AsRef<Path>>(
    workspace_path: P,
) -> Result<Vec<InterproceduralEdge>, NTreeError> {
    let options = InterproceduralOptions::summary_only();
    let result = analyze_interprocedural_cfg(workspace_path, options)?;
    Ok(result.get_summary_edges().into_iter().cloned().collect())
}

/// Compute reachability from entry points.
pub fn compute_program_reachability<P: AsRef<Path>>(
    workspace_path: P,
) -> Result<HashMap<SymbolId, ReachabilityInfo>, NTreeError> {
    let options = InterproceduralOptions::all_enabled();
    let result = analyze_interprocedural_cfg(workspace_path, options)?;
    Ok(result.reachability)
}

/// Analyze exceptional control flow across functions.
pub fn analyze_exceptional_control_flow<P: AsRef<Path>>(
    workspace_path: P,
) -> Result<Vec<ExceptionalEdge>, NTreeError> {
    let options = InterproceduralOptions {
        summary_edges: true,
        exceptional_control_flow: true,
        ..Default::default()
    };
    let result = analyze_interprocedural_cfg(workspace_path, options)?;
    Ok(result.exceptional_edges)
}