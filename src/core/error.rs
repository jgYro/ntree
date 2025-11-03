use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NTreeError {
    IoError(io::Error),
    ParseError(String),
}

impl fmt::Display for NTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NTreeError::IoError(e) => write!(f, "IO Error: {}", e),
            NTreeError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

impl std::error::Error for NTreeError {}

impl From<io::Error> for NTreeError {
    fn from(error: io::Error) -> Self {
        NTreeError::IoError(error)
    }
}