/// Language-specific analyzers and language-agnostic IR normalization.

pub mod language_specific;
pub mod for_loop_normalizer;

pub use for_loop_normalizer::ForLoopNormalizer;