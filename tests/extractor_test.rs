use ntree::create_tree_from_file;
use std::fs;

#[test]
fn test_extract_identifiers() {
    let test_file = "test_extractor.rs";
    let content = r#"
pub fn my_function() {
    println!("test");
}

pub struct MyStruct {
    field: String,
}

pub enum MyEnum {
    Variant1,
    Variant2,
}
"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let root = match create_tree_from_file(test_file) {
        Ok(r) => r,
        Err(e) => panic!("Failed to create tree: {:?}", e),
    };

    let mut cursor = root.walk();
    let children: Vec<_> = root.named_children(&mut cursor).collect();

    assert!(children.len() >= 3);

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}