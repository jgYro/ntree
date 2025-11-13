use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Tests CFG-07: Break statement in while loop
#[test]
fn test_break_statement() {
    let code = r#"
fn test_break() {
    let mut x = 0;
    while true {
        if x > 5 {
            break;
        }
        x = x + 1;
    }
    println!("Done");
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

    // Verify break statement exists
    assert!(cfg.jsonl.contains("\"break\""));

    // Verify break edge goes to after_while
    assert!(cfg.jsonl.contains("\"kind\":\"break\""));

    // Verify while loop structure
    assert!(cfg.jsonl.contains("\"while true\""));
    assert!(cfg.jsonl.contains("\"after_while\""));

    // Statement after loop should be reachable
    assert!(cfg.jsonl.contains("\"println!"));
}

/// Tests CFG-07: Continue statement in while loop
#[test]
fn test_continue_statement() {
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

    // Verify continue statement exists
    assert!(cfg.jsonl.contains("\"continue\""));

    // Verify continue edge goes to condition
    assert!(cfg.jsonl.contains("\"kind\":\"continue\""));

    // Verify while loop structure
    assert!(cfg.jsonl.contains("\"while x < 10\""));

    // Statement after continue should exist but not be reached from continue
    assert!(cfg.jsonl.contains("\"println!"));
}

/// Tests nested loops with break/continue targeting correct loop
#[test]
fn test_nested_loops_break_continue() {
    let code = r#"
fn test_nested_break_continue() {
    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 5 {
            if j == 2 {
                break; // Should break inner loop only
            }
            if j == 1 {
                j = j + 1;
                continue; // Should continue inner loop only
            }
            j = j + 1;
        }
        println!("Outer loop: {}", i);
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

    // Verify nested while loop structures
    assert!(cfg.jsonl.contains("\"while i < 3\""));
    assert!(cfg.jsonl.contains("\"while j < 5\""));

    // Verify break and continue statements
    assert!(cfg.jsonl.contains("\"break\""));
    assert!(cfg.jsonl.contains("\"continue\""));

    // Verify edge types
    assert!(cfg.jsonl.contains("\"kind\":\"break\""));
    assert!(cfg.jsonl.contains("\"kind\":\"continue\""));

    // Should have multiple back edges (one per loop)
    let back_edge_count = cfg.jsonl.matches("\"kind\":\"back\"").count();
    assert!(back_edge_count >= 2);

    // Outer loop statement should be reachable after inner loop
    assert!(cfg.jsonl.contains("\"println!"));
}

/// Tests multiple break statements in same loop
#[test]
fn test_multiple_breaks() {
    let code = r#"
fn test_multiple_breaks(values: &[i32]) {
    let mut i = 0;
    while i < values.len() {
        if values[i] == 0 {
            break;
        }
        if values[i] < 0 {
            println!("Negative found");
            break;
        }
        if values[i] > 100 {
            println!("Too large");
            break;
        }
        i = i + 1;
    }
    println!("Loop ended");
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

    // Should have multiple break statements
    let break_count = cfg.jsonl.matches("\"break\"").count();
    assert!(break_count >= 3);

    // Should have multiple break edges
    let break_edge_count = cfg.jsonl.matches("\"kind\":\"break\"").count();
    assert!(break_edge_count >= 3);

    // All should target the same after_while node
    assert!(cfg.jsonl.contains("\"after_while\""));

    // Statement after loop should be reachable
    assert!(cfg.jsonl.contains("\"Loop ended\""));
}

/// Tests break and continue in complex control structures
#[test]
fn test_break_continue_with_if_else() {
    let code = r#"
fn test_complex_control() {
    let mut x = 0;
    while x < 20 {
        if x < 5 {
            x = x + 1;
            continue;
        } else if x > 15 {
            break;
        } else {
            if x == 10 {
                x = x + 2;
                continue;
            }
            x = x + 1;
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

    // Verify while loop structure
    assert!(cfg.jsonl.contains("\"while x < 20\""));

    // Verify break and continue statements
    assert!(cfg.jsonl.contains("\"break\""));
    assert!(cfg.jsonl.contains("\"continue\""));

    // Should have multiple continue statements
    let continue_count = cfg.jsonl.matches("\"continue\"").count();
    assert!(continue_count >= 2);

    // Verify edge types
    assert!(cfg.jsonl.contains("\"kind\":\"break\""));
    assert!(cfg.jsonl.contains("\"kind\":\"continue\""));

    // Should also have if/else structure
    assert!(cfg.jsonl.contains("\"if x < 5\""));
    assert!(cfg.jsonl.contains("\"join\""));
}
