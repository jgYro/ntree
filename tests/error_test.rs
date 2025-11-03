use ntree::NTreeError;
use std::io;

#[test]
fn test_error_display() {
    let err = NTreeError::ParseError("test error".to_string());
    assert_eq!(format!("{}", err), "Parse Error: test error");
}

#[test]
fn test_io_error_conversion() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let tree_err: NTreeError = io_err.into();
    match tree_err {
        NTreeError::IoError(_) => {}
        _ => panic!("Expected IoError variant"),
    }
}