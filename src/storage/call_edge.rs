use super::symbol_core::SymbolId;
use serde::{Deserialize, Serialize};

/// Call site resolution confidence levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CallConfidence {
    /// Direct call with single known target
    Direct,
    /// OO/trait call with multiple candidates
    Virtual,
    /// Dynamic call that can't be statically resolved
    Dynamic,
    /// Unresolved call site
    Unknown,
}

/// Type of call site.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CallType {
    /// Free function call
    FreeFunction,
    /// Static method call
    StaticMethod,
    /// Instance method call (OO)
    InstanceMethod,
    /// Constructor call
    Constructor,
    /// Dynamic dispatch (Python/JS)
    Dynamic,
}

/// Call edge representing potential function call relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEdge {
    /// Symbol ID of the calling function
    pub caller_sym: SymbolId,
    /// Location of the call site
    pub site_span: String,
    /// Original call expression text
    pub callee_expr_text: String,
    /// Resolved target symbols (empty if unresolved)
    pub targets: Vec<SymbolId>,
    /// Resolution confidence
    pub confidence: CallConfidence,
    /// Type of call
    pub call_type: CallType,
    /// Module hints for dynamic calls
    pub module_hints: Vec<String>,
}

impl CallEdge {
    /// Create a new call edge.
    pub fn new(caller_sym: SymbolId, site_span: String, callee_expr_text: String) -> Self {
        CallEdge {
            caller_sym,
            site_span,
            callee_expr_text,
            targets: Vec::new(),
            confidence: CallConfidence::Unknown,
            call_type: CallType::FreeFunction,
            module_hints: Vec::new(),
        }
    }

    /// Set direct call target.
    pub fn with_direct_target(mut self, target: SymbolId) -> Self {
        self.targets = vec![target];
        self.confidence = CallConfidence::Direct;
        self.call_type = CallType::FreeFunction;
        self
    }

    /// Set virtual call candidates.
    pub fn with_virtual_candidates(mut self, candidates: Vec<SymbolId>) -> Self {
        self.targets = candidates;
        self.confidence = CallConfidence::Virtual;
        self.call_type = CallType::InstanceMethod;
        self
    }

    /// Set dynamic call with module hints.
    pub fn with_dynamic_hints(mut self, hints: Vec<String>) -> Self {
        self.targets.clear();
        self.confidence = CallConfidence::Dynamic;
        self.call_type = CallType::Dynamic;
        self.module_hints = hints;
        self
    }

    /// Check if call has definitive target.
    pub fn has_definitive_target(&self) -> bool {
        matches!(self.confidence, CallConfidence::Direct) && self.targets.len() == 1
    }

    /// Check if call is ambiguous.
    pub fn is_ambiguous(&self) -> bool {
        self.targets.len() > 1
    }
}
