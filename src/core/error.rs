use std::fmt;
use std::io;

/// Error types for ntree operations.
#[derive(Debug)]
pub enum NTreeError {
    /// IO-related errors (file reading, permissions, etc.)
    IoError(io::Error),
    /// Parsing-related errors (invalid syntax, language setup, etc.)
    ParseError(String),
    /// Invalid input or configuration
    InvalidInput(String),
}

impl fmt::Display for NTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NTreeError::IoError(e) => write!(f, "IO Error: {}", e),
            NTreeError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            NTreeError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl std::error::Error for NTreeError {}

impl From<io::Error> for NTreeError {
    fn from(error: io::Error) -> Self {
        NTreeError::IoError(error)
    }
}
