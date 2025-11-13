use super::interprocedural::{
    CallSiteSummary, EntryPoint, ExceptionalEdge, FunctionExit, InterproceduralEdge,
    ReachabilityInfo,
};
use super::interprocedural::{ExceptionAnalyzer, ReachabilityAnalyzer, SummaryEdgeGenerator};
use crate::core::NTreeError;
use crate::models::ControlFlowGraph;
use crate::storage::{CallEdge, SymbolId, SymbolStore};
use std::collections::HashMap;

/// Interprocedural Control Flow Graph manager.
#[derive(Debug)]
pub struct InterproceduralCFG {
    /// Individual function CFGs
    function_cfgs: HashMap<SymbolId, ControlFlowGraph>,
    /// Summary edge generator
    summary_generator: SummaryEdgeGenerator,
    /// Reachability analyzer
    reachability_analyzer: ReachabilityAnalyzer,
    /// Exception analyzer
    exception_analyzer: ExceptionAnalyzer,
}

impl InterproceduralCFG {
    /// Create a new interprocedural CFG.
    pub fn new() -> Self {
        InterproceduralCFG {
            function_cfgs: HashMap::new(),
            summary_generator: SummaryEdgeGenerator::new(),
            reachability_analyzer: ReachabilityAnalyzer::new(),
            exception_analyzer: ExceptionAnalyzer::new(),
        }
    }

    /// Add a function CFG to the interprocedural graph.
    pub fn add_function_cfg(&mut self, symbol_id: SymbolId, cfg: ControlFlowGraph) {
        self.function_cfgs.insert(symbol_id.clone(), cfg);
        self.reachability_analyzer.add_function(symbol_id.clone());
        self.exception_analyzer.add_function(symbol_id);
    }

    /// Generate summary edges from call edges.
    pub fn generate_summary_edges(
        &mut self,
        call_edges: &[CallEdge],
        symbol_store: &SymbolStore,
    ) -> Result<(), NTreeError> {
        self.summary_generator
            .generate_summary_edges(call_edges, &self.function_cfgs, symbol_store)
    }

    /// Add entry point for program-level analysis.
    pub fn add_entry_point(&mut self, sym_id: SymbolId, reason: String) -> Result<(), NTreeError> {
        self.reachability_analyzer
            .add_entry_point(sym_id, reason, &self.function_cfgs)
    }

    /// Compute reachability from all entry points.
    pub fn compute_reachability(&mut self) -> Result<(), NTreeError> {
        self.reachability_analyzer.compute_reachability(
            &self.function_cfgs,
            self.summary_generator.get_interprocedural_edges(),
        )
    }

    /// Generate exceptional control flow edges.
    pub fn generate_exceptional_edges(&mut self) -> Result<(), NTreeError> {
        self.exception_analyzer.generate_exceptional_edges(
            &self.function_cfgs,
            self.summary_generator.get_call_sites(),
        )
    }

    /// Get interprocedural edges.
    pub fn get_interprocedural_edges(&self) -> &[InterproceduralEdge] {
        self.summary_generator.get_interprocedural_edges()
    }

    /// Get call site summaries.
    pub fn get_call_sites(&self) -> &HashMap<usize, CallSiteSummary> {
        self.summary_generator.get_call_sites()
    }

    /// Get entry points.
    pub fn get_entry_points(&self) -> &[EntryPoint] {
        self.reachability_analyzer.get_entry_points()
    }

    /// Get reachability information.
    pub fn get_reachability(&self) -> &HashMap<SymbolId, ReachabilityInfo> {
        self.reachability_analyzer.get_reachability()
    }

    /// Get exceptional edges.
    pub fn get_exceptional_edges(&self) -> &[ExceptionalEdge] {
        self.exception_analyzer.get_exceptional_edges()
    }

    /// Get function exits.
    pub fn get_function_exits(&self) -> &HashMap<SymbolId, FunctionExit> {
        self.exception_analyzer.get_function_exits()
    }
}
