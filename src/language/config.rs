use tree_sitter::Language;

/// Configuration for language-specific parsing behavior.
///
/// Encapsulates language-specific details to allow the parser
/// to work with different programming languages.
pub struct LanguageConfig {
    /// Tree-sitter language parser
    pub language: Language,
    /// Node type for function definitions
    pub function_node_type: &'static str,
    /// Node type for function bodies (usually "block")
    pub body_node_type: &'static str,
    /// Valid identifier node types
    pub identifier_types: Vec<&'static str>,
}

impl LanguageConfig {
    /// Creates a configuration for Rust language.
    pub fn rust() -> Self {
        LanguageConfig {
            language: tree_sitter_rust::language().into(),
            function_node_type: "function_item",
            body_node_type: "block",
            identifier_types: vec!["identifier", "type_identifier"],
        }
    }

    /// Returns the node type for function definitions.
    pub fn get_function_node_type(&self) -> &str {
        self.function_node_type
    }

    /// Returns the node type for function bodies.
    pub fn get_body_node_type(&self) -> &str {
        self.body_node_type
    }
}