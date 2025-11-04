use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test that for loops generate language-agnostic normalized labels
#[test]
fn test_for_loop_normalized_labels() {
    let code = r#"
fn test_for() {
    for item in items {
        println!("{}", item);
    }
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

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    // Verify normalized labels instead of language-specific syntax
    assert!(cfg.jsonl.contains("\"for_loop(cond: items.has_next, pattern: item)\""));
    assert!(cfg.jsonl.contains("\"for_loop_body\""));
    assert!(cfg.jsonl.contains("\"after_for_loop\""));

    println!("JSONL output:");
    println!("{}", cfg.jsonl);
}

/// Test that while loops generate language-agnostic normalized labels
#[test]
fn test_while_loop_normalized_labels() {
    let code = r#"
fn test_while() {
    let mut x = 0;
    while x < 10 {
        x = x + 1;
    }
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

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    // Verify normalized labels
    assert!(cfg.jsonl.contains("\"while_loop(cond: x < 10;)\""));
    assert!(cfg.jsonl.contains("\"while_loop_body\""));
    assert!(cfg.jsonl.contains("\"after_while_loop\""));

    println!("JSONL output:");
    println!("{}", cfg.jsonl);
}

/// Test that break/continue use normalized labels
#[test]
fn test_break_continue_normalized_labels() {
    let code = r#"
fn test_controls() {
    while true {
        break;
    }
    for item in items {
        continue;
    }
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

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    // Verify normalized control flow labels
    assert!(cfg.jsonl.contains("\"break_stmt\""));
    assert!(cfg.jsonl.contains("\"continue_stmt\""));

    println!("JSONL output:");
    println!("{}", cfg.jsonl);
}