use super::LanguageConfig;
use crate::core::NTreeError;
use std::path::Path;

/// Supported programming languages for analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum SupportedLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Java,
    C,
    Cpp,
}

impl SupportedLanguage {
    /// Detect language from file extension.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, NTreeError> {
        let path = path.as_ref();

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => Ok(SupportedLanguage::Rust),
            Some("py") => Ok(SupportedLanguage::Python),
            Some("js") | Some("mjs") => Ok(SupportedLanguage::JavaScript),
            Some("ts") => Ok(SupportedLanguage::TypeScript),
            Some("java") => Ok(SupportedLanguage::Java),
            Some("c") | Some("h") => Ok(SupportedLanguage::C),
            Some("cpp") | Some("cc") | Some("cxx") | Some("hpp") | Some("hxx") => {
                Ok(SupportedLanguage::Cpp)
            }
            Some(ext) => Err(NTreeError::ParseError(format!(
                "Unsupported file extension: .{}",
                ext
            ))),
            None => Err(NTreeError::ParseError(
                "Unable to determine file extension".to_string(),
            )),
        }
    }

    /// Get the corresponding language configuration.
    pub fn get_config(&self) -> LanguageConfig {
        match self {
            SupportedLanguage::Rust => LanguageConfig::rust(),
            SupportedLanguage::Python => LanguageConfig::python(),
            SupportedLanguage::JavaScript => LanguageConfig::javascript(),
            SupportedLanguage::TypeScript => LanguageConfig::typescript(),
            SupportedLanguage::Java => LanguageConfig::java(),
            SupportedLanguage::C => LanguageConfig::c(),
            SupportedLanguage::Cpp => LanguageConfig::cpp(),
        }
    }

    /// Get language name as string.
    pub fn name(&self) -> &'static str {
        match self {
            SupportedLanguage::Rust => "rust",
            SupportedLanguage::Python => "python",
            SupportedLanguage::JavaScript => "javascript",
            SupportedLanguage::TypeScript => "typescript",
            SupportedLanguage::Java => "java",
            SupportedLanguage::C => "c",
            SupportedLanguage::Cpp => "cpp",
        }
    }
}

/// Detect language and create appropriate parser configuration.
pub fn detect_language_config<P: AsRef<Path>>(path: P) -> Result<LanguageConfig, NTreeError> {
    let language = SupportedLanguage::from_path(path)?;
    Ok(language.get_config())
}
