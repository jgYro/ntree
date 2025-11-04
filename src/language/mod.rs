pub mod config;
pub mod detection;

pub use config::LanguageConfig;
pub use detection::{SupportedLanguage, detect_language_config};