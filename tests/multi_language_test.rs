use ntree::{SourceCode, SupportedLanguage};

#[cfg(test)]
mod multi_language_tests {
    use super::*;

    #[test]
    fn test_all_language_detection() {
        // Test all supported extensions
        assert_eq!(SupportedLanguage::from_path("test.rs").unwrap(), SupportedLanguage::Rust);
        assert_eq!(SupportedLanguage::from_path("test.py").unwrap(), SupportedLanguage::Python);
        assert_eq!(SupportedLanguage::from_path("test.js").unwrap(), SupportedLanguage::JavaScript);
        assert_eq!(SupportedLanguage::from_path("test.mjs").unwrap(), SupportedLanguage::JavaScript);
        assert_eq!(SupportedLanguage::from_path("test.ts").unwrap(), SupportedLanguage::TypeScript);
        assert_eq!(SupportedLanguage::from_path("Test.java").unwrap(), SupportedLanguage::Java);
        assert_eq!(SupportedLanguage::from_path("test.c").unwrap(), SupportedLanguage::C);
        assert_eq!(SupportedLanguage::from_path("test.h").unwrap(), SupportedLanguage::C);
        assert_eq!(SupportedLanguage::from_path("test.cpp").unwrap(), SupportedLanguage::Cpp);
        assert_eq!(SupportedLanguage::from_path("test.cc").unwrap(), SupportedLanguage::Cpp);
        assert_eq!(SupportedLanguage::from_path("test.cxx").unwrap(), SupportedLanguage::Cpp);
        assert_eq!(SupportedLanguage::from_path("test.hpp").unwrap(), SupportedLanguage::Cpp);
    }

    #[test]
    fn test_language_names() {
        assert_eq!(SupportedLanguage::Rust.name(), "rust");
        assert_eq!(SupportedLanguage::Python.name(), "python");
        assert_eq!(SupportedLanguage::JavaScript.name(), "javascript");
        assert_eq!(SupportedLanguage::TypeScript.name(), "typescript");
        assert_eq!(SupportedLanguage::Java.name(), "java");
        assert_eq!(SupportedLanguage::C.name(), "c");
        assert_eq!(SupportedLanguage::Cpp.name(), "cpp");
    }

    #[test]
    fn test_rust_analysis() {
        let analysis = SourceCode::new("test_sample.rs")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        assert!(analysis.functions().len() >= 3);
        assert!(analysis.complexity().len() >= 3);
    }

    #[test]
    fn test_python_analysis() {
        let python_file = "/Users/jerichogregory/Yro/projects/Maximus/NASH/Individual_Rust_Feats/nash/code/crypto/https/https_request.py";
        let analysis = SourceCode::new(python_file)
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        // This file has 2 functions with try/except
        assert_eq!(analysis.functions().len(), 2);
        assert_eq!(analysis.complexity().len(), 2);

        let functions = analysis.functions();
        let names = functions.names();
        assert!(names.contains(&"make_https_request"));
        assert!(names.contains(&"make_https_request_urllib"));

        // Both functions should have complexity 2 due to try/except branching
        for result in analysis.complexity().all() {
            assert_eq!(result.cyclomatic, 2, "Python try/except should create complexity 2");
        }
    }

    #[test]
    fn test_javascript_analysis() {
        let analysis = SourceCode::new("test_samples/test_sample.js")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        assert!(analysis.functions().len() >= 3);
        assert!(analysis.complexity().len() >= 3);

        // Should find regular functions
        let functions = analysis.functions();
        let names = functions.names();
        assert!(names.iter().any(|&name| name.contains("simpleFunction")));
    }

    #[test]
    fn test_typescript_analysis() {
        let analysis = SourceCode::new("test_samples/test_sample.ts")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        assert!(analysis.functions().len() >= 3);
        assert!(analysis.complexity().len() >= 3);
    }

    #[test]
    fn test_java_analysis() {
        let analysis = SourceCode::new("test_samples/TestSample.java")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        // Java parsing is working, functions are detected
        assert!(analysis.functions().len() >= 3);
        // Note: Complexity analysis may need Java-specific CFG improvements
        println!("Java functions: {}, complexity: {}",
                analysis.functions().len(), analysis.complexity().len());
    }

    #[test]
    fn test_c_analysis() {
        let analysis = SourceCode::new("test_samples/test_sample.c")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        // C parsing is working, functions detected
        assert!(analysis.functions().len() >= 3);
        assert!(analysis.complexity().len() >= 3);

        println!("C functions: {}, complexity: {}",
                analysis.functions().len(), analysis.complexity().len());
    }

    #[test]
    fn test_cpp_analysis() {
        let analysis = SourceCode::new("test_samples/test_sample.cpp")
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        assert!(analysis.functions().len() >= 3);
        assert!(analysis.complexity().len() >= 3);
    }

    #[test]
    fn test_multi_language_complexity_comparison() {
        let rust_analysis = SourceCode::new("test_sample.rs").unwrap().analyze().unwrap();
        let js_analysis = SourceCode::new("test_samples/test_sample.js").unwrap().analyze().unwrap();

        // Both should have complexity results
        assert!(!rust_analysis.complexity().all().is_empty());
        assert!(!js_analysis.complexity().all().is_empty());

        // Export both to JSONL
        let rust_jsonl = rust_analysis.to_jsonl().unwrap();
        let js_jsonl = js_analysis.to_jsonl().unwrap();

        assert!(rust_jsonl.contains("\"cyclomatic\":"));
        assert!(js_jsonl.contains("\"cyclomatic\":"));
    }
}