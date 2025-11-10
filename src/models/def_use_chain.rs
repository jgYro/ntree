use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents def-use chains linking variable definitions to their uses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefUseChain {
    /// Variable name
    pub variable: String,
    /// Definition site
    pub definition: DefUseSite,
    /// All use sites reachable from this definition
    pub uses: Vec<DefUseSite>,
    /// Function containing this chain
    pub function_name: String,
}

/// Represents a definition or use site for a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefUseSite {
    /// Location in source code
    pub span: String,
    /// Statement containing the def/use
    pub statement: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Type of site
    pub site_type: DefUseSiteType,
    /// Context information
    pub context: String,
}

/// Types of definition/use sites.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DefUseSiteType {
    /// Variable declaration
    Declaration,
    /// Assignment to variable
    Assignment,
    /// Parameter definition
    Parameter,
    /// Variable read/use
    Use,
    /// Variable in expression
    Expression,
    /// Return statement
    Return,
}

/// Collection of def-use chains for analysis results.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DefUseChainSet {
    /// Chains organized by function
    chains: HashMap<String, Vec<DefUseChain>>,
    /// Total number of definitions
    total_definitions: usize,
    /// Total number of uses
    total_uses: usize,
}

impl DefUseChain {
    /// Create a new def-use chain.
    pub fn new(variable: String, definition: DefUseSite, function_name: String) -> Self {
        DefUseChain {
            variable,
            definition,
            uses: Vec::new(),
            function_name,
        }
    }

    /// Add a use site to this chain.
    pub fn add_use(&mut self, use_site: DefUseSite) {
        self.uses.push(use_site);
    }

    /// Check if this definition has any uses.
    pub fn has_uses(&self) -> bool {
        !self.uses.is_empty()
    }

    /// Get number of uses.
    pub fn use_count(&self) -> usize {
        self.uses.len()
    }

    /// Check if this is a dead definition (no uses).
    pub fn is_dead(&self) -> bool {
        self.uses.is_empty()
    }
}

impl DefUseSite {
    /// Create a new definition/use site.
    pub fn new(
        span: String,
        statement: String,
        line: u32,
        column: u32,
        site_type: DefUseSiteType
    ) -> Self {
        DefUseSite {
            span,
            statement,
            line,
            column,
            site_type,
            context: String::new(),
        }
    }

    /// Set context information.
    pub fn with_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }

    /// Check if this is a definition site.
    pub fn is_definition(&self) -> bool {
        matches!(self.site_type,
            DefUseSiteType::Declaration |
            DefUseSiteType::Assignment |
            DefUseSiteType::Parameter)
    }

    /// Check if this is a use site.
    pub fn is_use(&self) -> bool {
        matches!(self.site_type,
            DefUseSiteType::Use |
            DefUseSiteType::Expression |
            DefUseSiteType::Return)
    }
}

impl DefUseChainSet {
    /// Create new empty set.
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
            total_definitions: 0,
            total_uses: 0,
        }
    }

    /// Add a def-use chain.
    pub fn add_chain(&mut self, chain: DefUseChain) {
        self.total_definitions += 1;
        self.total_uses += chain.uses.len();

        self.chains
            .entry(chain.function_name.clone())
            .or_default()
            .push(chain);
    }

    /// Get all chains.
    pub fn all(&self) -> Vec<&DefUseChain> {
        self.chains.values().flat_map(|chains| chains.iter()).collect()
    }

    /// Get chains for a specific function.
    pub fn for_function(&self, function_name: &str) -> Vec<&DefUseChain> {
        self.chains.get(function_name)
            .map(|chains| chains.iter().collect())
            .unwrap_or_default()
    }

    /// Get all dead definitions (definitions with no uses).
    pub fn dead_definitions(&self) -> Vec<&DefUseChain> {
        self.all().into_iter().filter(|chain| chain.is_dead()).collect()
    }
}