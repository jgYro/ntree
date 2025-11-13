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

    /// Creates a configuration for Python language.
    pub fn python() -> Self {
        LanguageConfig {
            language: tree_sitter_python::LANGUAGE.into(),
            function_node_type: "function_definition",
            body_node_type: "block",
            identifier_types: vec!["identifier"],
        }
    }

    /// Creates a configuration for JavaScript language.
    pub fn javascript() -> Self {
        LanguageConfig {
            language: tree_sitter_javascript::LANGUAGE.into(),
            function_node_type: "function_declaration",
            body_node_type: "statement_block",
            identifier_types: vec!["identifier", "property_identifier"],
        }
    }

    /// Creates a configuration for TypeScript language.
    pub fn typescript() -> Self {
        LanguageConfig {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            function_node_type: "function_declaration",
            body_node_type: "statement_block",
            identifier_types: vec!["identifier", "property_identifier", "type_identifier"],
        }
    }

    /// Creates a configuration for Java language.
    pub fn java() -> Self {
        LanguageConfig {
            language: tree_sitter_java::LANGUAGE.into(),
            function_node_type: "method_declaration",
            body_node_type: "block",
            identifier_types: vec!["identifier"],
        }
    }

    /// Creates a configuration for C language.
    pub fn c() -> Self {
        LanguageConfig {
            language: tree_sitter_c::LANGUAGE.into(),
            function_node_type: "function_definition",
            body_node_type: "compound_statement",
            identifier_types: vec!["identifier"],
        }
    }

    /// Creates a configuration for C++ language.
    pub fn cpp() -> Self {
        LanguageConfig {
            language: tree_sitter_cpp::LANGUAGE.into(),
            function_node_type: "function_definition",
            body_node_type: "compound_statement",
            identifier_types: vec!["identifier"],
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
