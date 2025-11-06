use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Tests CFG-06: Basic while loop with condition + back edge
#[test]
fn test_simple_while_loop() {
    let code = r#"
fn test_while() {
    let mut x = 0;
    while x < 10 {
        x = x + 1;
    }
    return x;
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
    assert_eq!(cfg.function_name, "test_while");

    // Verify while loop structure in JSONL
    assert!(cfg.jsonl.contains("\"while x < 10;\""));
    assert!(cfg.jsonl.contains("\"while_body\""));
    assert!(cfg.jsonl.contains("\"after_while\""));

    // Verify edge types: true, false, back
    assert!(cfg.jsonl.contains("\"kind\":\"true\""));
    assert!(cfg.jsonl.contains("\"kind\":\"false\""));
    assert!(cfg.jsonl.contains("\"kind\":\"back\""));

    // Verify Mermaid output
    assert!(cfg.mermaid.contains("while"));
    assert!(cfg.mermaid.contains("true"));
    assert!(cfg.mermaid.contains("false"));
}

/// Tests while loop with break statement
#[test]
fn test_while_with_break() {
    let code = r#"
fn test_break() {
    let mut x = 0;
    while x < 100 {
        if x > 5 {
            break;
        }
        x = x + 1;
    }
    return x;
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

    // Verify break statement and edge
    assert!(cfg.jsonl.contains("\"break\""));
    assert!(cfg.jsonl.contains("\"kind\":\"break\""));

    // Verify while structure still exists
    assert!(cfg.jsonl.contains("\"while x < 100;\""));
    assert!(cfg.jsonl.contains("\"after_while\""));
}

/// Tests while loop with continue statement
#[test]
fn test_while_with_continue() {
    let code = r#"
fn test_continue() {
    let mut x = 0;
    while x < 10 {
        x = x + 1;
        if x % 2 == 0 {
            continue;
        }
        println!("Odd: {}", x);
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

    // Verify continue statement and edge
    assert!(cfg.jsonl.contains("\"continue\""));
    assert!(cfg.jsonl.contains("\"kind\":\"continue\""));

    // Verify while structure
    assert!(cfg.jsonl.contains("\"while x < 10;\""));
}

/// Tests nested while loops with break/continue
#[test]
fn test_nested_while_loops() {
    let code = r#"
fn test_nested() {
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 3 {
            if i == j {
                break;
            }
            j = j + 1;
        }
        i = i + 1;
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

    // Verify nested loop structures
    assert!(cfg.jsonl.contains("\"while i < 3;\""));
    assert!(cfg.jsonl.contains("\"while j < 3;\""));

    // Verify break goes to inner loop's after node
    assert!(cfg.jsonl.contains("\"break\""));
    assert!(cfg.jsonl.contains("\"kind\":\"break\""));

    // Should have multiple back edges
    let back_edge_count = cfg.jsonl.matches("\"kind\":\"back\"").count();
    assert!(back_edge_count >= 2); // At least one for each loop
}

/// Tests while loop with empty body
#[test]
fn test_while_empty_body() {
    let code = r#"
fn test_empty_while() {
    let mut x = 0;
    while x < 5 {
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

    // Verify basic while structure
    assert!(cfg.jsonl.contains("\"while x < 5;\""));
    assert!(cfg.jsonl.contains("\"while_body\""));
    assert!(cfg.jsonl.contains("\"after_while\""));

    // Should still have true/false/back edges
    assert!(cfg.jsonl.contains("\"kind\":\"true\""));
    assert!(cfg.jsonl.contains("\"kind\":\"false\""));
    assert!(cfg.jsonl.contains("\"kind\":\"back\""));
}