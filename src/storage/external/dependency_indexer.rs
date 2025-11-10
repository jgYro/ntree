use std::collections::HashMap;
use std::path::PathBuf;
use crate::core::NTreeError;
use crate::storage::{SymbolStore, SymbolId};
use super::summary::ExternalSummary;

/// Types of dependencies that can be indexed.
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyType {
    /// Rust crate (Cargo.toml)
    CargoPackage,
    /// NPM package (package.json)
    NodeModule,
    /// Java JAR file
    JavaJar,
    /// Python package (setup.py/pyproject.toml)
    PythonPackage,
    /// Unknown/unsupported dependency type
    Unknown,
}

/// Indexer for dependency sources (jars/crates/node_modules).
#[derive(Debug)]
pub struct DependencyIndexer {
    /// Indexed dependency sources by library name
    indexed_dependencies: HashMap<String, DependencyInfo>,
    /// Source file cache
    source_cache: HashMap<PathBuf, SymbolStore>,
}

impl DependencyIndexer {
    /// Create new dependency indexer.
    pub fn new() -> Self {
        DependencyIndexer {
            indexed_dependencies: HashMap::new(),
            source_cache: HashMap::new(),
        }
    }

    /// Index a dependency directory or archive.
    pub fn index_dependency(
        &mut self,
        library_name: String,
        dependency_path: PathBuf,
        index_source: bool
    ) -> Result<(), NTreeError> {
        let mut dependency_info = DependencyInfo::new(library_name.clone(), dependency_path.clone());

        // Detect dependency type
        dependency_info.dependency_type = self.detect_dependency_type(&dependency_path);

        if index_source {
            // Index source files if available
            match dependency_info.dependency_type {
                DependencyType::CargoPackage => {
                    self.index_rust_crate(&dependency_path, &mut dependency_info)?;
                },
                DependencyType::NodeModule => {
                    self.index_npm_package(&dependency_path, &mut dependency_info)?;
                },
                DependencyType::JavaJar => {
                    // JAR indexing would require additional tooling
                    dependency_info.has_source = false;
                },
                DependencyType::PythonPackage => {
                    self.index_python_package(&dependency_path, &mut dependency_info)?;
                },
                DependencyType::Unknown => {
                    dependency_info.has_source = false;
                },
            }
        }

        self.indexed_dependencies.insert(library_name, dependency_info);
        Ok(())
    }

    /// Detect type of dependency.
    fn detect_dependency_type(&self, path: &PathBuf) -> DependencyType {
        if path.join("Cargo.toml").exists() {
            DependencyType::CargoPackage
        } else if path.join("package.json").exists() {
            DependencyType::NodeModule
        } else if path.extension().and_then(|s| s.to_str()) == Some("jar") {
            DependencyType::JavaJar
        } else if path.join("setup.py").exists() || path.join("pyproject.toml").exists() {
            DependencyType::PythonPackage
        } else {
            DependencyType::Unknown
        }
    }

    /// Index Rust crate source.
    fn index_rust_crate(&mut self, _path: &PathBuf, dependency_info: &mut DependencyInfo) -> Result<(), NTreeError> {
        // Placeholder: would use existing ntree analysis on dependency source
        dependency_info.has_source = true;
        dependency_info.exported_symbols = vec!["example::function".to_string()];
        Ok(())
    }

    /// Index NPM package source.
    fn index_npm_package(&mut self, _path: &PathBuf, dependency_info: &mut DependencyInfo) -> Result<(), NTreeError> {
        // Placeholder: would analyze JS/TS files
        dependency_info.has_source = true;
        dependency_info.exported_symbols = vec!["default".to_string()];
        Ok(())
    }

    /// Index Python package source.
    fn index_python_package(&mut self, _path: &PathBuf, dependency_info: &mut DependencyInfo) -> Result<(), NTreeError> {
        // Placeholder: would analyze Python files
        dependency_info.has_source = true;
        dependency_info.exported_symbols = vec!["__init__".to_string()];
        Ok(())
    }

    /// Get dependency information.
    pub fn get_dependency_info(&self, library: &str) -> Option<&DependencyInfo> {
        self.indexed_dependencies.get(library)
    }

    /// Check if dependency has indexed source.
    pub fn has_indexed_source(&self, library: &str) -> bool {
        self.indexed_dependencies
            .get(library)
            .map(|info| info.has_source)
            .unwrap_or(false)
    }

    /// Get indexer statistics.
    pub fn get_stats(&self) -> IndexerStats {
        let total_dependencies = self.indexed_dependencies.len();
        let with_source = self.indexed_dependencies.values()
            .filter(|info| info.has_source)
            .count();

        IndexerStats {
            total_dependencies,
            dependencies_with_source: with_source,
            cached_sources: self.source_cache.len(),
        }
    }
}

/// Information about an indexed dependency.
#[derive(Debug, Clone)]
pub struct DependencyInfo {
    /// Library name
    pub name: String,
    /// Path to dependency
    pub path: PathBuf,
    /// Type of dependency
    pub dependency_type: DependencyType,
    /// Whether source is available and indexed
    pub has_source: bool,
    /// Exported symbols (if indexed)
    pub exported_symbols: Vec<String>,
}

impl DependencyInfo {
    /// Create new dependency info.
    pub fn new(name: String, path: PathBuf) -> Self {
        DependencyInfo {
            name,
            path,
            dependency_type: DependencyType::Unknown,
            has_source: false,
            exported_symbols: Vec::new(),
        }
    }
}

/// Dependency indexer statistics.
#[derive(Debug, Clone)]
pub struct IndexerStats {
    pub total_dependencies: usize,
    pub dependencies_with_source: usize,
    pub cached_sources: usize,
}