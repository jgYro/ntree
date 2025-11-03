use serde::{Deserialize, Serialize};

/// Represents a node in the Control Flow Graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgNode {
    /// Unique identifier for the node
    pub cfg_node: usize,
    /// Label describing the node content
    pub label: String,
}

impl CfgNode {
    /// Creates a new CFG node.
    pub fn new(id: usize, label: String) -> Self {
        CfgNode {
            cfg_node: id,
            label,
        }
    }
}

/// Represents an edge in the Control Flow Graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgEdge {
    /// Source node ID
    pub from: usize,
    /// Target node ID
    pub to: usize,
    /// Type of edge (e.g., "next" for sequential flow)
    pub kind: String,
}

impl CfgEdge {
    /// Creates a new CFG edge.
    pub fn new(from: usize, to: usize, kind: String) -> Self {
        CfgEdge { from, to, kind }
    }
}

/// Wrapper for serializing edges in the expected format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgEdgeWrapper {
    pub cfg_edge: CfgEdge,
}

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
        let mut mermaid = String::from("graph TD\n");

        for node in &self.nodes {
            let label = node.label.replace('"', "'");
            mermaid.push_str(&format!("    {}[\"{}\"]\n", node.cfg_node, label));
        }

        for edge in &self.edges {
            // Use different arrow styles for different edge types
            let arrow = match edge.kind.as_str() {
                "exit" => "-.->",  // Dotted arrow for exit edges
                _ => "-->",        // Regular arrow for next edges
            };

            // Add edge label for non-next edges
            if edge.kind != "next" {
                mermaid.push_str(&format!(
                    "    {} {}|{}| {}\n",
                    edge.from, arrow, edge.kind, edge.to
                ));
            } else {
                mermaid.push_str(&format!("    {} {} {}\n", edge.from, arrow, edge.to));
            }
        }

        mermaid
    }

    /// Converts to JSONL format.
    pub fn to_jsonl(&self) -> String {
        let mut jsonl = String::new();

        for node in &self.nodes {
            match serde_json::to_string(node) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(_) => {}
            }
        }

        for edge in &self.edges {
            let wrapper = CfgEdgeWrapper {
                cfg_edge: edge.clone(),
            };
            match serde_json::to_string(&wrapper) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(_) => {}
            }
        }

        jsonl
    }
}