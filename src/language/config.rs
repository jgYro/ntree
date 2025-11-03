use tree_sitter::Language;

pub struct LanguageConfig {
    pub language: Language,
    pub function_node_type: &'static str,
    pub body_node_type: &'static str,
    pub identifier_types: Vec<&'static str>,
}

impl LanguageConfig {
    pub fn rust() -> Self {
        LanguageConfig {
            language: tree_sitter_rust::language().into(),
            function_node_type: "function_item",
            body_node_type: "block",
            identifier_types: vec!["identifier", "type_identifier"],
        }
    }

    pub fn get_function_node_type(&self) -> &str {
        self.function_node_type
    }

    pub fn get_body_node_type(&self) -> &str {
        self.body_node_type
    }
}