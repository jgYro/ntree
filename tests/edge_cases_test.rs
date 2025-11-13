use ntree::api::list_functions;
use ntree::items_to_jsonl;
use std::fs;

#[test]
fn test_function_with_attributes() {
    let test_file = "test_attributes.rs";
    let content = r#"
#[derive(Debug)]
#[allow(dead_code)]
fn with_attributes() {
    println!("has attributes");
}

#[test]
fn test_function()
{
    assert_eq!(1, 1);
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

    // First function with attributes
    assert_eq!(functions[0].function, "with_attributes");
    assert!(functions[0].body.is_some());

    // Second function with brace on new line
    assert_eq!(functions[1].function, "test_function");
    assert!(functions[1].body.is_some());

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_function_with_where_clause() {
    let test_file = "test_where.rs";
    let content = r#"
fn complex_function<T, U>(x: T, y: U) -> String
where
    T: std::fmt::Display,
    U: std::fmt::Debug,
{
    format!("{} {:?}", x, y)
}

impl<T> MyStruct<T>
where
    T: Clone,
{
    fn method(&self) -> T
    where
        T: Default,
    {
        T::default()
    }
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

    // Should find both the free function and the method
    assert!(functions.len() >= 1);

    let complex_fn = functions.iter().find(|f| f.function == "complex_function");
    assert!(complex_fn.is_some());
    assert!(complex_fn.unwrap().body.is_some());

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_async_and_const_functions() {
    let test_file = "test_async_const.rs";
    let content = r#"
async fn async_function() -> Result<(), Error> {
    do_async_work().await
}

const fn const_function(n: usize) -> usize {
    n * 2
}

pub unsafe fn unsafe_function() {
    // unsafe operations
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

    assert_eq!(functions.len(), 3);

    assert!(functions.iter().any(|f| f.function == "async_function"));
    assert!(functions.iter().any(|f| f.function == "const_function"));
    assert!(functions.iter().any(|f| f.function == "unsafe_function"));

    // All should have body spans
    for function in &functions {
        assert!(
            function.body.is_some(),
            "Function {} should have body span",
            function.function
        );
    }

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_jsonl_one_line_per_function() {
    let test_file = "test_jsonl.rs";
    let content = r#"
fn first() {}
fn second() {}
fn third() {}
"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let functions = match list_functions(test_file) {
        Ok(f) => f,
        Err(e) => panic!("Failed to list functions: {:?}", e),
    };

    let jsonl = match functions_to_jsonl(&functions) {
        Ok(j) => j,
        Err(e) => panic!("Failed to convert to JSONL: {:?}", e),
    };

    // Verify one line per function
    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(
        lines.len(),
        3,
        "Should have exactly 3 lines for 3 functions"
    );

    // Verify each line is valid JSON
    for line in lines {
        match serde_json::from_str::<serde_json::Value>(line) {
            Ok(_) => {}
            Err(e) => panic!("Invalid JSON line: {} - Error: {}", line, e),
        }
    }

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}
