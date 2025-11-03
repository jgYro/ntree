use serde::{Deserialize, Serialize};

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