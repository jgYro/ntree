use crate::models::{CFGEdgeIR, CFGNodeIR, ControlFlowGraph, FunctionCFGIR};

/// Convert internal CFG representation to language-neutral IR.
pub struct CFGToIRConverter;

impl CFGToIRConverter {
    /// Convert a CFG to language-neutral IR format.
    pub fn convert_to_ir(
        cfg: &ControlFlowGraph,
        function_name: String,
        source_file: Option<String>,
    ) -> FunctionCFGIR {
        let mut ir = FunctionCFGIR::new(function_name.clone(), source_file);

        // Convert nodes
        for node in &cfg.nodes {
            let node_ir = CFGNodeIR::new(
                function_name.clone(),
                format!("N{}", node.cfg_node),
                node.label.clone(),
                format!("{}:{}", node.cfg_node, node.cfg_node), // Simple span for now
            );
            ir.add_node(node_ir);
        }

        // Convert edges
        for edge in &cfg.edges {
            let edge_ir = CFGEdgeIR::new(
                function_name.clone(),
                format!("N{}", edge.from),
                format!("N{}", edge.to),
                edge.kind.clone(),
            );
            ir.add_edge(edge_ir);
        }

        ir
    }

    /// Convert multiple CFGs to IR format.
    pub fn convert_multiple_to_ir(
        cfgs: &[(ControlFlowGraph, String)], // (cfg, function_name)
        source_file: Option<String>,
    ) -> Vec<FunctionCFGIR> {
        cfgs.iter()
            .map(|(cfg, func_name)| {
                Self::convert_to_ir(cfg, func_name.clone(), source_file.clone())
            })
            .collect()
    }

    /// Serialize multiple function CFGs to JSONL.
    pub fn serialize_to_jsonl(function_irs: &[FunctionCFGIR]) -> String {
        let mut jsonl = String::new();

        for func_ir in function_irs {
            jsonl.push_str(&func_ir.to_jsonl());
        }

        jsonl
    }

    /// Parse JSONL back to IR structs for round-trip testing.
    pub fn parse_from_jsonl(jsonl: &str) -> Result<Vec<FunctionCFGIR>, String> {
        let mut functions = Vec::new();
        let mut current_function: Option<FunctionCFGIR> = None;

        for line in jsonl.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Try to parse as node or edge
            if let Ok(node) = serde_json::from_str::<CFGNodeIR>(line) {
                // Ensure we have a function for this node
                if current_function.is_none()
                    || current_function.as_ref().unwrap().function_name != node.func {

                    // Finish previous function if exists
                    if let Some(func) = current_function.take() {
                        functions.push(func);
                    }

                    // Start new function
                    current_function = Some(FunctionCFGIR::new(node.func.clone(), None));
                }

                if let Some(ref mut func) = current_function {
                    func.add_node(node);
                }
            } else if let Ok(edge) = serde_json::from_str::<CFGEdgeIR>(line) {
                // Add edge to current function
                if let Some(ref mut func) = current_function {
                    if func.function_name == edge.func {
                        func.add_edge(edge);
                    }
                }
            }
        }

        // Add final function
        if let Some(func) = current_function {
            functions.push(func);
        }

        Ok(functions)
    }
}