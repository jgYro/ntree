use super::module_normalizer::ModuleNormalizer;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Normalized module identifier across all languages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ModuleId(String);

impl ModuleId {
    /// Create normalized module identifier.
    pub fn new(id: String) -> Self {
        ModuleId(id)
    }

    /// Create from language-specific module path.
    pub fn from_language_path(path: &str, language: &str) -> Self {
        ModuleNormalizer::normalize(path, language)
    }

    /// Get the identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Module representation with path roots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: ModuleId,
    pub path_roots: Vec<PathBuf>,
    pub language: String,
    pub module_type: ModuleType,
}

/// Types of modules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModuleType {
    /// Local file/directory module
    Local,
    /// External package/crate
    External,
    /// Standard library
    Standard,
    /// System header (C/C++)
    System,
}

/// Directed edge between modules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEdge {
    pub from: ModuleId,
    pub to: ModuleId,
    pub kind: EdgeKind,
    pub span: String,
}

/// Types of module dependencies.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EdgeKind {
    /// Import statement
    Import,
    /// Include statement (C/C++)
    Include,
    /// Dynamic import
    Dynamic,
}

impl Module {
    /// Create a new module.
    pub fn new(
        id: ModuleId,
        path_roots: Vec<PathBuf>,
        language: String,
        module_type: ModuleType,
    ) -> Self {
        Module {
            id,
            path_roots,
            language,
            module_type,
        }
    }
}

impl ModuleEdge {
    /// Create a new module edge.
    pub fn new(from: ModuleId, to: ModuleId, kind: EdgeKind, span: String) -> Self {
        ModuleEdge {
            from,
            to,
            kind,
            span,
        }
    }
}
