use ntree::{SourceCode, SupportedLanguage};

#[cfg(test)]
mod cross_platform_tests {
    use super::*;

    #[test]
    fn test_language_detection_rust() {
        let lang = SupportedLanguage::from_path("test.rs").expect("Should detect Rust");
        assert_eq!(lang, SupportedLanguage::Rust);
        assert_eq!(lang.name(), "rust");
    }

    #[test]
    fn test_language_detection_python() {
        let lang = SupportedLanguage::from_path("test.py").expect("Should detect Python");
        assert_eq!(lang, SupportedLanguage::Python);
        assert_eq!(lang.name(), "python");
    }

    #[test]
    fn test_unsupported_language() {
        let result = SupportedLanguage::from_path("test.go"); // Go is not supported yet
        assert!(result.is_err());
    }

    #[test]
    fn test_rust_file_analysis() {
        let analysis = SourceCode::new("test_sample.rs")
            .expect("Valid Rust file")
            .analyze()
            .expect("Analysis should succeed");

        assert!(!analysis.functions().all().is_empty());
        assert!(!analysis.complexity().all().is_empty());
        assert!(!analysis.cfgs().all().is_empty());
    }

    #[test]
    fn test_python_file_analysis() {
        let python_file = "test_samples/test_python_classes.py";

        let analysis = SourceCode::new(python_file)
            .expect("Valid Python file")
            .analyze()
            .expect("Python analysis should succeed");

        let functions = analysis.functions();
        let complexity = analysis.complexity();

        // Should find Python functions
        assert!(!functions.all().is_empty(), "Should find Python functions");
        assert!(!complexity.all().is_empty(), "Should have complexity results");

        // Check for Python class constructors
        let symbols = analysis.symbols();
        let search = symbols.named("__init__").regex(false);
        let constructors = search.search().expect("Search should work");
        assert!(!constructors.is_empty(), "Should find Python constructors");
    }

    #[test]
    fn test_cross_language_jsonl_output() {
        // Test Rust
        let rust_analysis = SourceCode::new("test_sample.rs")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        let rust_jsonl = rust_analysis.to_jsonl().expect("JSONL should work");
        assert!(!rust_jsonl.is_empty());

        // Test Python
        let python_file = "test_samples/test_python_classes.py";
        let python_analysis = SourceCode::new(python_file)
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        let python_jsonl = python_analysis.to_jsonl().expect("JSONL should work");
        assert!(!python_jsonl.is_empty());

        // Both should contain complexity results
        assert!(rust_jsonl.contains("\"cyclomatic\":"));
        assert!(python_jsonl.contains("\"cyclomatic\":"));
    }

    #[test]
    fn test_invalid_file_path() {
        let result = SourceCode::new("nonexistent.py");
        assert!(result.is_err());
    }
}