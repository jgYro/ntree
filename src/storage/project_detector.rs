use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::core::NTreeError;
use super::file_walker::FileWalker;
use super::file_record::FileRecord;

/// Detects project roots and boundaries within a workspace.
pub struct ProjectDetector {
    /// Known project manifest files by language
    manifest_files: HashMap<String, Vec<String>>,
}

impl ProjectDetector {
    /// Create a new project detector.
    pub fn new() -> Self {
        let mut manifest_files = HashMap::new();

        manifest_files.insert("rust".to_string(), vec!["Cargo.toml".to_string()]);
        manifest_files.insert("python".to_string(), vec![
            "requirements.txt".to_string(),
            "pyproject.toml".to_string(),
            "Pipfile".to_string(),
            "setup.py".to_string(),
        ]);
        manifest_files.insert("javascript".to_string(), vec![
            "package.json".to_string(),
            "package-lock.json".to_string(),
        ]);
        manifest_files.insert("typescript".to_string(), vec![
            "package.json".to_string(),
            "tsconfig.json".to_string(),
        ]);
        manifest_files.insert("java".to_string(), vec![
            "pom.xml".to_string(),
            "build.gradle".to_string(),
            "build.gradle.kts".to_string(),
        ]);
        manifest_files.insert("c".to_string(), vec![
            "CMakeLists.txt".to_string(),
            "Makefile".to_string(),
            "configure.ac".to_string(),
        ]);
        manifest_files.insert("cpp".to_string(), vec![
            "CMakeLists.txt".to_string(),
            "Makefile".to_string(),
        ]);

        ProjectDetector { manifest_files }
    }

    /// Detect all projects within a workspace using existing FileWalker.
    pub fn detect_projects(&self, workspace_path: &Path) -> Result<Vec<ProjectInfo>, NTreeError> {
        let walker = FileWalker::new(workspace_path);
        let file_records = walker.discover_files()?;

        let mut projects = Vec::new();
        let mut processed_dirs = std::collections::HashSet::new();

        for file_record in &file_records {
            let file_path = &file_record.path;

            if let Some(parent_dir) = file_path.parent() {
                if processed_dirs.contains(parent_dir) {
                    continue;
                }

                if let Some(project_info) = self.detect_project_in_directory(parent_dir, &file_records)? {
                    projects.push(project_info);
                    processed_dirs.insert(parent_dir);
                }
            }
        }

        // If no projects found, treat entire workspace as single project
        if projects.is_empty() {
            projects.push(ProjectInfo {
                root_path: workspace_path.to_path_buf(),
                project_type: ProjectType::Generic,
                manifest_file: None,
                source_files: file_records,
            });
        }

        Ok(projects)
    }

    /// Detect project in a specific directory.
    fn detect_project_in_directory(
        &self,
        dir_path: &Path,
        all_files: &[FileRecord],
    ) -> Result<Option<ProjectInfo>, NTreeError> {
        for (language, manifest_names) in &self.manifest_files {
            for manifest_name in manifest_names {
                let manifest_path = dir_path.join(manifest_name);
                if manifest_path.exists() {
                    let source_files = self.collect_project_files(dir_path, all_files);
                    let project_type = ProjectType::from_language(language);

                    return Ok(Some(ProjectInfo {
                        root_path: dir_path.to_path_buf(),
                        project_type,
                        manifest_file: Some(manifest_path),
                        source_files,
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Collect source files belonging to a project.
    fn collect_project_files(&self, project_root: &Path, all_files: &[FileRecord]) -> Vec<FileRecord> {
        all_files
            .iter()
            .filter(|file| file.path.starts_with(project_root))
            .cloned()
            .collect()
    }
}

/// Information about a detected project.
#[derive(Debug, Clone)]
pub struct ProjectInfo {
    /// Root directory of the project
    pub root_path: PathBuf,
    /// Type of project detected
    pub project_type: ProjectType,
    /// Path to the manifest file (if any)
    pub manifest_file: Option<PathBuf>,
    /// Source files belonging to this project
    pub source_files: Vec<FileRecord>,
}

/// Types of projects that can be detected.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Java,
    C,
    Cpp,
    Generic,
}

impl ProjectType {
    /// Convert language name to project type.
    fn from_language(language: &str) -> Self {
        match language {
            "rust" => ProjectType::Rust,
            "python" => ProjectType::Python,
            "javascript" => ProjectType::JavaScript,
            "typescript" => ProjectType::TypeScript,
            "java" => ProjectType::Java,
            "c" => ProjectType::C,
            "cpp" => ProjectType::Cpp,
            _ => ProjectType::Generic,
        }
    }
}