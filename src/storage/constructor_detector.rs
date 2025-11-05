use regex::Regex;
use crate::core::NTreeError;
use super::symbol_core::TopLevelSymbol;
use super::symbol_store::SymbolStore;

/// Language-agnostic constructor detection patterns.
pub struct ConstructorDetector;

impl ConstructorDetector {
    /// Find constructor functions across all languages.
    pub fn find_constructors(store: &SymbolStore) -> Result<Vec<&TopLevelSymbol>, NTreeError> {
        let mut constructors = Vec::new();

        // Rust constructors: new, default, from_*, with_*, etc.
        let rust_patterns = [
            "^new$",
            "^default$",
            "^from_.*",
            "^with_.*",
        ];

        // Python constructors
        let python_patterns = [
            "^__init__$",
            "^__new__$",
        ];

        // JavaScript/TypeScript constructors
        let js_patterns = [
            "^constructor$",
        ];

        // Java/C++ constructors (class name = method name, harder to detect without context)
        // For now, we'll rely on kind detection rather than name patterns

        let all_patterns = [rust_patterns.as_ref(), python_patterns.as_ref(), js_patterns.as_ref()].concat();

        for pattern in all_patterns {
            match Self::find_by_regex(store, pattern) {
                Ok(matches) => constructors.extend(matches),
                Err(_) => continue, // Skip invalid patterns
            }
        }

        // Deduplicate by symbol ID
        constructors.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
        constructors.dedup_by(|a, b| a.id == b.id);

        Ok(constructors)
    }


    /// Helper to search by regex pattern.
    fn find_by_regex<'a>(store: &'a SymbolStore, pattern: &str) -> Result<Vec<&'a TopLevelSymbol>, NTreeError> {
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => return Err(NTreeError::ParseError(format!("Invalid regex: {}", e))),
        };

        let matches = store
            .get_all_symbols()
            .filter(|symbol| symbol.kind == "function" && regex.is_match(&symbol.name))
            .collect();

        Ok(matches)
    }

    /// Find symbols by custom regex pattern (language-agnostic).
    pub fn find_by_pattern<'a>(store: &'a SymbolStore, pattern: &str) -> Result<Vec<&'a TopLevelSymbol>, NTreeError> {
        Self::find_by_regex(store, pattern)
    }

}