use ntree::api::AnalysisOptions;
use ntree::SourceCode;
use std::path::Path;

#[cfg(test)]
mod source_code_tests {
    use super::*;

    fn get_test_file() -> &'static str {
        "test_sample.rs"
    }

    #[test]
    fn test_source_code_new() {
        let source_code = SourceCode::new(get_test_file());
        assert!(source_code.is_ok());

        let sc = source_code.unwrap();
        assert_eq!(sc.path(), Path::new(get_test_file()));
        assert_eq!(sc.options(), &AnalysisOptions::default());
    }

    #[test]
    fn test_source_code_nonexistent_file() {
        let result = SourceCode::new("nonexistent_file.rs");
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_pattern_complexity() {
        let source_code = SourceCode::new(get_test_file())
            .expect("Valid file")
            .with_complexity_analysis(false);

        assert!(!source_code.options().complexity_analysis);
        assert!(source_code.options().cfg_generation); // Still enabled
    }

    #[test]
    fn test_builder_pattern_cfg() {
        let source_code = SourceCode::new(get_test_file())
            .expect("Valid file")
            .with_cfg_generation(false);

        assert!(!source_code.options().cfg_generation);
        assert!(source_code.options().complexity_analysis); // Still enabled
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let source_code = SourceCode::new(get_test_file())
            .expect("Valid file")
            .with_complexity_analysis(false)
            .with_cfg_generation(false)
            .with_early_exit_analysis(true);

        let options = source_code.options();
        assert!(!options.complexity_analysis);
        assert!(!options.cfg_generation);
        assert!(options.early_exit_analysis);
    }

    #[test]
    fn test_minimal_configuration() {
        let source_code = SourceCode::new(get_test_file())
            .expect("Valid file")
            .minimal();

        let options = source_code.options();
        assert!(options.complexity_analysis);
        assert!(options.cfg_generation);
        assert!(!options.early_exit_analysis);
        assert!(!options.loop_analysis);
        assert!(!options.basic_blocks);
    }

    #[test]
    fn test_none_configuration() {
        let source_code = SourceCode::new(get_test_file()).expect("Valid file").none();

        let options = source_code.options();
        assert!(!options.complexity_analysis);
        assert!(!options.cfg_generation);
        assert!(!options.early_exit_analysis);
        assert!(!options.loop_analysis);
        assert!(!options.basic_blocks);
    }

    #[test]
    fn test_analyze_with_no_options_fails() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .none()
            .analyze();

        assert!(result.is_err());
    }

    #[test]
    fn test_analyze_with_minimal_options_succeeds() {
        let result = SourceCode::new(get_test_file())
            .expect("Valid file")
            .minimal()
            .analyze();

        assert!(result.is_ok());
    }
}
