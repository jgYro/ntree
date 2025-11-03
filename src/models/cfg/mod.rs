// Re-export from submodules
pub use self::cfg_node::CfgNode;
pub use self::cfg_edge::{CfgEdge, CfgEdgeWrapper};
pub use self::cfg_graph::ControlFlowGraph;
pub use self::mermaid_utils::{escape_mermaid_label, validate_mermaid};

mod cfg_node;
mod cfg_edge;
mod cfg_graph;
mod mermaid_utils;