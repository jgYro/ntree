pub mod config;
pub mod detection;

pub use config::LanguageConfig;
pub use detection::{detect_language_config, SupportedLanguage};
