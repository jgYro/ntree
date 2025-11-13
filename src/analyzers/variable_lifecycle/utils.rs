/// Utility functions for variable lifecycle analysis.
pub struct LifecycleUtils;

impl LifecycleUtils {
    /// Infer variable type from statement context.
    pub fn infer_variable_type(statement: &str) -> Option<String> {
        // Simple type inference based on patterns
        if statement.contains(": ") {
            // Explicit type annotation
            if let Some(start) = statement.find(": ") {
                let after_colon = &statement[start + 2..];
                if let Some(end) = after_colon.find([' ', '=', ';', ',', ')']) {
                    return Some(after_colon[..end].trim().to_string());
                }
            }
        }

        // Infer from literal values
        if statement.contains("= \"") {
            return Some("String".to_string());
        }
        if statement.contains("= '") {
            return Some("char".to_string());
        }
        if statement.contains("= true") || statement.contains("= false") {
            return Some("bool".to_string());
        }

        // Check for integer literals
        let words: Vec<&str> = statement.split_whitespace().collect();
        for word in words {
            if word.parse::<i32>().is_ok() {
                return Some("i32".to_string());
            }
            if word.parse::<f64>().is_ok() {
                return Some("f64".to_string());
            }
        }

        None
    }

    /// Extract line number from span string.
    pub fn extract_line_number(span: &str) -> u32 {
        span.split(':')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    /// Extract column number from span string.
    pub fn extract_column_number(span: &str) -> u32 {
        span.split(':')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}