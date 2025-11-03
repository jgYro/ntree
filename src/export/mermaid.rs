use crate::models::{CfgNode, CfgEdge, ControlFlowGraph};

/// Utilities for Mermaid diagram generation.

/// Escape a label for use in Mermaid diagrams.
/// Mermaid has specific requirements for escaping special characters.
pub fn escape_mermaid_label(label: &str) -> String {
    // First, escape ampersands (must be done first to avoid double-escaping)
    let escaped = label.replace('&', "&amp;");

    // Then escape other HTML entities
    let escaped = escaped.replace('"', "&quot;");
    let escaped = escaped.replace('\'', "&apos;");
    let escaped = escaped.replace('<', "&lt;");
    let escaped = escaped.replace('>', "&gt;");

    // Escape backslashes
    let escaped = escaped.replace('\\', "\\\\");

    // Replace newlines with spaces for readability
    let escaped = escaped.replace('\n', " ");

    escaped
}

/// Validate that a Mermaid diagram is syntactically correct.
/// Returns Ok(()) if valid, or Err with a description of the problem.
pub fn validate_mermaid(mermaid: &str) -> Result<(), String> {
    // Check for common issues
    let lines: Vec<&str> = mermaid.lines().collect();

    if lines.is_empty() {
        return Err("Empty diagram".to_string());
    }

    if !lines[0].starts_with("graph") {
        return Err("Diagram must start with 'graph'".to_string());
    }

    // Check for unescaped quotes in node labels
    for (i, line) in lines.iter().enumerate() {
        if line.contains('[') && line.contains(']') {
            // Extract content between brackets
            if let Some(start) = line.find('[') {
                if let Some(end) = line.rfind(']') {
                    let label = &line[start+1..end];

                    // Check for problematic characters
                    if label.contains('\'') && !label.contains("&apos;") {
                        return Err(format!("Line {}: Unescaped single quote in label", i + 1));
                    }
                    if label.contains('"') && !label.contains("&quot;") {
                        return Err(format!("Line {}: Unescaped double quote in label", i + 1));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Export a Control Flow Graph to Mermaid diagram format.
pub fn export_mermaid(cfg: &ControlFlowGraph) -> String {
    let mut mermaid = String::from("graph TD\n");

    for node in &cfg.nodes {
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

    for edge in &cfg.edges {
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

/// Export a Control Flow Graph to validated Mermaid diagram format.
pub fn export_mermaid_validated(cfg: &ControlFlowGraph) -> Result<String, String> {
    let mermaid = export_mermaid(cfg);
    validate_mermaid(&mermaid)?;
    Ok(mermaid)
}