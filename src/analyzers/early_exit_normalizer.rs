use crate::models::EarlyExitIR;
use crate::analyzers::language_specific::rust::RustEarlyExitAnalyzer;
use tree_sitter::Node;

/// Language-agnostic early-exit normalizer.
/// Routes to appropriate language-specific analyzers.
pub struct EarlyExitNormalizer;

impl EarlyExitNormalizer {
    /// Normalize an early-exit construct from any supported language into language-agnostic IR.
    pub fn normalize(
        node: Node,
        source: &str,
        language: &str,
        exit_id: String,
    ) -> Option<EarlyExitIR> {
        match language {
            "rust" => RustEarlyExitAnalyzer::analyze(node, source, exit_id),
            "javascript" | "js" => Self::analyze_js_early_exit(node, source, exit_id),
            "java" => Self::analyze_java_early_exit(node, source, exit_id),
            "c" | "cpp" => Self::analyze_c_early_exit(node, source, exit_id),
            "python" => Self::analyze_python_early_exit(node, source, exit_id),
            _ => None,
        }
    }

    /// Analyze JavaScript early-exit constructs: throw new Error().
    fn analyze_js_early_exit(_node: Node, _source: &str, _exit_id: String) -> Option<EarlyExitIR> {
        // TODO: Implement for JavaScript support
        // Would handle: throw new Error("message")
        None
    }

    /// Analyze Java early-exit constructs: throw new Exception().
    fn analyze_java_early_exit(_node: Node, _source: &str, _exit_id: String) -> Option<EarlyExitIR> {
        // TODO: Implement for Java support
        // Would handle: throw new RuntimeException("message")
        None
    }

    /// Analyze C/C++ early-exit constructs: exit() calls.
    fn analyze_c_early_exit(_node: Node, _source: &str, _exit_id: String) -> Option<EarlyExitIR> {
        // TODO: Implement for C/C++ support
        // Would handle: exit(1), abort(), etc.
        None
    }

    /// Analyze Python early-exit constructs: raise statements.
    fn analyze_python_early_exit(_node: Node, _source: &str, _exit_id: String) -> Option<EarlyExitIR> {
        // TODO: Implement for Python support
        // Would handle: raise Exception("message")
        None
    }

    /// Auto-detect language and normalize early-exit construct.
    pub fn auto_detect_and_normalize(
        node: Node,
        source: &str,
        exit_id: String,
    ) -> Option<EarlyExitIR> {
        // For now, assume Rust since that's what we're working with
        Self::normalize(node, source, "rust", exit_id)
    }
}