use crate::storage::symbol_core::SymbolId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Type of interprocedural edge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InterproceduralEdgeKind {
    /// Call edge from caller to callee entry
    Call,
    /// Return edge from callee exit to caller continuation
    Return,
    /// Exception edge for exceptional control flow
    Exception,
}

/// Interprocedural edge connecting CFG nodes across function boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterproceduralEdge {
    /// Source CFG node ID
    pub from_node: usize,
    /// Target CFG node ID
    pub to_node: usize,
    /// Kind of interprocedural edge
    pub kind: InterproceduralEdgeKind,
    /// Call site identifier for tracking
    pub callsite_id: Option<usize>,
    /// Target symbol for calls
    pub callee_sym: Option<SymbolId>,
    /// Source function symbol
    pub caller_sym: SymbolId,
}

impl InterproceduralEdge {
    /// Create a new call edge.
    pub fn new_call(
        from_node: usize,
        to_node: usize,
        callsite_id: usize,
        caller_sym: SymbolId,
        callee_sym: SymbolId,
    ) -> Self {
        InterproceduralEdge {
            from_node,
            to_node,
            kind: InterproceduralEdgeKind::Call,
            callsite_id: Some(callsite_id),
            callee_sym: Some(callee_sym),
            caller_sym,
        }
    }

    /// Create a new return edge.
    pub fn new_return(
        from_node: usize,
        to_node: usize,
        callsite_id: usize,
        caller_sym: SymbolId,
        callee_sym: SymbolId,
    ) -> Self {
        InterproceduralEdge {
            from_node,
            to_node,
            kind: InterproceduralEdgeKind::Return,
            callsite_id: Some(callsite_id),
            callee_sym: Some(callee_sym),
            caller_sym,
        }
    }

    /// Create a new exception edge.
    pub fn new_exception(
        from_node: usize,
        to_node: usize,
        caller_sym: SymbolId,
        callee_sym: Option<SymbolId>,
    ) -> Self {
        InterproceduralEdge {
            from_node,
            to_node,
            kind: InterproceduralEdgeKind::Exception,
            callsite_id: None,
            callee_sym,
            caller_sym,
        }
    }
}

/// Call site summary containing entry and exit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSiteSummary {
    /// Unique identifier for this call site
    pub callsite_id: usize,
    /// CFG node ID where call occurs
    pub caller_node: usize,
    /// Symbol of the calling function
    pub caller_sym: SymbolId,
    /// Target callee entry node
    pub callee_entry_id: usize,
    /// Target callee exit nodes
    pub callee_exit_ids: Vec<usize>,
    /// Target symbol
    pub callee_sym: SymbolId,
    /// Node to continue execution after call returns
    pub continuation_node: usize,
}

impl CallSiteSummary {
    /// Create a new call site summary.
    pub fn new(
        callsite_id: usize,
        caller_node: usize,
        caller_sym: SymbolId,
        callee_entry_id: usize,
        callee_exit_ids: Vec<usize>,
        callee_sym: SymbolId,
        continuation_node: usize,
    ) -> Self {
        CallSiteSummary {
            callsite_id,
            caller_node,
            caller_sym,
            callee_entry_id,
            callee_exit_ids,
            callee_sym,
            continuation_node,
        }
    }
}

/// Entry point for program-level CFG analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    /// Symbol ID of entry function
    pub sym_id: SymbolId,
    /// Reason this is considered an entry point
    pub reason: String,
    /// Entry CFG node ID
    pub entry_node: usize,
}

impl EntryPoint {
    /// Create a new entry point.
    pub fn new(sym_id: SymbolId, reason: String, entry_node: usize) -> Self {
        EntryPoint {
            sym_id,
            reason,
            entry_node,
        }
    }
}

/// Reachability information for functions and nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachabilityInfo {
    /// Symbol ID of the function
    pub sym_id: SymbolId,
    /// Whether this function is reachable
    pub reachable: bool,
    /// Set of reachable CFG nodes within this function
    pub reachable_nodes: HashSet<usize>,
    /// Entry points that can reach this function
    pub reached_from: Vec<SymbolId>,
}

impl ReachabilityInfo {
    /// Create new reachability info.
    pub fn new(sym_id: SymbolId) -> Self {
        ReachabilityInfo {
            sym_id,
            reachable: false,
            reachable_nodes: HashSet::new(),
            reached_from: Vec::new(),
        }
    }

    /// Mark as reachable from an entry point.
    pub fn mark_reachable_from(&mut self, entry_sym: SymbolId) {
        self.reachable = true;
        if !self.reached_from.contains(&entry_sym) {
            self.reached_from.push(entry_sym);
        }
    }

    /// Add reachable node.
    pub fn add_reachable_node(&mut self, node_id: usize) {
        self.reachable_nodes.insert(node_id);
    }
}

/// Types of exceptional exits from functions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExceptionExitKind {
    /// Normal exception (throw/raise)
    Exception,
    /// Early return with error (Rust ?, Option/Result)
    EarlyReturn,
    /// Panic/abort
    Panic,
    /// Timeout/interruption
    Interruption,
}

/// Function exit information for exceptional control flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionExit {
    /// Symbol of the function
    pub function_sym: SymbolId,
    /// Normal exit node (return statements)
    pub normal_exit_node: Option<usize>,
    /// Exceptional exit nodes by kind
    pub exceptional_exit_nodes: HashMap<ExceptionExitKind, Vec<usize>>,
}

impl FunctionExit {
    /// Create new function exit info.
    pub fn new(function_sym: SymbolId) -> Self {
        FunctionExit {
            function_sym,
            normal_exit_node: None,
            exceptional_exit_nodes: HashMap::new(),
        }
    }

    /// Set normal exit node.
    pub fn set_normal_exit(&mut self, node_id: usize) {
        self.normal_exit_node = Some(node_id);
    }

    /// Add exceptional exit node.
    pub fn add_exceptional_exit(&mut self, kind: ExceptionExitKind, node_id: usize) {
        self.exceptional_exit_nodes
            .entry(kind)
            .or_insert_with(Vec::new)
            .push(node_id);
    }

    /// Get all exit nodes.
    pub fn get_all_exits(&self) -> Vec<usize> {
        let mut exits = Vec::new();
        if let Some(normal) = self.normal_exit_node {
            exits.push(normal);
        }
        for exit_nodes in self.exceptional_exit_nodes.values() {
            exits.extend(exit_nodes);
        }
        exits
    }
}

/// Exceptional edge for cross-function exception handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionalEdge {
    /// Source node where exception originates
    pub from_node: usize,
    /// Target node (handler or function exit)
    pub to_node: usize,
    /// Kind of exceptional edge
    pub kind: ExceptionExitKind,
    /// Handler function symbol if handled
    pub handled_by: Option<SymbolId>,
    /// Source function symbol
    pub source_function: SymbolId,
}

impl ExceptionalEdge {
    /// Create new exceptional edge.
    pub fn new(
        from_node: usize,
        to_node: usize,
        kind: ExceptionExitKind,
        source_function: SymbolId,
    ) -> Self {
        ExceptionalEdge {
            from_node,
            to_node,
            kind,
            handled_by: None,
            source_function,
        }
    }

    /// Set handler function.
    pub fn with_handler(mut self, handler_sym: SymbolId) -> Self {
        self.handled_by = Some(handler_sym);
        self
    }
}
