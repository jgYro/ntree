use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Tests CFG-08: Basic match expression with multiple arms
#[test]
fn test_simple_match() {
    let code = r#"
fn test_match(x: i32) {
    let result = match x {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    return result;
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
    assert_eq!(cfg.function_name, "test_match");

    // Verify match structure in JSONL
    assert!(cfg.jsonl.contains("\"match x\""));
    assert!(cfg.jsonl.contains("\"match_join\""));

    // Verify arms are present
    assert!(cfg.jsonl.contains("\"arm: 1\"") || cfg.jsonl.contains("\"arm_start: 1\""));
    assert!(cfg.jsonl.contains("\"arm: 2\"") || cfg.jsonl.contains("\"arm_start: 2\""));
    assert!(cfg.jsonl.contains("\"_\""));

    // Verify Mermaid output
    assert!(cfg.mermaid.contains("match"));
}

/// Tests match expression with block arms
#[test]
fn test_match_with_blocks() {
    let code = r#"
fn test_match_blocks(x: i32) {
    match x {
        1 => {
            println!("One");
            return 1;
        },
        2 => {
            println!("Two");
            return 2;
        },
        _ => {
            println!("Other");
            return 0;
        },
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

    // Verify match structure
    assert!(cfg.jsonl.contains("\"match x\""));
    assert!(cfg.jsonl.contains("\"match_join\""));

    // Verify arm starts are present
    assert!(cfg.jsonl.contains("\"arm_start: 1\""));
    assert!(cfg.jsonl.contains("\"arm_start: 2\""));

    // Verify that each arm has return statements
    let return_count = cfg.jsonl.matches("\"return").count();
    assert!(return_count >= 3); // One for each arm

    // Verify exit edges
    assert!(cfg.jsonl.contains("\"kind\":\"exit\""));
}

/// Tests nested match expressions
#[test]
fn test_nested_match() {
    let code = r#"
fn test_nested_match(x: i32, y: i32) {
    match x {
        1 => {
            match y {
                10 => return 11,
                20 => return 21,
                _ => return 1,
            }
        },
        2 => return 2,
        _ => return 0,
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

    // Verify outer match structure
    assert!(cfg.jsonl.contains("\"match x\""));

    // Verify inner match structure
    assert!(cfg.jsonl.contains("\"match y\""));

    // Should have multiple match_join nodes
    let join_count = cfg.jsonl.matches("\"match_join\"").count();
    assert!(join_count >= 2); // One for each match

    // Should have multiple return statements
    let return_count = cfg.jsonl.matches("\"return").count();
    assert!(return_count >= 4);
}

/// Tests match with break/continue in loops
#[test]
fn test_match_in_loop_with_break() {
    let code = r#"
fn test_match_loop_break(values: &[i32]) {
    let mut i = 0;
    while i < values.len() {
        match values[i] {
            0 => break,
            1 => continue,
            _ => {
                println!("Value: {}", values[i]);
            },
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

    // Verify while loop structure
    assert!(cfg.jsonl.contains("\"while i < values.len()\""));

    // Verify match structure
    assert!(cfg.jsonl.contains("\"match values[i]\""));

    // Verify break and continue statements
    assert!(cfg.jsonl.contains("\"break\""));
    assert!(cfg.jsonl.contains("\"continue\""));

    // Verify edge types
    assert!(cfg.jsonl.contains("\"kind\":\"break\""));
    assert!(cfg.jsonl.contains("\"kind\":\"continue\""));
    assert!(cfg.jsonl.contains("\"kind\":\"back\""));
}

/// Tests match with complex patterns
#[test]
fn test_match_complex_patterns() {
    let code = r#"
fn test_complex_match(option: Option<i32>) {
    match option {
        Some(n) if n > 0 => {
            println!("Positive: {}", n);
        },
        Some(n) if n < 0 => {
            println!("Negative: {}", n);
        },
        Some(0) => {
            println!("Zero");
        },
        None => {
            println!("No value");
        },
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

    // Verify match structure
    assert!(cfg.jsonl.contains("\"match option\""));
    assert!(cfg.jsonl.contains("\"match_join\""));

    // Should have multiple arm starts for complex patterns
    let arm_count = cfg.jsonl.matches("\"arm").count();
    assert!(arm_count >= 4); // At least 4 arms

    // Verify different patterns are recognized
    assert!(cfg.jsonl.contains("Some") || cfg.jsonl.contains("None"));
}
