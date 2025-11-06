use super::module_graph::ModuleId;

/// Module identifier normalization utilities.
pub struct ModuleNormalizer;

impl ModuleNormalizer {
    /// Normalize module path based on language.
    pub fn normalize(path: &str, language: &str) -> ModuleId {
        match language {
            "rust" => Self::normalize_rust(path),
            "python" => Self::normalize_python(path),
            "javascript" | "typescript" => Self::normalize_js(path),
            "java" => Self::normalize_java(path),
            "c" | "cpp" => Self::normalize_c(path),
            _ => ModuleId::new(path.to_string()),
        }
    }

    /// Normalize Rust module path (crate::module::submodule).
    fn normalize_rust(path: &str) -> ModuleId {
        ModuleId::new(format!("rust:{}", path))
    }

    /// Normalize Python module path (package.module).
    fn normalize_python(path: &str) -> ModuleId {
        ModuleId::new(format!("python:{}", path))
    }

    /// Normalize JavaScript/TypeScript module path.
    fn normalize_js(path: &str) -> ModuleId {
        if path.starts_with("./") || path.starts_with("../") {
            ModuleId::new(format!("js:relative:{}", path))
        } else if path.starts_with('@') {
            ModuleId::new(format!("js:scoped:{}", path))
        } else {
            ModuleId::new(format!("js:package:{}", path))
        }
    }

    /// Normalize Java package path.
    fn normalize_java(path: &str) -> ModuleId {
        ModuleId::new(format!("java:{}", path))
    }

    /// Normalize C/C++ include path.
    fn normalize_c(path: &str) -> ModuleId {
        if path.starts_with('<') && path.ends_with('>') {
            // System header
            ModuleId::new(format!("c:system:{}", &path[1..path.len()-1]))
        } else {
            // Local header
            ModuleId::new(format!("c:local:{}", path.trim_matches('"')))
        }
    }

    /// Extract package version if available (for package@version syntax).
    pub fn extract_version(path: &str) -> Option<String> {
        if let Some(at_pos) = path.find('@') {
            if let Some(colon_pos) = path[at_pos..].find(':') {
                let version = &path[at_pos+1..at_pos+colon_pos];
                Some(version.to_string())
            } else {
                let version = &path[at_pos+1..];
                Some(version.to_string())
            }
        } else {
            None
        }
    }

    /// Create normalized ID with version info.
    pub fn normalize_with_version(path: &str, language: &str) -> ModuleId {
        match Self::extract_version(path) {
            Some(version) => {
                let base_path = path.split('@').next().unwrap_or(path);
                let normalized = Self::normalize(base_path, language);
                ModuleId::new(format!("{}@{}", normalized.as_str(), version))
            }
            None => Self::normalize(path, language),
        }
    }
}