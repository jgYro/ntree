use ntree::{DependencyGraph, ModuleId, ModuleEdge, EdgeKind, Module, ModuleType};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Module Dependency Graph Demo ===");

    // Create a sample dependency graph
    let mut graph = DependencyGraph::new();

    // Add some modules
    let mod_a = Module::new(
        ModuleId::new("rust:project::mod_a".to_string()),
        vec![PathBuf::from("src/mod_a.rs")],
        "rust".to_string(),
        ModuleType::Local,
    );

    let mod_b = Module::new(
        ModuleId::new("rust:project::mod_b".to_string()),
        vec![PathBuf::from("src/mod_b.rs")],
        "rust".to_string(),
        ModuleType::Local,
    );

    let mod_c = Module::new(
        ModuleId::new("rust:project::mod_c".to_string()),
        vec![PathBuf::from("src/mod_c.rs")],
        "rust".to_string(),
        ModuleType::Local,
    );

    let external_crate = Module::new(
        ModuleId::new("rust:serde".to_string()),
        vec![],
        "rust".to_string(),
        ModuleType::External,
    );

    graph.add_module(mod_a.clone());
    graph.add_module(mod_b.clone());
    graph.add_module(mod_c.clone());
    graph.add_module(external_crate.clone());

    // Add dependencies: A -> B, B -> C, A -> serde
    graph.add_edge(ModuleEdge::new(
        mod_a.id.clone(),
        mod_b.id.clone(),
        EdgeKind::Import,
        "1:1-1:20".to_string(),
    ));

    graph.add_edge(ModuleEdge::new(
        mod_b.id.clone(),
        mod_c.id.clone(),
        EdgeKind::Import,
        "2:1-2:20".to_string(),
    ));

    graph.add_edge(ModuleEdge::new(
        mod_a.id.clone(),
        external_crate.id.clone(),
        EdgeKind::Import,
        "3:1-3:15".to_string(),
    ));

    // Analyze dependencies
    let analysis = graph.analyze_dependencies();

    println!("📊 Dependency Analysis Results:");
    println!("  {}", analysis.summary());
    println!("  Cycles detected: {}", analysis.cycles);
    println!("  Can process topologically: {}", analysis.can_process_topologically());

    if let Some(order) = analysis.processing_order() {
        println!("\n🔄 Topological Processing Order:");
        for (i, module) in order.iter().enumerate() {
            println!("  {}. {}", i + 1, module.as_str());
        }
    }

    if !analysis.get_cycles().is_empty() {
        println!("\n⚠️ Cycles Found:");
        for (i, cycle) in analysis.get_cycles().iter().enumerate() {
            println!("  Cycle {}: {:?}", i + 1, cycle.iter().map(|m| m.as_str()).collect::<Vec<_>>());
        }
    }

    // Test with a cyclic graph
    println!("\n=== Testing Cyclic Dependencies ===");
    let mut cyclic_graph = DependencyGraph::new();

    let mod_x = Module::new(ModuleId::new("test::x".to_string()), vec![], "rust".to_string(), ModuleType::Local);
    let mod_y = Module::new(ModuleId::new("test::y".to_string()), vec![], "rust".to_string(), ModuleType::Local);

    cyclic_graph.add_module(mod_x.clone());
    cyclic_graph.add_module(mod_y.clone());

    // Create cycle: X -> Y, Y -> X
    cyclic_graph.add_edge(ModuleEdge::new(mod_x.id.clone(), mod_y.id.clone(), EdgeKind::Import, "1:1".to_string()));
    cyclic_graph.add_edge(ModuleEdge::new(mod_y.id.clone(), mod_x.id.clone(), EdgeKind::Import, "1:1".to_string()));

    let cyclic_analysis = cyclic_graph.analyze_dependencies();
    println!("  {}", cyclic_analysis.summary());

    println!("\n🎯 Module graph system ready for import extraction!");

    Ok(())
}