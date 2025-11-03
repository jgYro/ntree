use ntree::reader::read_file;
use std::fs;

#[test]
fn test_read_file() {
    let test_file = "test_reader.txt";
    let content = "test content";

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let result = match read_file(test_file) {
        Ok(data) => data,
        Err(e) => panic!("Failed to read file: {}", e),
    };
    assert_eq!(result, content);

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_read_nonexistent_file() {
    let result = read_file("nonexistent_file.txt");
    assert!(result.is_err());
}