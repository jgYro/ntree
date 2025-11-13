use ntree::analyzers::ComplexityAnalyzer;
use ntree::{CFGEdgeIR, CFGNodeIR, FunctionCFGIR};

#[cfg(test)]
mod complexity_tests {
    use super::*;

    fn create_test_node(func: &str, id: &str, label: &str) -> CFGNodeIR {
        CFGNodeIR::new(
            func.to_string(),
            id.to_string(),
            label.to_string(),
            "test_span".to_string(),
        )
    }

    fn create_test_edge(func: &str, from: &str, to: &str, kind: &str) -> CFGEdgeIR {
        CFGEdgeIR::new(
            func.to_string(),
            from.to_string(),
            to.to_string(),
            kind.to_string(),
        )
    }

    #[test]
    fn test_empty_cfg_complexity() {
        let analyzer = ComplexityAnalyzer::new();
        let cfg = FunctionCFGIR::new("empty_func".to_string(), None);

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "empty_func");
        assert_eq!(result.cyclomatic, 1);
        assert_eq!(result.unreachable, Vec::<String>::new());
    }

    #[test]
    fn test_linear_cfg_complexity() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("linear_func".to_string(), None);

        // Linear sequence: N1 -> N2 -> N3
        cfg.add_node(create_test_node("linear_func", "N1", "ENTRY"));
        cfg.add_node(create_test_node("linear_func", "N2", "statement"));
        cfg.add_node(create_test_node("linear_func", "N3", "EXIT"));

        cfg.add_edge(create_test_edge("linear_func", "N1", "N2", "next"));
        cfg.add_edge(create_test_edge("linear_func", "N2", "N3", "next"));

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "linear_func");
        // E=2, N=3, so complexity = 2 - 3 + 2 = 1
        assert_eq!(result.cyclomatic, 1);
        assert_eq!(result.unreachable, Vec::<String>::new());
    }

    #[test]
    fn test_if_else_complexity() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("if_func".to_string(), None);

        // If-else: N1 -> N2, N1 -> N3, N2 -> N4, N3 -> N4
        cfg.add_node(create_test_node("if_func", "N1", "ENTRY"));
        cfg.add_node(create_test_node("if_func", "N2", "if_true"));
        cfg.add_node(create_test_node("if_func", "N3", "if_false"));
        cfg.add_node(create_test_node("if_func", "N4", "EXIT"));

        cfg.add_edge(create_test_edge("if_func", "N1", "N2", "true"));
        cfg.add_edge(create_test_edge("if_func", "N1", "N3", "false"));
        cfg.add_edge(create_test_edge("if_func", "N2", "N4", "next"));
        cfg.add_edge(create_test_edge("if_func", "N3", "N4", "next"));

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "if_func");
        // E=4, N=4, so complexity = 4 - 4 + 2 = 2
        assert_eq!(result.cyclomatic, 2);
        assert_eq!(result.unreachable, Vec::<String>::new());
    }

    #[test]
    fn test_unreachable_blocks() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("unreachable_func".to_string(), None);

        // Reachable path: N1 -> N2 -> N4
        // Unreachable blocks: N3, N5
        cfg.add_node(create_test_node("unreachable_func", "N1", "ENTRY"));
        cfg.add_node(create_test_node("unreachable_func", "N2", "reachable"));
        cfg.add_node(create_test_node("unreachable_func", "N3", "unreachable1"));
        cfg.add_node(create_test_node("unreachable_func", "N4", "EXIT"));
        cfg.add_node(create_test_node("unreachable_func", "N5", "unreachable2"));

        cfg.add_edge(create_test_edge("unreachable_func", "N1", "N2", "next"));
        cfg.add_edge(create_test_edge("unreachable_func", "N2", "N4", "next"));
        // N3 and N5 have no incoming edges from reachable nodes

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "unreachable_func");
        // E=2, N=5, so complexity = 2 - 5 + 2 = -1, but minimum is 1
        assert_eq!(result.cyclomatic, 1);

        let mut expected_unreachable = vec!["N3".to_string(), "N5".to_string()];
        expected_unreachable.sort();
        assert_eq!(result.unreachable, expected_unreachable);
    }

    #[test]
    fn test_if_false_unreachable() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("check".to_string(), None);

        // Simulates: if (false) { unreachable_code(); }
        cfg.add_node(create_test_node("check", "N1", "ENTRY"));
        cfg.add_node(create_test_node("check", "N2", "condition"));
        cfg.add_node(create_test_node("check", "N3", "true_branch"));
        cfg.add_node(create_test_node("check", "N4", "false_branch"));
        cfg.add_node(create_test_node("check", "N5", "EXIT"));

        // Only connect the false branch (simulating if(false))
        cfg.add_edge(create_test_edge("check", "N1", "N2", "next"));
        cfg.add_edge(create_test_edge("check", "N2", "N4", "false"));
        cfg.add_edge(create_test_edge("check", "N4", "N5", "next"));
        // N3 is not connected, making it unreachable

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "check");
        // E=3, N=5, so complexity = 3 - 5 + 2 = 0, but minimum is 1
        assert_eq!(result.cyclomatic, 1);
        assert_eq!(result.unreachable, vec!["N3"]);
    }

    #[test]
    fn test_complex_cfg_with_loop() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("loop_func".to_string(), None);

        // Loop with exit: N1 -> N2 -> N3 -> N2 (back edge), N2 -> N4 (exit)
        cfg.add_node(create_test_node("loop_func", "N1", "ENTRY"));
        cfg.add_node(create_test_node("loop_func", "N2", "loop_condition"));
        cfg.add_node(create_test_node("loop_func", "N3", "loop_body"));
        cfg.add_node(create_test_node("loop_func", "N4", "EXIT"));

        cfg.add_edge(create_test_edge("loop_func", "N1", "N2", "next"));
        cfg.add_edge(create_test_edge("loop_func", "N2", "N3", "true"));
        cfg.add_edge(create_test_edge("loop_func", "N3", "N2", "next"));
        cfg.add_edge(create_test_edge("loop_func", "N2", "N4", "false"));

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "loop_func");
        // E=4, N=4, so complexity = 4 - 4 + 2 = 2
        assert_eq!(result.cyclomatic, 2);
        assert_eq!(result.unreachable, Vec::<String>::new());
    }

    #[test]
    fn test_no_entry_node() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("no_entry".to_string(), None);

        cfg.add_node(create_test_node("no_entry", "N1", "first"));
        cfg.add_node(create_test_node("no_entry", "N2", "second"));

        cfg.add_edge(create_test_edge("no_entry", "N1", "N2", "next"));

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        assert_eq!(result.function, "no_entry");
        // Should use first node as entry point
        assert_eq!(result.cyclomatic, 1);
        assert_eq!(result.unreachable, Vec::<String>::new());
    }

    #[test]
    fn test_cfg13_example_output() {
        let analyzer = ComplexityAnalyzer::new();
        let mut cfg = FunctionCFGIR::new("check".to_string(), None);

        // Create a CFG that results in complexity 3 with N7 and N9 unreachable
        cfg.add_node(create_test_node("check", "N1", "ENTRY"));
        cfg.add_node(create_test_node("check", "N2", "if_condition"));
        cfg.add_node(create_test_node("check", "N3", "then_branch"));
        cfg.add_node(create_test_node("check", "N4", "else_branch"));
        cfg.add_node(create_test_node("check", "N5", "merge"));
        cfg.add_node(create_test_node("check", "N6", "another_if"));
        cfg.add_node(create_test_node("check", "N7", "unreachable_branch"));
        cfg.add_node(create_test_node("check", "N8", "final"));
        cfg.add_node(create_test_node("check", "N9", "dead_code"));

        // Build a graph with complexity 3 (E=10, N=9, so 10-9+2=3)
        cfg.add_edge(create_test_edge("check", "N1", "N2", "next"));
        cfg.add_edge(create_test_edge("check", "N2", "N3", "true"));
        cfg.add_edge(create_test_edge("check", "N2", "N4", "false"));
        cfg.add_edge(create_test_edge("check", "N3", "N5", "next"));
        cfg.add_edge(create_test_edge("check", "N4", "N5", "next"));
        cfg.add_edge(create_test_edge("check", "N5", "N6", "next"));
        cfg.add_edge(create_test_edge("check", "N6", "N3", "loop_back")); // Add back edge for complexity
        cfg.add_edge(create_test_edge("check", "N6", "N8", "exit"));
        cfg.add_edge(create_test_edge("check", "N5", "N8", "alternate")); // Add another edge for more complexity
        cfg.add_edge(create_test_edge("check", "N3", "N8", "shortcut")); // One more edge to get complexity 3
                                                                         // N7 and N9 have no incoming edges, making them unreachable

        let result = analyzer.analyze(&cfg).expect("Analysis should succeed");

        // Expected output: {"function":"check","cyclomatic":3,"unreachable":["N7","N9"]}
        assert_eq!(result.function, "check");
        assert_eq!(result.cyclomatic, 3);
        assert_eq!(result.unreachable, vec!["N7", "N9"]);

        // Test JSON serialization
        let json = serde_json::to_string(&result).expect("Should serialize to JSON");
        assert!(json.contains("\"function\":\"check\""));
        assert!(json.contains("\"cyclomatic\":3"));
        assert!(json.contains("\"unreachable\":[\"N7\",\"N9\"]"));
    }
}
