use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a decision tree mapping conditional branches to outcomes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionTree {
    /// Function name containing this decision tree
    pub function_name: String,
    /// Root decision node
    pub root: DecisionTreeNode,
    /// All paths through the decision tree
    pub paths: Vec<DecisionPath>,
    /// Variables influenced by decisions
    pub influenced_variables: Vec<String>,
}

/// Represents a node in a decision tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionTreeNode {
    /// Node identifier
    pub id: String,
    /// Decision condition (if any)
    pub condition: Option<DecisionCondition>,
    /// Branches from this node
    pub branches: Vec<DecisionBranch>,
    /// Variable state at this node
    pub variable_state: HashMap<String, VariableState>,
    /// Source location span
    pub span: String,
}

/// Represents a decision condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionCondition {
    /// Left operand of condition
    pub left: String,
    /// Operator
    pub operator: ConditionOperator,
    /// Right operand of condition
    pub right: String,
    /// Original condition text
    pub original: String,
    /// Variables involved in condition
    pub variables: Vec<String>,
}

/// Types of condition operators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
}

/// Represents a branch in the decision tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionBranch {
    /// Branch condition (true, false, or pattern)
    pub branch_type: BranchType,
    /// Target node
    pub target: Option<Box<DecisionTreeNode>>,
    /// Actions taken on this branch
    pub actions: Vec<DecisionAction>,
    /// Source location span
    pub span: String,
}

/// Types of decision branches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BranchType {
    /// True branch of if condition
    True,
    /// False branch of if condition
    False,
    /// Pattern match branch
    Pattern(String),
    /// Default/else branch
    Default,
}

/// Represents an action taken based on a decision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionAction {
    /// Type of action
    pub action_type: ActionType,
    /// Variable affected
    pub variable: String,
    /// Value assigned or operation performed
    pub value: String,
    /// Source location span
    pub span: String,
}

/// Types of decision actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    /// Variable assignment
    Assignment,
    /// Function call
    FunctionCall,
    /// Return statement
    Return,
    /// Break/continue
    ControlFlow,
}

/// Represents variable state at a decision point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableState {
    /// Variable name
    pub name: String,
    /// Possible values
    pub values: Vec<String>,
    /// Whether value is known
    pub is_constant: bool,
    /// Constraints on this variable
    pub constraints: Vec<String>,
}

/// Represents a complete path through the decision tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionPath {
    /// Path identifier
    pub id: String,
    /// Conditions that must be true for this path
    pub conditions: Vec<String>,
    /// Final variable states
    pub final_state: HashMap<String, VariableState>,
    /// Actions performed along this path
    pub actions: Vec<DecisionAction>,
    /// Whether this path is reachable
    pub is_reachable: bool,
}

impl DecisionTree {
    /// Create a new decision tree.
    pub fn new(function_name: String, root: DecisionTreeNode) -> Self {
        DecisionTree {
            function_name,
            root,
            paths: Vec::new(),
            influenced_variables: Vec::new(),
        }
    }

    /// Add a complete path through the tree.
    pub fn add_path(&mut self, path: DecisionPath) {
        self.paths.push(path);
    }

    /// Get all reachable paths.
    pub fn reachable_paths(&self) -> Vec<&DecisionPath> {
        self.paths.iter().filter(|path| path.is_reachable).collect()
    }

    /// Get unreachable paths (dead code).
    pub fn unreachable_paths(&self) -> Vec<&DecisionPath> {
        self.paths
            .iter()
            .filter(|path| !path.is_reachable)
            .collect()
    }
}

impl DecisionTreeNode {
    /// Create a new decision node.
    pub fn new(id: String, span: String) -> Self {
        DecisionTreeNode {
            id,
            condition: None,
            branches: Vec::new(),
            variable_state: HashMap::new(),
            span,
        }
    }

    /// Set condition for this node.
    pub fn with_condition(mut self, condition: DecisionCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Add a branch to this node.
    pub fn add_branch(&mut self, branch: DecisionBranch) {
        self.branches.push(branch);
    }
}

/// Collection of decision trees for analysis results.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DecisionTreeSet {
    trees: HashMap<String, DecisionTree>,
}

impl DecisionTreeSet {
    /// Create new empty set.
    pub fn new() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }

    /// Add a decision tree.
    pub fn add_tree(&mut self, tree: DecisionTree) {
        self.trees.insert(tree.function_name.clone(), tree);
    }

    /// Get all decision trees.
    pub fn all(&self) -> Vec<&DecisionTree> {
        self.trees.values().collect()
    }

    /// Get tree for specific function.
    pub fn for_function(&self, function_name: &str) -> Option<&DecisionTree> {
        self.trees.get(function_name)
    }
}
