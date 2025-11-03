use crate::models::{CfgNode, CfgEdge, CfgEdgeWrapper};
use crate::export::{export_mermaid, export_jsonl, export_mermaid_validated};

/// Represents a complete Control Flow Graph.
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub nodes: Vec<CfgNode>,
    pub edges: Vec<CfgEdge>,
}

impl ControlFlowGraph {
    /// Creates a new empty CFG.
    pub fn new() -> Self {
        ControlFlowGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Adds a node to the CFG.
    pub fn add_node(&mut self, node: CfgNode) {
        self.nodes.push(node);
    }

    /// Adds an edge to the CFG.
    pub fn add_edge(&mut self, edge: CfgEdge) {
        self.edges.push(edge);
    }

    /// Generates Mermaid diagram syntax.
    pub fn to_mermaid(&self) -> String {
        export_mermaid(self)
    }

    /// Converts to JSONL format.
    pub fn to_jsonl(&self) -> String {
        export_jsonl(self)
    }

    /// Validates the Mermaid output and returns it if valid.
    pub fn to_mermaid_validated(&self) -> Result<String, String> {
        export_mermaid_validated(self)
    }
}