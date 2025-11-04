use ntree::{generate_cfg_ir, generate_cfg_ir_jsonl, generate_cfgs};
use ntree::extractors::cfg::ir_converter::CFGToIRConverter;
use std::io::Write;
use tempfile::NamedTempFile;

/// Tests CFG-11: Language-neutral IR serialization with stable schema
#[test]
fn test_language_neutral_ir_schema() {
    let code = r#"
fn server_run(x: i32) {
    if x > 0 {
        println!("Positive");
    } else {
        println!("Non-positive");
    }
    return x;
}
"#;

    let mut temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let function_irs = match generate_cfg_ir(temp_file.path()) {
        Ok(irs) => irs,
        Err(e) => panic!("Failed to generate CFG IR: {:?}", e),
    };

    assert_eq!(function_irs.len(), 1);
    let ir = &function_irs[0];

    // Verify stable schema structure
    assert_eq!(ir.function_name, "server_run");
    assert!(ir.nodes.len() > 0);
    assert!(ir.edges.len() > 0);

    println!("Language-neutral IR JSONL:");
    let jsonl = ir.to_jsonl();
    println!("{}", jsonl);

    // Verify stable schema format
    assert!(jsonl.contains("\"type\":\"CFGNode\""));
    assert!(jsonl.contains("\"type\":\"CFGEdge\""));
    assert!(jsonl.contains("\"func\":\"server_run\""));
    assert!(jsonl.contains("\"id\":\"N"));
    assert!(jsonl.contains("\"from\":\"N"));
    assert!(jsonl.contains("\"to\":\"N"));

    // Verify language-agnostic labels
    assert!(jsonl.contains("\"label\":\"ENTRY\""));
    assert!(jsonl.contains("\"label\":\"EXIT\""));
}

/// Tests round-trip: serialize to JSONL then parse back
#[test]
fn test_round_trip_jsonl() {
    let code = r#"
fn test_function() {
    let a = 1;
    let b = 2;
    if a > b {
        return a;
    }
    return b;
}
"#;

    let mut temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    // Generate original IR
    let original_irs = match generate_cfg_ir(temp_file.path()) {
        Ok(irs) => irs,
        Err(e) => panic!("Failed to generate original CFG IR: {:?}", e),
    };

    assert_eq!(original_irs.len(), 1);
    let original = &original_irs[0];

    // Serialize to JSONL
    let jsonl = CFGToIRConverter::serialize_to_jsonl(&original_irs);

    println!("Round-trip JSONL:");
    println!("{}", jsonl);

    // Parse back from JSONL
    let parsed_irs = match CFGToIRConverter::parse_from_jsonl(&jsonl) {
        Ok(irs) => irs,
        Err(e) => panic!("Failed to parse JSONL: {}", e),
    };

    assert_eq!(parsed_irs.len(), 1);
    let parsed = &parsed_irs[0];

    // Verify round-trip integrity
    assert_eq!(original.function_name, parsed.function_name);
    assert_eq!(original.node_count(), parsed.node_count());
    assert_eq!(original.edge_count(), parsed.edge_count());

    println!("Original: {} nodes, {} edges", original.node_count(), original.edge_count());
    println!("Parsed:   {} nodes, {} edges", parsed.node_count(), parsed.edge_count());

    // Verify nodes match
    for (orig_node, parsed_node) in original.nodes.iter().zip(parsed.nodes.iter()) {
        assert_eq!(orig_node.func, parsed_node.func);
        assert_eq!(orig_node.id, parsed_node.id);
        assert_eq!(orig_node.label, parsed_node.label);
    }

    // Verify edges match
    for (orig_edge, parsed_edge) in original.edges.iter().zip(parsed.edges.iter()) {
        assert_eq!(orig_edge.func, parsed_edge.func);
        assert_eq!(orig_edge.from, parsed_edge.from);
        assert_eq!(orig_edge.to, parsed_edge.to);
        assert_eq!(orig_edge.kind, parsed_edge.kind);
    }
}

/// Test IR generation vs original CFG for node/edge counts
#[test]
fn test_ir_vs_cfg_counts() {
    let code = r#"
fn compare_counts() {
    let x = 1;
    let y = 2;
    let z = x + y;
    return z;
}
"#;

    let mut temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    // Generate original CFG
    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    // Generate IR
    let irs = match generate_cfg_ir(temp_file.path()) {
        Ok(irs) => irs,
        Err(e) => panic!("Failed to generate IR: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    assert_eq!(irs.len(), 1);

    let cfg = &cfgs[0];
    let ir = &irs[0];

    println!("CFG JSONL:");
    println!("{}", cfg.jsonl);
    println!("\nIR JSONL:");
    println!("{}", ir.to_jsonl());

    // Count nodes and edges in original CFG
    let cfg_node_count = cfg.jsonl.matches("\"cfg_node\":").count();
    let cfg_edge_count = cfg.jsonl.matches("\"cfg_edge\":").count();

    // IR should have same counts
    assert_eq!(ir.node_count(), cfg_node_count);
    assert_eq!(ir.edge_count(), cfg_edge_count);

    println!("CFG: {} nodes, {} edges", cfg_node_count, cfg_edge_count);
    println!("IR:  {} nodes, {} edges", ir.node_count(), ir.edge_count());
}

/// Test early-exit constructs in IR format
#[test]
fn test_early_exit_ir_schema() {
    let code = r#"
fn test_early_exits() {
    let result = risky_call()?;
    if result < 0 {
        panic!("Invalid result");
    }
    return result;
}
"#;

    let mut temp_file = match NamedTempFile::new() {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let jsonl = match generate_cfg_ir_jsonl(temp_file.path()) {
        Ok(jsonl) => jsonl,
        Err(e) => panic!("Failed to generate CFG IR JSONL: {:?}", e),
    };

    println!("Early-exit IR JSONL:");
    println!("{}", jsonl);

    // Verify early-exit constructs in stable schema
    assert!(jsonl.contains("\"type\":\"CFGNode\""));
    assert!(jsonl.contains("\"type\":\"CFGEdge\""));
    assert!(jsonl.contains("\"func\":\"test_early_exits\""));

    // Should contain try and panic constructs with language-agnostic labels
    assert!(jsonl.contains("\"try_expr(") || jsonl.contains("risky_call()"));
    assert!(jsonl.contains("\"panic_expr(") || jsonl.contains("panic!("));

    // Should have error/exception edges
    assert!(jsonl.contains("\"kind\":\"error\"") || jsonl.contains("\"kind\":\"exception\""));
}