use crate::models::ForLoopIR;
use crate::analyzers::language_specific::RustForLoopAnalyzer;
use tree_sitter::Node;

/// Language-agnostic for loop normalizer.
/// Routes to appropriate language-specific analyzers.
pub struct ForLoopNormalizer;

impl ForLoopNormalizer {
    /// Normalize a for loop from any supported language into language-agnostic IR.
    pub fn normalize(
        for_node: Node,
        source: &str,
        language: &str,
        loop_id: String,
    ) -> Option<ForLoopIR> {
        match language {
            "rust" => RustForLoopAnalyzer::analyze(for_node, source, loop_id),
            "javascript" | "js" => Self::analyze_c_style_for(for_node, source, loop_id),
            "java" => Self::analyze_c_style_for(for_node, source, loop_id),
            "c" | "cpp" => Self::analyze_c_style_for(for_node, source, loop_id),
            "python" => Self::analyze_python_for(for_node, source, loop_id),
            _ => None,
        }
    }

    /// Analyze C-style for loops: for(init; condition; update)
    /// Will be implemented when we add support for C/Java/JS.
    fn analyze_c_style_for(_for_node: Node, _source: &str, _loop_id: String) -> Option<ForLoopIR> {
        // TODO: Implement for C/Java/JavaScript support
        None
    }

    /// Analyze Python-style for loops: for x in xs:
    /// Will be implemented when we add Python support.
    fn analyze_python_for(_for_node: Node, _source: &str, _loop_id: String) -> Option<ForLoopIR> {
        // TODO: Implement for Python support
        None
    }

    /// Auto-detect language from node types (fallback).
    pub fn auto_detect_and_normalize(
        for_node: Node,
        source: &str,
        loop_id: String,
    ) -> Option<ForLoopIR> {
        // For now, assume Rust since that's what we're working with
        // Later we can add more sophisticated language detection
        Self::normalize(for_node, source, "rust", loop_id)
    }
}