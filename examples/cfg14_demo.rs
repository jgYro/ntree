use ntree::api::generate_cfg_ir;
use ntree::models::ir::CFGEdgeIR;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CFG-14 Edge Annotations Demo ===\n");

    // Generate CFG IR with annotations for test file
    let cfg_ir_results = generate_cfg_ir("test_sample.rs")?;

    println!("1. CFG IR with Annotations:");
    for cfg_ir in &cfg_ir_results {
        println!("Function: {}", cfg_ir.function_name);

        let jsonl = cfg_ir.to_jsonl();
        let lines: Vec<&str> = jsonl.lines().take(3).collect();

        for line in lines {
            println!("  {}", line);
        }
        println!("  ... ({} total lines)\n", jsonl.lines().count());
    }

    // Demonstrate different provenance levels
    println!("2. Different Provenance Examples:");

    // Tree-sitter (default) - exact confidence
    let tree_sitter_edge = CFGEdgeIR::new(
        "example".to_string(),
        "N1".to_string(),
        "N2".to_string(),
        "true".to_string(),
    );

    // Compiler-inferred edge
    let compiler_edge = CFGEdgeIR::with_provenance(
        "example".to_string(),
        "N2".to_string(),
        "N3".to_string(),
        "call".to_string(),
        "compiler".to_string(),
        "inferred".to_string(),
    );

    // LSP-uncertain edge
    let lsp_edge = CFGEdgeIR::with_provenance(
        "example".to_string(),
        "N3".to_string(),
        "N4".to_string(),
        "exception".to_string(),
        "lsp".to_string(),
        "uncertain".to_string(),
    );

    println!("Tree-sitter edge: {}", serde_json::to_string(&tree_sitter_edge)?);
    println!("Compiler edge:    {}", serde_json::to_string(&compiler_edge)?);
    println!("LSP edge:         {}", serde_json::to_string(&lsp_edge)?);

    println!("\n3. CFG-14 Specification Match:");
    println!("Expected: {{\"type\":\"CFGEdge\",\"from\":\"N1\",\"to\":\"N2\",\"kind\":\"true\",\"source\":\"tree-sitter\",\"confidence\":\"exact\"}}");
    println!("Actual:   {}", serde_json::to_string(&tree_sitter_edge)?);

    // Verify the match
    let json = serde_json::to_string(&tree_sitter_edge)?;
    let has_required_fields = json.contains("\"source\":\"tree-sitter\"")
        && json.contains("\"confidence\":\"exact\"")
        && json.contains("\"type\":\"CFGEdge\"");

    println!("✓ CFG-14 format compliance: {}", has_required_fields);

    Ok(())
}