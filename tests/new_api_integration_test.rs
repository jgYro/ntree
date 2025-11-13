use ntree::SourceCode;

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn get_test_file() -> &'static str {
        "test_sample.rs"
    }

    #[test]
    fn test_full_analysis_workflow() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        // Check that all result sets are accessible
        let complexity = result.complexity();
        let cfgs = result.cfgs();
        let functions = result.functions();
        let _basic_blocks = result.basic_blocks();

        // Basic sanity checks
        assert!(
            !functions.all().is_empty(),
            "Should find functions in test file"
        );
        assert!(
            !complexity.all().is_empty(),
            "Should have complexity results"
        );
        assert!(!cfgs.all().is_empty(), "Should have CFG results");
    }

    #[test]
    fn test_selective_analysis() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .with_complexity_analysis(true)
            .with_cfg_generation(false)
            .with_basic_blocks(false)
            .analyze()
            .expect("Analysis should succeed");

        let complexity = result.complexity();
        let cfgs = result.cfgs();
        let basic_blocks = result.basic_blocks();

        assert!(
            !complexity.all().is_empty(),
            "Should have complexity results"
        );
        assert!(cfgs.all().is_empty(), "Should not have CFG results");
        assert!(
            basic_blocks.all().is_empty(),
            "Should not have basic block results"
        );
    }

    #[test]
    fn test_result_filtering() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .analyze()
            .expect("Analysis should succeed");

        let functions = result.functions();

        // Test function filtering
        let all_functions = functions.all();
        let filtered = functions.filter_by_name("new");

        assert!(!all_functions.is_empty());
        // May or may not have functions with "new" in the name, just check it doesn't crash
        assert!(filtered.len() <= all_functions.len());
    }

    #[test]
    fn test_jsonl_export() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .minimal()
            .analyze()
            .expect("Analysis should succeed");

        let jsonl_output = result.to_jsonl().expect("JSONL export should work");
        assert!(!jsonl_output.is_empty(), "JSONL output should not be empty");

        // Should contain JSON data
        assert!(jsonl_output.contains("{"), "Should contain JSON objects");
    }

    #[test]
    fn test_complexity_filtering() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .with_complexity_analysis(true)
            .with_cfg_generation(false)
            .with_basic_blocks(false)
            .analyze()
            .expect("Analysis should succeed");

        let complexity = result.complexity();
        let all_complexity = complexity.all();

        // Test complexity filtering with fresh reference
        let high_complexity = result.complexity().filter_by_complexity(5);

        assert!(high_complexity.len() <= all_complexity.len());

        // Verify JSONL export works
        let jsonl = result.complexity().to_jsonl().expect("JSONL should work");
        assert!(!jsonl.is_empty());
    }
}
