use super::types::{CallSiteSummary, InterproceduralEdge};
use crate::core::NTreeError;
use crate::models::ControlFlowGraph;
use crate::storage::{CallEdge, SymbolId, SymbolStore};
use std::collections::{HashMap, HashSet};

/// Summary edge generator for interprocedural analysis.
#[derive(Debug)]
pub struct SummaryEdgeGenerator {
    call_sites: HashMap<usize, CallSiteSummary>,
    interprocedural_edges: Vec<InterproceduralEdge>,
    next_callsite_id: usize,
}

impl SummaryEdgeGenerator {
    /// Create new summary edge generator.
    pub fn new() -> Self {
        SummaryEdgeGenerator {
            call_sites: HashMap::new(),
            interprocedural_edges: Vec::new(),
            next_callsite_id: 0,
        }
    }

    /// Generate summary edges from call edges.
    pub fn generate_summary_edges(
        &mut self,
        call_edges: &[CallEdge],
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
        _symbol_store: &SymbolStore,
    ) -> Result<(), NTreeError> {
        for call_edge in call_edges {
            if call_edge.has_definitive_target() && !call_edge.targets.is_empty() {
                match self.create_summary_edge(call_edge, function_cfgs) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Warning: Failed to create summary edge for call: {}", e);
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    /// Create summary edges for a specific call.
    fn create_summary_edge(
        &mut self,
        call_edge: &CallEdge,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
    ) -> Result<(), NTreeError> {
        let callsite_id = self.next_callsite_id;
        self.next_callsite_id += 1;

        let caller_sym = call_edge.caller_sym.clone();
        let callee_sym = call_edge.targets[0].clone();

        let caller_node =
            match self.find_call_site_node(&caller_sym, &call_edge.site_span, function_cfgs) {
                Some(node) => node,
                None => {
                    return Err(NTreeError::InvalidInput(format!(
                        "Could not find call site node for span: {}",
                        call_edge.site_span
                    )))
                }
            };

        let (callee_entry, callee_exits) =
            match self.get_function_entry_exit(&callee_sym, function_cfgs) {
                Some(nodes) => nodes,
                None => {
                    return Err(NTreeError::InvalidInput(format!(
                        "Could not find entry/exit nodes for callee: {:?}",
                        callee_sym
                    )))
                }
            };

        let continuation_node =
            match self.find_continuation_node(&caller_sym, caller_node, function_cfgs) {
                Some(node) => node,
                None => {
                    return Err(NTreeError::InvalidInput(
                        "Could not find continuation node after call".to_string(),
                    ))
                }
            };

        let summary = CallSiteSummary::new(
            callsite_id,
            caller_node,
            caller_sym.clone(),
            callee_entry,
            callee_exits.clone(),
            callee_sym.clone(),
            continuation_node,
        );
        self.call_sites.insert(callsite_id, summary);

        let call_edge = InterproceduralEdge::new_call(
            caller_node,
            callee_entry,
            callsite_id,
            caller_sym.clone(),
            callee_sym.clone(),
        );
        self.interprocedural_edges.push(call_edge);

        for &exit_node in &callee_exits {
            let return_edge = InterproceduralEdge::new_return(
                exit_node,
                continuation_node,
                callsite_id,
                caller_sym.clone(),
                callee_sym.clone(),
            );
            self.interprocedural_edges.push(return_edge);
        }

        Ok(())
    }

    fn find_call_site_node(
        &self,
        caller_sym: &SymbolId,
        site_span: &str,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
    ) -> Option<usize> {
        if let Some(cfg) = function_cfgs.get(caller_sym) {
            for node in &cfg.nodes {
                if node.label.contains(site_span) || self.span_matches(&node.label, site_span) {
                    return Some(node.cfg_node);
                }
            }
            if !cfg.nodes.is_empty() {
                return Some(cfg.nodes[0].cfg_node);
            }
        }
        None
    }

    fn find_continuation_node(
        &self,
        caller_sym: &SymbolId,
        call_node: usize,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
    ) -> Option<usize> {
        if let Some(cfg) = function_cfgs.get(caller_sym) {
            for edge in &cfg.edges {
                if edge.from == call_node {
                    return Some(edge.to);
                }
            }
        }
        None
    }

    fn get_function_entry_exit(
        &self,
        function_sym: &SymbolId,
        function_cfgs: &HashMap<SymbolId, ControlFlowGraph>,
    ) -> Option<(usize, Vec<usize>)> {
        if let Some(cfg) = function_cfgs.get(function_sym) {
            if cfg.nodes.is_empty() {
                return None;
            }

            let entry = cfg.nodes[0].cfg_node;
            let mut exits = Vec::new();
            let has_outgoing: HashSet<usize> = cfg.edges.iter().map(|e| e.from).collect();

            for node in &cfg.nodes {
                let node_id = node.cfg_node;
                if !has_outgoing.contains(&node_id)
                    || node.label.contains("return")
                    || node.label.contains("exit")
                {
                    exits.push(node_id);
                }
            }

            if exits.is_empty() {
                exits.push(cfg.nodes.last().unwrap().cfg_node);
            }

            return Some((entry, exits));
        }
        None
    }

    fn span_matches(&self, node_label: &str, span: &str) -> bool {
        node_label.contains(span) || span.contains("line") && node_label.contains("line")
    }

    pub fn get_call_sites(&self) -> &HashMap<usize, CallSiteSummary> {
        &self.call_sites
    }

    pub fn get_interprocedural_edges(&self) -> &[InterproceduralEdge] {
        &self.interprocedural_edges
    }
}
