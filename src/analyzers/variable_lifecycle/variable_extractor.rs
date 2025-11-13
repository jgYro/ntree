use crate::models::VariableEventType;

/// Handles extraction of variable information from code statements.
pub struct VariableExtractor;

impl VariableExtractor {
    /// Extract variable definition from statement.
    pub fn extract_definition(statement: &str) -> Option<(String, VariableEventType)> {
        // Handle let declarations
        if statement.contains("let ") {
            if let Some(var_name) = Self::extract_let_variable(statement) {
                return Some((var_name, VariableEventType::Definition));
            }
        }

        // Handle function parameters (simplified)
        if statement.contains("(") && statement.contains(":") {
            if let Some(param_name) = Self::extract_parameter(statement) {
                return Some((param_name, VariableEventType::Definition));
            }
        }

        None
    }

    /// Extract variable uses from statement.
    pub fn extract_variable_uses(statement: &str) -> Vec<String> {
        let mut uses = Vec::new();

        // Skip if this is a definition
        if statement.contains("let ") {
            // For "let x = y", y is a use
            if statement.contains(" = ") {
                let parts: Vec<&str> = statement.split(" = ").collect();
                if parts.len() > 1 {
                    uses.extend(Self::extract_variables_from_expression(parts[1]));
                }
            }
        } else if statement.contains(" = ") {
            // For "x = y", y is a use
            let parts: Vec<&str> = statement.split(" = ").collect();
            if parts.len() > 1 {
                uses.extend(Self::extract_variables_from_expression(parts[1]));
            }
        } else {
            // Other expressions - all variables are uses
            uses.extend(Self::extract_variables_from_expression(statement));
        }

        uses
    }

    /// Extract variable mutation from statement.
    pub fn extract_mutation(statement: &str) -> Option<String> {
        // Handle reassignments (not let declarations)
        if statement.contains(" = ") && !statement.contains("let ") {
            let parts: Vec<&str> = statement.split(" = ").collect();
            if !parts.is_empty() {
                let lhs = parts[0].trim().replace(";", "");
                if lhs.chars().all(|c| c.is_alphanumeric() || c == '_') && !lhs.is_empty() {
                    return Some(lhs);
                }
            }
        }

        None
    }

    /// Extract variable name from let statement.
    fn extract_let_variable(statement: &str) -> Option<String> {
        if let Some(start) = statement.find("let ") {
            let after_let = &statement[start + 4..];
            if let Some(end) = after_let.find([' ', '=', ';', ':']) {
                let var_name = after_let[..end].trim();
                if var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return Some(var_name.to_string());
                }
            }
        }
        None
    }

    /// Extract parameter name (simplified).
    fn extract_parameter(statement: &str) -> Option<String> {
        // Very basic parameter extraction - would need language-specific implementation
        if statement.contains("(") && statement.contains(":") {
            // Look for patterns like "fn name(param: type)"
            if let Some(start) = statement.find('(') {
                if let Some(end) = statement.find(':') {
                    if end > start {
                        let param_text = &statement[start + 1..end].trim();
                        if param_text.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            return Some(param_text.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract variables from expression.
    fn extract_variables_from_expression(expression: &str) -> Vec<String> {
        let mut variables = Vec::new();

        // Simple heuristic: split on common delimiters and filter for variable-like tokens
        let tokens: Vec<&str> = expression
            .split(&[' ', '(', ')', '[', ']', '{', '}', ',', ';', '=', '+', '-', '*', '/', '%', '!', '&', '|', '^', '<', '>', '.'])
            .filter(|s| !s.is_empty())
            .collect();

        for token in tokens {
            let word = token.trim();
            // Filter out keywords, numbers, and operators
            if !word.is_empty()
                && !Self::is_keyword(word)
                && !word.chars().all(|c| c.is_numeric() || c == '.')
                && word.chars().all(|c| c.is_alphanumeric() || c == '_')
                && word.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_')
            {
                variables.push(word.to_string());
            }
        }

        variables
    }

    /// Check if a word is a Rust keyword.
    fn is_keyword(word: &str) -> bool {
        matches!(
            word,
            "let" | "mut" | "const" | "static" | "fn" | "if" | "else" | "while" | "for" | "loop"
                | "match" | "return" | "break" | "continue" | "true" | "false" | "self" | "Self"
                | "super" | "crate" | "mod" | "pub" | "use" | "as" | "where" | "impl" | "trait"
                | "struct" | "enum" | "type" | "union"
        )
    }
}