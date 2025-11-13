use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Tests CFG-10: Try operator (?) with conditional early return
#[test]
fn test_try_operator_early_return() {
    let code = r#"
fn test_try() {
    let result = foo()?;
    println!("Success: {}", result);
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

    println!("Try operator CFG JSONL:");
    println!("{}", cfg.jsonl);

    // Verify try expression creates two paths
    assert!(cfg.jsonl.contains("\"try_expr(foo()?)\""));

    // Should have error edge to EXIT
    assert!(cfg.jsonl.contains("\"kind\":\"error\""));

    // Should have ok edge to continuation
    assert!(cfg.jsonl.contains("\"kind\":\"ok\""));

    // Next statement should still be reachable on ok path
    assert!(cfg.jsonl.contains("\"try_ok\""));
    assert!(cfg.jsonl.contains("\"println!"));
}

/// Tests CFG-10: panic! macro with exceptional termination
#[test]
fn test_panic_macro_termination() {
    let code = r#"
fn test_panic() {
    let x = 1;
    panic!("Something went wrong");
    let y = 2; // This should be unreachable
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

    println!("Panic macro CFG JSONL:");
    println!("{}", cfg.jsonl);

    // Verify panic expression exists
    assert!(cfg.jsonl.contains("\"panic_expr(panic!("));

    // Should have exceptional edge to EXIT
    assert!(cfg.jsonl.contains("\"kind\":\"exception\""));

    // Statement after panic should NOT be reachable
    // (Should not contain "let y = 2" as a separate node)
    let y_assignment_count = cfg.jsonl.matches("let y = 2").count();
    assert_eq!(y_assignment_count, 0); // Unreachable code
}

/// Tests mixed early-exit constructs in same function
#[test]
fn test_mixed_early_exits() {
    let code = r#"
fn test_mixed() {
    let result = foo()?;
    if result < 0 {
        panic!("Negative result");
    }
    let processed = bar(result)?;
    return processed;
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

    println!("Mixed early exits CFG JSONL:");
    println!("{}", cfg.jsonl);

    // Should have multiple try expressions
    assert!(cfg.jsonl.contains("\"try_expr(foo()?)\""));
    assert!(cfg.jsonl.contains("\"try_expr(bar(result)?)\""));

    // Should have panic statement (may be in if branch)
    assert!(cfg.jsonl.contains("\"panic!(\\\"Negative result\\\")"));

    // Should have both error/ok edges from try expressions
    assert!(cfg.jsonl.contains("\"kind\":\"error\""));
    assert!(cfg.jsonl.contains("\"kind\":\"ok\""));

    // Should have multiple try_ok blocks
    let ok_block_count = cfg.jsonl.matches("\"try_ok\"").count();
    assert!(ok_block_count >= 2);
}

/// Tests early-exit in control flow branches
#[test]
fn test_early_exit_in_branches() {
    let code = r#"
fn test_branches() {
    if condition {
        let result = risky_call()?;
        println!("{}", result);
    } else {
        panic!("No alternative");
    }
    println!("After if-else");
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

    println!("Early exit in branches CFG JSONL:");
    println!("{}", cfg.jsonl);

    // Should have if condition
    assert!(cfg.jsonl.contains("\"if (condition)\""));

    // Should contain early-exit constructs (try or panic statements)
    assert!(cfg.jsonl.contains("risky_call()") && cfg.jsonl.contains("panic!("));

    // Should have join after if-else for reachable code
    assert!(cfg.jsonl.contains("\"join\""));
}
