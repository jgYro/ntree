/// Language-specific analyzers and language-agnostic IR normalization.

pub mod complexity_analyzer;
pub mod early_exit_normalizer;
pub mod for_loop_normalizer;
pub mod language_specific;

pub use complexity_analyzer::{ComplexityAnalyzer, ComplexityResult};
pub use early_exit_normalizer::EarlyExitNormalizer;
pub use for_loop_normalizer::ForLoopNormalizer;