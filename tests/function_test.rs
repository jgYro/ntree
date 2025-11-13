use ntree::{functions_to_jsonl, list_functions, FunctionSpan};
use std::fs;

#[test]
fn test_function_span_creation() {
    let span = FunctionSpan::new(
        "my_func".to_string(),
        "10:5-15:10".to_string(),
        Some("10:10-15:5".to_string()),
    );

    assert_eq!(span.function, "my_func");
    assert_eq!(span.span, "10:5-15:10");
    assert_eq!(span.body, Some("10:10-15:5".to_string()));
}

#[test]
fn test_format_span() {
    let formatted = FunctionSpan::format_span(0, 0, 5, 10);
    assert_eq!(formatted, "1:1–6:11");
}

#[test]
fn test_function_with_empty_body() {
    let test_file = "test_empty_func.rs";
    let content = r#"
fn empty_function() {}

fn another_empty() {
}
"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let functions = match list_functions(test_file) {
        Ok(f) => f,
        Err(e) => panic!("Failed to list functions: {:?}", e),
    };

    assert_eq!(functions.len(), 2);
    assert!(functions[0].body.is_some());
    assert!(functions[1].body.is_some());

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_function_with_body() {
    let test_file = "test_func_body.rs";
    let content = r#"
fn calculate(x: i32, y: i32) -> i32 {
    let sum = x + y;
    let product = x * y;
    sum + product
}

pub fn check(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    for c in input.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }

    true
}
"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let functions = match list_functions(test_file) {
        Ok(f) => f,
        Err(e) => panic!("Failed to list functions: {:?}", e),
    };

    assert_eq!(functions.len(), 2);

    assert_eq!(functions[0].function, "calculate");
    assert!(functions[0].body.is_some());

    assert_eq!(functions[1].function, "check");
    assert!(functions[1].body.is_some());

    let jsonl = match functions_to_jsonl(&functions) {
        Ok(j) => j,
        Err(e) => panic!("Failed to convert to JSONL: {:?}", e),
    };

    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), 2);

    println!("Function JSONL output:");
    for line in &lines {
        println!("{}", line);
    }

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_nested_functions() {
    let test_file = "test_nested.rs";
    let content = r#"
fn outer() {
    fn inner() {
        println!("nested");
    }

    inner();
}
"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let functions = match list_functions(test_file) {
        Ok(f) => f,
        Err(e) => panic!("Failed to list functions: {:?}", e),
    };

    assert!(functions.len() >= 1);
    assert_eq!(functions[0].function, "outer");

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}
