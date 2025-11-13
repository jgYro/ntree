use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Tests CFG-FOR-AG-01: Basic Rust for loop (iterator style)
#[test]
fn test_rust_for_loop() {
    let code = r#"
fn test_for() {
    for i in 0..10 {
        println!("{}", i);
    }
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
    assert_eq!(cfg.function_name, "test_for");

    // Print actual output to see what we're generating
    println!("For loop JSONL output:");
    println!("{}", cfg.jsonl);

    // Verify for loop structure in JSONL (normalized labels)
    assert!(cfg
        .jsonl
        .contains("\"for_loop(cond: 0..10.has_next, pattern: i)\""));
    assert!(cfg.jsonl.contains("\"for_loop_body\""));
    assert!(cfg.jsonl.contains("\"after_for_loop\""));

    // Verify edge types: true, false, back
    assert!(cfg.jsonl.contains("\"kind\":\"true\""));
    assert!(cfg.jsonl.contains("\"kind\":\"false\""));
    assert!(cfg.jsonl.contains("\"kind\":\"back\""));
}

/// Tests for loop with break statement
#[test]
fn test_for_with_break() {
    let code = r#"
fn test_for_break() {
    for i in 0..100 {
        if i > 5 {
            break;
        }
        println!("{}", i);
    }
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

    // Verify for loop structure
    assert!(cfg.jsonl.contains("\"for i in 0..100\""));
    assert!(cfg.jsonl.contains("\"for_body\""));
    assert!(cfg.jsonl.contains("\"after_for\""));

    // Should have back edge
    assert!(cfg.jsonl.contains("\"kind\":\"back\""));
}

/// Tests for loop with continue statement
#[test]
fn test_for_with_continue() {
    let code = r#"
fn test_for_continue() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("Odd: {}", i);
    }
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

    // Verify for loop structure
    assert!(cfg.jsonl.contains("\"for i in 0..10\""));
    assert!(cfg.jsonl.contains("\"for_body\""));

    // Should have back edge
    assert!(cfg.jsonl.contains("\"kind\":\"back\""));
}

/// Tests nested for loops
#[test]
fn test_nested_for_loops() {
    let code = r#"
fn test_nested_for() {
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                break;
            }
            println!("{}, {}", i, j);
        }
    }
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

    // Verify nested for loop structures
    assert!(cfg.jsonl.contains("\"for i in 0..3\""));
    assert!(cfg.jsonl.contains("\"for j in 0..3\""));

    // Should have multiple back edges (one per loop)
    let back_edge_count = cfg.jsonl.matches("\"kind\":\"back\"").count();
    assert!(back_edge_count >= 2);
}
