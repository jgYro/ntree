#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CfgNode, CfgEdge, ControlFlowGraph};

    fn create_test_symbol(id: u32, name: &str) -> SymbolId {
        SymbolId::from_string(format!("test_{}_{}", id, name))
    }

    fn create_simple_cfg(entry_id: usize, exit_id: usize) -> ControlFlowGraph {
        let entry = CfgNode::new(entry_id, format!("entry_{}", entry_id));
        let exit = CfgNode::new(exit_id, format!("return_{}", exit_id));
        let edge = CfgEdge::new(entry_id, exit_id, "next".to_string());

        ControlFlowGraph {
            nodes: vec![entry, exit],
            edges: vec![edge],
        }
    }

    #[test]
    fn test_interprocedural_edge_creation() {
        let caller_sym = create_test_symbol(1, "caller");
        let callee_sym = create_test_symbol(2, "callee");

        let call_edge = InterproceduralEdge::new_call(
            100, 200, 1, caller_sym, callee_sym
        );

        assert_eq!(call_edge.from_node, 100);
        assert_eq!(call_edge.to_node, 200);
        assert_eq!(call_edge.kind, InterproceduralEdgeKind::Call);
        assert_eq!(call_edge.callsite_id, Some(1));
        assert_eq!(call_edge.caller_sym, caller_sym);
        assert_eq!(call_edge.callee_sym, Some(callee_sym));
    }

    #[test]
    fn test_call_site_summary() {
        let caller_sym = create_test_symbol(1, "main");
        let callee_sym = create_test_symbol(2, "helper");

        let summary = CallSiteSummary::new(
            42,
            100,
            caller_sym,
            200,
            vec![210, 220],
            callee_sym,
            105,
        );

        assert_eq!(summary.callsite_id, 42);
        assert_eq!(summary.caller_node, 100);
        assert_eq!(summary.callee_entry_id, 200);
        assert_eq!(summary.callee_exit_ids, vec![210, 220]);
        assert_eq!(summary.continuation_node, 105);
    }

    #[test]
    fn test_entry_point() {
        let main_sym = create_test_symbol(1, "main");
        let entry = EntryPoint::new(
            main_sym,
            "Main function".to_string(),
            100
        );

        assert_eq!(entry.sym_id, main_sym);
        assert_eq!(entry.reason, "Main function");
        assert_eq!(entry.entry_node, 100);
    }

    #[test]
    fn test_reachability_info() {
        let sym = create_test_symbol(1, "function");
        let entry_sym = create_test_symbol(2, "main");

        let mut reachability = ReachabilityInfo::new(sym);
        assert!(!reachability.reachable);
        assert!(reachability.reachable_nodes.is_empty());

        reachability.mark_reachable_from(entry_sym);
        assert!(reachability.reachable);
        assert!(reachability.reached_from.contains(&entry_sym));

        reachability.add_reachable_node(42);
        assert!(reachability.reachable_nodes.contains(&42));
    }

    #[test]
    fn test_function_exit() {
        let sym = create_test_symbol(1, "function");
        let mut exit = FunctionExit::new(sym);

        exit.set_normal_exit(100);
        assert_eq!(exit.normal_exit_node, Some(100));

        exit.add_exceptional_exit(ExceptionExitKind::Panic, 200);
        exit.add_exceptional_exit(ExceptionExitKind::Exception, 300);

        let all_exits = exit.get_all_exits();
        assert!(all_exits.contains(&100)); // normal exit
        assert!(all_exits.contains(&200)); // panic exit
        assert!(all_exits.contains(&300)); // exception exit
        assert_eq!(all_exits.len(), 3);
    }

    #[test]
    fn test_exceptional_edge() {
        let source_sym = create_test_symbol(1, "source");
        let handler_sym = create_test_symbol(2, "handler");

        let edge = ExceptionalEdge::new(
            100, 200, ExceptionExitKind::Exception, source_sym
        ).with_handler(handler_sym);

        assert_eq!(edge.from_node, 100);
        assert_eq!(edge.to_node, 200);
        assert_eq!(edge.kind, ExceptionExitKind::Exception);
        assert_eq!(edge.source_function, source_sym);
        assert_eq!(edge.handled_by, Some(handler_sym));
    }

    #[test]
    fn test_interprocedural_cfg_basic() {
        let mut icfg = InterproceduralCFG::new();
        let sym1 = create_test_symbol(1, "function1");
        let sym2 = create_test_symbol(2, "function2");

        let cfg1 = create_simple_cfg(100, 110);
        let cfg2 = create_simple_cfg(200, 210);

        icfg.add_function_cfg(sym1, cfg1);
        icfg.add_function_cfg(sym2, cfg2);

        assert_eq!(icfg.get_entry_points().len(), 0);
        assert_eq!(icfg.get_interprocedural_edges().len(), 0);
        assert_eq!(icfg.get_call_sites().len(), 0);
    }

    #[test]
    fn test_add_entry_point() {
        let mut icfg = InterproceduralCFG::new();
        let main_sym = create_test_symbol(1, "main");
        let cfg = create_simple_cfg(100, 110);

        icfg.add_function_cfg(main_sym, cfg);

        let result = icfg.add_entry_point(main_sym, "Main function".to_string());
        assert!(result.is_ok());

        let entry_points = icfg.get_entry_points();
        assert_eq!(entry_points.len(), 1);
        assert_eq!(entry_points[0].sym_id, main_sym);
        assert_eq!(entry_points[0].reason, "Main function");
    }

    #[test]
    fn test_compute_reachability() -> Result<(), NTreeError> {
        let mut icfg = InterproceduralCFG::new();
        let main_sym = create_test_symbol(1, "main");
        let helper_sym = create_test_symbol(2, "helper");

        // Add CFGs
        let main_cfg = create_simple_cfg(100, 110);
        let helper_cfg = create_simple_cfg(200, 210);

        icfg.add_function_cfg(main_sym, main_cfg);
        icfg.add_function_cfg(helper_sym, helper_cfg);

        // Add entry point
        icfg.add_entry_point(main_sym, "Main function".to_string())?;

        // Compute reachability
        icfg.compute_reachability()?;

        let reachability = icfg.get_reachability();

        // Main should be reachable
        let main_reachability = reachability.get(&main_sym).unwrap();
        assert!(main_reachability.reachable);
        assert!(main_reachability.reached_from.contains(&main_sym));

        // Helper might not be reachable without call edges
        let helper_reachability = reachability.get(&helper_sym).unwrap();
        assert!(!helper_reachability.reachable);

        Ok(())
    }

    #[test]
    fn test_exception_classification() {
        let icfg = InterproceduralCFG::new();

        assert!(icfg.is_exception_node("throw Exception"));
        assert!(icfg.is_exception_node("raise ValueError"));
        assert!(icfg.is_exception_node("panic!"));
        assert!(icfg.is_exception_node("result?"));
        assert!(!icfg.is_exception_node("return value"));

        assert!(icfg.is_normal_exit("return value"));
        assert!(!icfg.is_normal_exit("panic!"));

        assert_eq!(icfg.classify_exception_type("panic!"), ExceptionExitKind::Panic);
        assert_eq!(icfg.classify_exception_type("result?"), ExceptionExitKind::EarlyReturn);
        assert_eq!(icfg.classify_exception_type("throw"), ExceptionExitKind::Exception);
    }

    #[test]
    fn test_function_exits_analysis() -> Result<(), NTreeError> {
        let mut icfg = InterproceduralCFG::new();
        let sym = create_test_symbol(1, "test_function");

        // Create CFG with exception nodes
        let mut cfg = ControlFlowGraph {
            nodes: vec![
                CfgNode::new(100, "function entry".to_string()),
                CfgNode::new(110, "return value".to_string()),
                CfgNode::new(120, "panic! if error".to_string()),
                CfgNode::new(130, "result?".to_string()),
            ],
            edges: vec![
                CfgEdge::new(100, 110, "normal".to_string()),
                CfgEdge::new(100, 120, "error".to_string()),
                CfgEdge::new(100, 130, "early_return".to_string()),
            ],
        };

        icfg.add_function_cfg(sym, cfg);
        icfg.analyze_function_exceptions(sym, icfg.function_cfgs.get(&sym).unwrap())?;

        let function_exits = icfg.get_function_exits();
        let exits = function_exits.get(&sym).unwrap();

        assert_eq!(exits.normal_exit_node, Some(110));
        assert!(exits.exceptional_exit_nodes.contains_key(&ExceptionExitKind::Panic));
        assert!(exits.exceptional_exit_nodes.contains_key(&ExceptionExitKind::EarlyReturn));

        Ok(())
    }
}