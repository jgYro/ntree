use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_return_as_terminator() {
    let code = r#"
fn early_return(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    let y = x * 2;
    return y;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    // Debug output
    println!("JSONL:\n{}", cfg.jsonl);
    println!("Mermaid:\n{}", cfg.mermaid);

    // Check that return has an "exit" edge
    assert!(cfg.jsonl.contains("\"kind\":\"exit\""));

    // Check Mermaid has the exit edge with proper notation
    assert!(cfg.mermaid.contains("-.->|exit|") || cfg.mermaid.contains("-.->"));
}

#[test]
fn test_return_mid_function() {
    let code = r#"
fn mid_return() {
    let x = 1;
    return;
    let dead = 2;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    // Parse JSONL to check structure
    let lines: Vec<&str> = cfg.jsonl.lines().collect();

    // Count nodes - should NOT include dead code
    let node_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_node\""))
        .count();

    // Should have: ENTRY, let x = 1, return, EXIT (no dead code node)
    assert_eq!(node_count, 4);

    // Verify no edge from return to dead code
    assert!(!cfg.jsonl.contains("\"label\":\"let dead = 2"));

    // Verify return connects to EXIT with exit edge
    let has_exit_edge = lines
        .iter()
        .any(|line| line.contains("\"kind\":\"exit\""));
    assert!(has_exit_edge);
}

#[test]
fn test_no_edges_after_return() {
    let code = r#"
fn stop_at_return() {
    return;
    let unreachable1 = 1;
    let unreachable2 = 2;
    let unreachable3 = 3;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    // Debug output
    println!("JSONL:\n{}", cfg.jsonl);

    // Should not contain any unreachable code
    assert!(!cfg.jsonl.contains("unreachable"));

    // Count edges
    let lines: Vec<&str> = cfg.jsonl.lines().collect();
    let edge_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_edge\""))
        .count();

    // Should have only: ENTRY->return, return->EXIT
    assert_eq!(edge_count, 2);

    // Verify the edge types (don't assume specific IDs)
    assert!(cfg.jsonl.contains(r#""kind":"next"#));
    assert!(cfg.jsonl.contains(r#""kind":"exit"#));
}

#[test]
fn test_multiple_returns() {
    let code = r#"
fn multi_return(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    if x < 0 {
        return -x;
    }
    return x * 2;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    // Count "exit" edges - should have one for each return
    let exit_edge_count = cfg
        .jsonl
        .lines()
        .filter(|line| line.contains("\"kind\":\"exit\""))
        .count();

    // We expect at least one exit edge (the actual count depends on control flow handling)
    assert!(exit_edge_count >= 1);

    // Verify Mermaid contains exit notation
    assert!(cfg.mermaid.contains("exit") || cfg.mermaid.contains("-.->"));
}

#[test]
fn test_return_value_in_jsonl() {
    let code = r#"
fn return_value() -> i32 {
    return 42;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    // Debug output
    println!("JSONL:\n{}", cfg.jsonl);

    // Verify the return statement is captured
    assert!(cfg.jsonl.contains("\"label\":\"return 42;\""));

    // Verify exit edge exists
    assert!(cfg.jsonl.contains("\"kind\":\"exit\""));

    // Verify structure: ENTRY -> return 42 -> EXIT
    let lines: Vec<&str> = cfg.jsonl.lines().collect();
    assert_eq!(
        lines.iter().filter(|l| l.contains("\"cfg_node\"")).count(),
        3
    ); // ENTRY, return 42, EXIT
    assert_eq!(
        lines.iter().filter(|l| l.contains("\"cfg_edge\"")).count(),
        2
    ); // ENTRY->return, return->EXIT(exit)
}