use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use super::summary::ExternalSummary;

/// Handler for external library analysis and stub generation.
#[derive(Debug)]
pub struct ExternalLibraryHandler {
    /// Known external function summaries
    external_summaries: HashMap<String, ExternalSummary>,
    /// Library dependency paths
    dependency_paths: HashMap<String, PathBuf>,
    /// Standard library summaries by language
    stdlib_summaries: HashMap<String, HashMap<String, ExternalSummary>>,
}

impl ExternalLibraryHandler {
    /// Create new external library handler.
    pub fn new() -> Self {
        let mut handler = ExternalLibraryHandler {
            external_summaries: HashMap::new(),
            dependency_paths: HashMap::new(),
            stdlib_summaries: HashMap::new(),
        };

        handler.load_standard_summaries();
        handler
    }

    /// Load standard library summaries for common functions.
    fn load_standard_summaries(&mut self) {
        self.load_rust_stdlib();
        self.load_python_stdlib();
    }

    /// Load Rust standard library summaries.
    fn load_rust_stdlib(&mut self) {
        use super::summary::{TaintKind, ContractSpec};
        use super::super::incremental::func_summary::EffectKind;

        let mut rust_summaries = HashMap::new();

        // std::println! - I/O sink
        let println_summary = ExternalSummary::new("std::println!".to_string(), "std".to_string())
            .with_effect(EffectKind::IoOperation)
            .with_taint(TaintKind::Sink)
            .with_contract(ContractSpec::new().with_side_effect("Writes to stdout".to_string()));
        rust_summaries.insert("println!".to_string(), println_summary);

        // std::panic! - terminates program
        let panic_summary = ExternalSummary::new("std::panic!".to_string(), "std".to_string())
            .with_effect(EffectKind::External)
            .may_not_terminate()
            .with_contract(ContractSpec::new().with_exception("Always panics".to_string()));
        rust_summaries.insert("panic!".to_string(), panic_summary);

        // Vec::new - pure allocation
        let vec_new_summary = ExternalSummary::new("std::vec::Vec::new".to_string(), "std".to_string())
            .with_effect(EffectKind::Allocation);
        rust_summaries.insert("Vec::new".to_string(), vec_new_summary);

        self.stdlib_summaries.insert("rust".to_string(), rust_summaries);
    }

    /// Load Python standard library summaries.
    fn load_python_stdlib(&mut self) {
        use super::summary::{TaintKind, ContractSpec};
        use super::super::incremental::func_summary::EffectKind;

        let mut python_summaries = HashMap::new();

        // print() - I/O operation
        let print_summary = ExternalSummary::new("builtins.print".to_string(), "builtins".to_string())
            .with_effect(EffectKind::IoOperation)
            .with_taint(TaintKind::Sink);
        python_summaries.insert("print".to_string(), print_summary);

        // input() - I/O source
        let input_summary = ExternalSummary::new("builtins.input".to_string(), "builtins".to_string())
            .with_effect(EffectKind::IoOperation)
            .with_taint(TaintKind::Source);
        python_summaries.insert("input".to_string(), input_summary);

        // eval() - dangerous sink
        let eval_summary = ExternalSummary::new("builtins.eval".to_string(), "builtins".to_string())
            .with_effect(EffectKind::External)
            .with_taint(TaintKind::Sink)
            .may_not_terminate()
            .with_contract(ContractSpec::new()
                .with_precondition("Input should be sanitized".to_string())
                .with_exception("Can raise any exception".to_string()));
        python_summaries.insert("eval".to_string(), eval_summary);

        self.stdlib_summaries.insert("python".to_string(), python_summaries);
    }

    /// Add external library summary.
    pub fn add_summary(&mut self, summary: ExternalSummary) {
        self.external_summaries.insert(summary.qualname.clone(), summary);
    }

    /// Get summary for external function.
    pub fn get_summary(&self, qualname: &str, language: Option<&str>) -> Option<&ExternalSummary> {
        // Check explicit summaries first
        if let Some(summary) = self.external_summaries.get(qualname) {
            return Some(summary);
        }

        // Check standard library summaries
        if let Some(lang) = language {
            if let Some(lang_summaries) = self.stdlib_summaries.get(lang) {
                return lang_summaries.get(qualname);
            }
        }

        None
    }

    /// Add dependency path for source indexing.
    pub fn add_dependency_path(&mut self, library: String, path: PathBuf) {
        self.dependency_paths.insert(library, path);
    }

    /// Check if library has available source.
    pub fn has_source(&self, library: &str) -> bool {
        self.dependency_paths.contains_key(library)
    }

    /// Get dependency path for library.
    pub fn get_dependency_path(&self, library: &str) -> Option<&PathBuf> {
        self.dependency_paths.get(library)
    }

    /// Create stub for unknown external function.
    pub fn create_stub(&self, qualname: String, library: String) -> ExternalSummary {
        use super::super::incremental::func_summary::EffectKind;

        // Conservative assumptions for unknown functions
        ExternalSummary::new(qualname, library)
            .with_effect(EffectKind::External) // Assume side effects
            .not_thread_safe() // Conservative assumption
            .may_not_terminate() // Conservative assumption
    }

    /// Get all external summaries.
    pub fn get_all_summaries(&self) -> impl Iterator<Item = &ExternalSummary> {
        self.external_summaries.values()
    }

    /// Get library statistics.
    pub fn get_stats(&self) -> LibraryStats {
        let total_summaries = self.external_summaries.len();
        let stdlib_summaries: usize = self.stdlib_summaries.values().map(|m| m.len()).sum();
        let dependencies_with_source = self.dependency_paths.len();

        LibraryStats {
            total_external_summaries: total_summaries,
            stdlib_summaries,
            dependencies_with_source,
            total_libraries: self.get_unique_libraries().len(),
        }
    }

    /// Get unique libraries referenced.
    fn get_unique_libraries(&self) -> HashSet<String> {
        let mut libraries = HashSet::new();
        for summary in self.external_summaries.values() {
            libraries.insert(summary.library.clone());
        }
        for lang_summaries in self.stdlib_summaries.values() {
            for summary in lang_summaries.values() {
                libraries.insert(summary.library.clone());
            }
        }
        libraries
    }
}

/// Statistics for external library handling.
#[derive(Debug, Clone)]
pub struct LibraryStats {
    pub total_external_summaries: usize,
    pub stdlib_summaries: usize,
    pub dependencies_with_source: usize,
    pub total_libraries: usize,
}