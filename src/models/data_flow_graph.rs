use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a data flow graph showing variable dependencies between statements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataFlowGraph {
    /// Function name this graph represents
    pub function_name: String,
    /// Data dependency edges between nodes
    pub edges: Vec<DataDependencyEdge>,
    /// Statement nodes in the graph
    pub nodes: HashMap<String, DataFlowNode>,
    /// Variable definitions reaching each node
    pub reaching_definitions: HashMap<String, Vec<VariableDefinition>>,
}

/// Represents a data dependency edge between two statements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataDependencyEdge {
    /// Source statement/node
    pub from: String,
    /// Target statement/node
    pub to: String,
    /// Variable causing the dependency
    pub variable: String,
    /// Type of data dependency
    pub dependency_type: DependencyType,
    /// Source location span
    pub span: String,
}

/// Types of data dependencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyType {
    /// True dependency: read after write
    TrueDependency,
    /// Anti-dependency: write after read
    AntiDependency,
    /// Output dependency: write after write
    OutputDependency,
    /// Control dependency: execution depends on condition
    ControlDependency,
}

/// Represents a node in the data flow graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataFlowNode {
    /// Node identifier
    pub id: String,
    /// Statement or expression
    pub statement: String,
    /// Variables defined at this node
    pub definitions: Vec<String>,
    /// Variables used at this node
    pub uses: Vec<String>,
    /// Source location span
    pub span: String,
    /// Line number
    pub line: u32,
}

/// Represents a variable definition for reaching definitions analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariableDefinition {
    /// Variable name
    pub variable: String,
    /// Statement where variable is defined
    pub definition_site: String,
    /// Source location span
    pub span: String,
    /// Whether this is an initial definition (parameter, etc.)
    pub is_initial: bool,
}

impl DataFlowGraph {
    /// Create a new data flow graph.
    pub fn new(function_name: String) -> Self {
        DataFlowGraph {
            function_name,
            edges: Vec::new(),
            nodes: HashMap::new(),
            reaching_definitions: HashMap::new(),
        }
    }

    /// Add a data flow node.
    pub fn add_node(&mut self, node: DataFlowNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Add a data dependency edge.
    pub fn add_edge(&mut self, edge: DataDependencyEdge) {
        self.edges.push(edge);
    }

    /// Set reaching definitions for a node.
    pub fn set_reaching_definitions(
        &mut self,
        node_id: String,
        definitions: Vec<VariableDefinition>,
    ) {
        self.reaching_definitions.insert(node_id, definitions);
    }

    /// Get all variables in the graph.
    pub fn get_variables(&self) -> Vec<String> {
        let mut variables = std::collections::HashSet::new();
        for edge in &self.edges {
            variables.insert(edge.variable.clone());
        }
        variables.into_iter().collect()
    }

    /// Get dependencies for a specific variable.
    pub fn get_variable_dependencies(&self, variable: &str) -> Vec<&DataDependencyEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.variable == variable)
            .collect()
    }
}

impl DataFlowNode {
    /// Create a new data flow node.
    pub fn new(id: String, statement: String, span: String, line: u32) -> Self {
        DataFlowNode {
            id,
            statement,
            definitions: Vec::new(),
            uses: Vec::new(),
            span,
            line,
        }
    }

    /// Add a variable definition.
    pub fn add_definition(&mut self, variable: String) {
        self.definitions.push(variable);
    }

    /// Add a variable use.
    pub fn add_use(&mut self, variable: String) {
        self.uses.push(variable);
    }
}
