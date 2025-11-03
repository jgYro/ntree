use crate::models::{CfgNode, CfgEdge, CfgEdgeWrapper, ControlFlowGraph};

/// Export a Control Flow Graph to JSONL format.
pub fn export_jsonl(cfg: &ControlFlowGraph) -> String {
    let mut jsonl = String::new();

    for node in &cfg.nodes {
        match serde_json::to_string(node) {
            Ok(json) => {
                jsonl.push_str(&json);
                jsonl.push('\n');
            }
            Err(_) => {}
        }
    }

    for edge in &cfg.edges {
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