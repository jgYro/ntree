use super::symbol_core::SymbolId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Resolution confidence levels for name bindings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResolutionOrigin {
    /// Exact match with full confidence
    Exact,
    /// Best guess based on heuristics
    Heuristic,
    /// Unable to determine resolution
    Unknown,
}

/// Name binding with resolution information and candidates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameBinding {
    /// Location where the name is used
    pub site_span: String,
    /// File containing the name usage
    pub file_path: PathBuf,
    /// The name being resolved
    pub name: String,
    /// Primary resolved symbol ID (if any)
    pub resolved_sym_id: Option<SymbolId>,
    /// Alternative candidates for resolution
    pub candidates: Vec<SymbolId>,
    /// Confidence level of the resolution
    pub confidence: f32,
    /// How the resolution was determined
    pub origin: ResolutionOrigin,
}

impl NameBinding {
    /// Create a new name binding.
    pub fn new(site_span: String, file_path: PathBuf, name: String) -> Self {
        NameBinding {
            site_span,
            file_path,
            name,
            resolved_sym_id: None,
            candidates: Vec::new(),
            confidence: 0.0,
            origin: ResolutionOrigin::Unknown,
        }
    }

    /// Set exact resolution.
    pub fn with_exact_resolution(mut self, symbol_id: SymbolId) -> Self {
        self.resolved_sym_id = Some(symbol_id);
        self.confidence = 1.0;
        self.origin = ResolutionOrigin::Exact;
        self
    }

    /// Set heuristic resolution with candidates.
    pub fn with_heuristic_resolution(
        mut self,
        primary: SymbolId,
        candidates: Vec<SymbolId>,
        confidence: f32,
    ) -> Self {
        self.resolved_sym_id = Some(primary);
        self.candidates = candidates;
        self.confidence = confidence;
        self.origin = ResolutionOrigin::Heuristic;
        self
    }

    /// Mark as unresolved.
    pub fn as_unresolved(mut self) -> Self {
        self.resolved_sym_id = None;
        self.candidates.clear();
        self.confidence = 0.0;
        self.origin = ResolutionOrigin::Unknown;
        self
    }

    /// Check if resolution is confident.
    pub fn is_confident(&self) -> bool {
        self.confidence >= 0.8
    }
    /// Check if there are multiple candidates.
    pub fn has_ambiguity(&self) -> bool {
        self.candidates.len() > 1 || (self.candidates.len() == 1 && self.resolved_sym_id.is_some())
    }
}

/// Resolution result for batch processing.
#[derive(Debug, Clone)]
pub struct ResolutionResult {
    pub resolved: usize,
    pub ambiguous: usize,
    pub unresolved: usize,
    pub total: usize,
}
