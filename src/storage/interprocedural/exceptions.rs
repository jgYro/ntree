use std::collections::HashMap;
use crate::core::NTreeError;
use crate::models::ControlFlowGraph;
use crate::storage::SymbolId;
use super::types::{
    FunctionExit, ExceptionalEdge, ExceptionExitKind, CallSiteSummary
};

/// Exception flow analyzer for interprocedural analysis.
#[derive(Debug)]
pub struct ExceptionAnalyzer {
    function_exits: HashMap<SymbolId, FunctionExit>,
    exceptional_edges: Vec<ExceptionalEdge>,
}

impl ExceptionAnalyzer {
    /// Create new exception analyzer.
    pub fn new() -> Self {
        ExceptionAnalyzer {
            function_exits: HashMap::new(),
            exceptional_edges: Vec::new(),
        }
    }

    /// Initialize function exit info.
    pub fn add_function(&mut self, symbol_id: SymbolId) {
        self.function_exits.insert(symbol_id.clone(), FunctionExit::new(symbol_id));
    }

    /// Generate exceptional control flow edges.
    pub fn generate_exceptional_edges(
        &mut self,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
        call_sites: &HashMap<usize, CallSiteSummary>
    ) -> Result<(), NTreeError> {
        let function_syms: Vec<SymbolId> = function_cfgs.keys().cloned().collect();
        for function_sym in function_syms {
            if let Some(cfg) = function_cfgs.get(&function_sym).cloned() {
                self.analyze_function_exceptions(function_sym, &cfg)?;
            }
        }

        self.connect_interprocedural_exceptions(call_sites)?;
        Ok(())
    }

    /// Analyze exceptional control flow within a function.
    fn analyze_function_exceptions(
        &mut self,
        function_sym: SymbolId,
        cfg: &ControlFlowGraph
    ) -> Result<(), NTreeError> {
        let mut function_exit = FunctionExit::new(function_sym.clone());

        for node in &cfg.nodes {
            if self.is_exception_node(&node.label) {
                let exception_kind = self.classify_exception_type(&node.label);
                function_exit.add_exceptional_exit(exception_kind, node.cfg_node);
            }

            if self.is_normal_exit(&node.label) {
                function_exit.set_normal_exit(node.cfg_node);
            }
        }

        self.function_exits.insert(function_sym, function_exit);
        Ok(())
    }

    /// Connect exceptional control flow across function boundaries.
    fn connect_interprocedural_exceptions(&mut self, call_sites: &HashMap<usize, CallSiteSummary>) -> Result<(), NTreeError> {
        for summary in call_sites.values() {
            if let Some(callee_exits) = self.function_exits.get(&summary.callee_sym) {
                for (exception_kind, exit_nodes) in &callee_exits.exceptional_exit_nodes {
                    for &exit_node in exit_nodes {
                        match self.find_exception_handler(
                            summary.caller_sym.clone(),
                            summary.caller_node,
                            exception_kind
                        ) {
                            Some(handler_node) => {
                                let exc_edge = ExceptionalEdge::new(
                                    exit_node,
                                    handler_node,
                                    exception_kind.clone(),
                                    summary.caller_sym.clone(),
                                );
                                self.exceptional_edges.push(exc_edge);
                            },
                            None => {
                                if let Some(caller_exits) = self.function_exits.get(&summary.caller_sym) {
                                    if let Some(caller_exc_exits) = caller_exits.exceptional_exit_nodes.get(exception_kind) {
                                        for &caller_exit in caller_exc_exits {
                                            let exc_edge = ExceptionalEdge::new(
                                                exit_node,
                                                caller_exit,
                                                exception_kind.clone(),
                                                summary.caller_sym.clone(),
                                            );
                                            self.exceptional_edges.push(exc_edge);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn is_exception_node(&self, label: &str) -> bool {
        label.contains("throw") || label.contains("raise") ||
        label.contains("panic") || label.contains("?") ||
        label.contains("unwrap") || label.contains("expect")
    }

    fn is_normal_exit(&self, label: &str) -> bool {
        label.contains("return") && !self.is_exception_node(label)
    }

    fn classify_exception_type(&self, label: &str) -> ExceptionExitKind {
        if label.contains("panic") || label.contains("unwrap") || label.contains("expect") {
            ExceptionExitKind::Panic
        } else if label.contains("?") || label.contains("Result") || label.contains("Option") {
            ExceptionExitKind::EarlyReturn
        } else {
            ExceptionExitKind::Exception
        }
    }

    fn find_exception_handler(&self, _caller_sym: SymbolId, _call_node: usize, _exception_kind: &ExceptionExitKind) -> Option<usize> {
        None
    }

    pub fn get_function_exits(&self) -> &HashMap<SymbolId, FunctionExit> {
        &self.function_exits
    }

    pub fn get_exceptional_edges(&self) -> &[ExceptionalEdge] {
        &self.exceptional_edges
    }
}