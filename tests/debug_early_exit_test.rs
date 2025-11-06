use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Debug test to see actual node types for try operator
#[test]
fn debug_try_operator() {
    let code = r#"
fn test() {
    let result = foo()?;
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

    println!("Try operator debug JSONL:");
    println!("{}", cfg.jsonl);

    assert!(cfg.jsonl.contains("ENTRY"));
}

/// Debug test to see actual node types for panic macro
#[test]
fn debug_panic_macro() {
    let code = r#"
fn test() {
    panic!("Error");
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

    println!("Panic macro debug JSONL:");
    println!("{}", cfg.jsonl);

    assert!(cfg.jsonl.contains("ENTRY"));
}