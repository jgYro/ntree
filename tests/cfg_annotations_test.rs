use ntree::models::ir::{CFGEdgeIR, CFGNodeIR, FunctionCFGIR};

#[cfg(test)]
mod annotation_tests {
    use super::*;

    #[test]
    fn test_cfg_edge_default_annotations() {
        let edge = CFGEdgeIR::new(
            "test_func".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            "next".to_string(),
        );

        assert_eq!(edge.source, "tree-sitter");
        assert_eq!(edge.confidence, "exact");
        assert_eq!(edge.func, "test_func");
        assert_eq!(edge.from, "N1");
        assert_eq!(edge.to, "N2");
        assert_eq!(edge.kind, "next");
    }

    #[test]
    fn test_cfg_edge_custom_annotations() {
        let edge = CFGEdgeIR::with_provenance(
            "test_func".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            "true".to_string(),
            "compiler".to_string(),
            "inferred".to_string(),
        );

        assert_eq!(edge.source, "compiler");
        assert_eq!(edge.confidence, "inferred");
        assert_eq!(edge.func, "test_func");
        assert_eq!(edge.from, "N1");
        assert_eq!(edge.to, "N2");
        assert_eq!(edge.kind, "true");
    }

    #[test]
    fn test_cfg_node_default_annotations() {
        let node = CFGNodeIR::new(
            "test_func".to_string(),
            "N1".to_string(),
            "ENTRY".to_string(),
            "1:1-1:1".to_string(),
        );

        assert_eq!(node.source, "tree-sitter");
        assert_eq!(node.confidence, "exact");
        assert_eq!(node.func, "test_func");
        assert_eq!(node.id, "N1");
        assert_eq!(node.label, "ENTRY");
        assert_eq!(node.span, "1:1-1:1");
    }

    #[test]
    fn test_cfg_node_custom_annotations() {
        let node = CFGNodeIR::with_provenance(
            "test_func".to_string(),
            "N1".to_string(),
            "call_site".to_string(),
            "5:10-5:20".to_string(),
            "lsp".to_string(),
            "uncertain".to_string(),
        );

        assert_eq!(node.source, "lsp");
        assert_eq!(node.confidence, "uncertain");
        assert_eq!(node.func, "test_func");
        assert_eq!(node.id, "N1");
        assert_eq!(node.label, "call_site");
        assert_eq!(node.span, "5:10-5:20");
    }

    #[test]
    fn test_cfg_ir_serialization_includes_annotations() {
        let mut cfg = FunctionCFGIR::new("test_func".to_string(), None);

        let node = CFGNodeIR::new(
            "test_func".to_string(),
            "N1".to_string(),
            "ENTRY".to_string(),
            "1:1-1:1".to_string(),
        );

        let edge = CFGEdgeIR::new(
            "test_func".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            "next".to_string(),
        );

        cfg.add_node(node);
        cfg.add_edge(edge);

        let jsonl = cfg.to_jsonl();

        // Check that JSONL contains the annotation fields
        assert!(jsonl.contains("\"source\":\"tree-sitter\""));
        assert!(jsonl.contains("\"confidence\":\"exact\""));
    }

    #[test]
    fn test_cfg_ir_generation_has_annotations() {
        // Test the IR generation directly since that's where annotations are used
        let cfg_ir_results = ntree::api::generate_cfg_ir("test_sample.rs")
            .expect("CFG IR generation should succeed");

        assert!(!cfg_ir_results.is_empty(), "Should have CFG IR results");

        for cfg_ir in cfg_ir_results {
            let jsonl = cfg_ir.to_jsonl();

            // All CFG IR records should include source and confidence fields
            let cfg_lines: Vec<&str> = jsonl
                .lines()
                .filter(|line| line.contains("\"type\":\"CFGNode\"") || line.contains("\"type\":\"CFGEdge\""))
                .collect();

            assert!(!cfg_lines.is_empty(), "Should have CFG IR records for function {}", cfg_ir.function_name);

            for line in cfg_lines {
                assert!(line.contains("\"source\":"), "Line should contain source field: {}", line);
                assert!(line.contains("\"confidence\":"), "Line should contain confidence field: {}", line);
                assert!(line.contains("\"source\":\"tree-sitter\""), "Source should be tree-sitter: {}", line);
                assert!(line.contains("\"confidence\":\"exact\""), "Confidence should be exact: {}", line);
            }
        }
    }

    #[test]
    fn test_different_confidence_levels() {
        let exact_edge = CFGEdgeIR::with_provenance(
            "func".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            "next".to_string(),
            "tree-sitter".to_string(),
            "exact".to_string(),
        );

        let inferred_edge = CFGEdgeIR::with_provenance(
            "func".to_string(),
            "N2".to_string(),
            "N3".to_string(),
            "call".to_string(),
            "compiler".to_string(),
            "inferred".to_string(),
        );

        let uncertain_edge = CFGEdgeIR::with_provenance(
            "func".to_string(),
            "N3".to_string(),
            "N4".to_string(),
            "exception".to_string(),
            "lsp".to_string(),
            "uncertain".to_string(),
        );

        assert_eq!(exact_edge.confidence, "exact");
        assert_eq!(inferred_edge.confidence, "inferred");
        assert_eq!(uncertain_edge.confidence, "uncertain");

        assert_eq!(exact_edge.source, "tree-sitter");
        assert_eq!(inferred_edge.source, "compiler");
        assert_eq!(uncertain_edge.source, "lsp");
    }

    #[test]
    fn test_cfg14_exact_output_format() {
        // Test the exact CFG-14 output format specified in the prompt
        let edge = CFGEdgeIR::new(
            "test_func".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            "true".to_string(),
        );

        let json = serde_json::to_string(&edge).expect("Should serialize to JSON");

        // Should match: {"type":"CFGEdge","from":"N1","to":"N2","kind":"true","source":"tree-sitter","confidence":"exact"}
        assert!(json.contains("\"type\":\"CFGEdge\""));
        assert!(json.contains("\"from\":\"N1\""));
        assert!(json.contains("\"to\":\"N2\""));
        assert!(json.contains("\"kind\":\"true\""));
        assert!(json.contains("\"source\":\"tree-sitter\""));
        assert!(json.contains("\"confidence\":\"exact\""));

        // Verify exact structure
        let expected_fields = ["type", "func", "from", "to", "kind", "source", "confidence"];
        for field in expected_fields {
            assert!(json.contains(&format!("\"{}\":", field)),
                "JSON should contain field {}: {}", field, json);
        }
    }
}