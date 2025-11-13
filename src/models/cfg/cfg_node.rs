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
