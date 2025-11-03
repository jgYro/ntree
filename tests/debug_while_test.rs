use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn debug_while_parsing() {
    let code = r#"
fn test_simple() {
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

    // Print the actual JSONL to see what we get
    println!("JSONL output:");
    println!("{}", cfg.jsonl);

    // This should pass so we can see the output
    assert!(cfg.jsonl.contains("ENTRY"));
}

#[test]
fn debug_break_parsing() {
    let code = r#"
fn test_break() {
    while true {
        break;
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

    // Print the actual JSONL to see what we get
    println!("Break JSONL output:");
    println!("{}", cfg.jsonl);

    // This should pass so we can see the output
    assert!(cfg.jsonl.contains("ENTRY"));
}

#[test]
fn debug_continue_parsing() {
    let code = r#"
fn test_continue() {
    while true {
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

    // Print the actual JSONL to see what we get
    println!("Continue JSONL output:");
    println!("{}", cfg.jsonl);

    // This should pass so we can see the output
    assert!(cfg.jsonl.contains("ENTRY"));
}

#[test]
fn debug_complex_break() {
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

    // Print the actual JSONL to see what we get
    println!("Complex Break JSONL output:");
    println!("{}", cfg.jsonl);

    // This should pass so we can see the output
    assert!(cfg.jsonl.contains("ENTRY"));
}