use crate::models::{CfgNode, CfgEdge, CfgEdgeWrapper};
use super::mermaid_utils::{escape_mermaid_label, validate_mermaid};

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
            let label = &node.label;
            let escaped_label = escape_mermaid_label(label);

            // Use different shapes based on node type
            if label.starts_with("if (") {
                // Diamond shape for condition nodes
                let condition = label.trim_start_matches("if (").trim_end_matches(')');
                let escaped_condition = escape_mermaid_label(condition);
                mermaid.push_str(&format!("    {}{{\"{}\"}}\n", node.cfg_node, escaped_condition));
            } else if label == "ENTRY" || label == "EXIT" {
                // Rounded rectangle for entry/exit
                mermaid.push_str(&format!("    {}([{}])\n", node.cfg_node, label));
            } else if label == "join" {
                // Circle for join nodes (minimal visual impact)
                mermaid.push_str(&format!("    {}(( ))\n", node.cfg_node));
            } else {
                // Regular rectangle for statements
                mermaid.push_str(&format!("    {}[\"{}\"]\n", node.cfg_node, escaped_label));
            }
        }

        for edge in &self.edges {
            // Use different arrow styles and labels for different edge types
            match edge.kind.as_str() {
                "true" => {
                    mermaid.push_str(&format!(
                        "    {} -->|T| {}\n",
                        edge.from, edge.to
                    ));
                }
                "false" => {
                    mermaid.push_str(&format!(
                        "    {} -->|F| {}\n",
                        edge.from, edge.to
                    ));
                }
                "exit" => {
                    mermaid.push_str(&format!(
                        "    {} -.-> {}\n",
                        edge.from, edge.to
                    ));
                }
                _ => {
                    // Regular next edges
                    mermaid.push_str(&format!("    {} --> {}\n", edge.from, edge.to));
                }
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

    /// Validates the Mermaid output and returns it if valid.
    pub fn to_mermaid_validated(&self) -> Result<String, String> {
        let mermaid = self.to_mermaid();
        validate_mermaid(&mermaid)?;
        Ok(mermaid)
    }
}