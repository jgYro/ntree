use crate::core::NTreeError;
use crate::storage::{CallGraph, SymbolStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Deep call chain tracking external library function calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepCallChain {
    /// The external function call (e.g., "requests.get")
    pub external_call: String,
    /// The library/module name (e.g., "requests")
    pub library: String,
    /// Internal functions called by the external function (just function names)
    pub internal_calls: Vec<String>,
    /// Full call expressions found in the function (with context)
    pub call_expressions: Vec<String>,
    /// Certificates and certificate-related parameters found in this call chain
    pub certificates: Vec<String>,
    /// Whether the library source was available for analysis
    pub source_available: bool,
    /// File path where the function was found (if source available)
    pub source_file: Option<String>,
    /// Recursive call chains - what each internal function calls
    pub nested_chains: Vec<DeepCallChain>,
}

/// Tracker for deep external library call analysis.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeepCallTracker {
    /// Call chains discovered
    call_chains: Vec<DeepCallChain>,
    /// Library paths indexed
    library_paths: HashMap<String, PathBuf>,
    /// Workspace path being analyzed
    workspace_path: Option<PathBuf>,
    /// Maximum recursion depth for call chain analysis
    max_depth: usize,
    /// Functions already analyzed (to avoid cycles)
    analyzed_functions: std::collections::HashSet<String>,
}

impl DeepCallTracker {
    /// Create a new deep call tracker.
    pub fn new() -> Self {
        DeepCallTracker {
            call_chains: Vec::new(),
            library_paths: HashMap::new(),
            workspace_path: None,
            max_depth: 5, // Default max depth
            analyzed_functions: std::collections::HashSet::new(),
        }
    }

    /// Create a new deep call tracker with workspace path.
    pub fn with_workspace_path(workspace_path: PathBuf) -> Self {
        DeepCallTracker {
            call_chains: Vec::new(),
            library_paths: HashMap::new(),
            workspace_path: Some(workspace_path),
            max_depth: 5, // Default max depth
            analyzed_functions: std::collections::HashSet::new(),
        }
    }

    /// Set maximum recursion depth for call chain analysis.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Analyze external calls and track their internal function calls.
    pub fn analyze_external_calls(
        &mut self,
        call_graph: &CallGraph,
        _symbol_store: &SymbolStore,
    ) -> Result<(), NTreeError> {
        // Get all call edges
        let call_edges = call_graph.get_all_call_edges();

        for call_edge in call_edges {
            // Check if this is an external library call
            if let Some(library) = Self::extract_library_name(&call_edge.callee_expr_text) {
                // Skip if it's a builtin like "print"
                if library == "print" || library == "len" || library == "str" || library == "int" || library == "float" {
                    continue;
                }
                
                // Try to find the library source
                let library_path = self.find_library_source(&library);

                let (internal_calls, call_expressions, mut certificates, nested_chains) = if let Some(path) = &library_path {
                    // Analyze the library source to find internal calls recursively
                    self.analyze_library_function_recursive(&library, &call_edge.callee_expr_text, path, 0)?
                } else {
                    (Vec::new(), Vec::new(), Vec::new(), Vec::new())
                };

                // For requests library, always check for default certificate bundle
                if library == "requests" {
                    if let Some(cert_path) = Self::find_default_certificate_bundle() {
                        let cert_entry = format!("Default certificate bundle: {}", cert_path);
                        if !certificates.contains(&cert_entry) {
                            certificates.push(cert_entry);
                        }
                    }
                }

                let source_file = library_path.as_ref().map(|p| {
                    // Try to find the specific file containing the function
                    if let Some(file) = self.find_function_file(&library, &call_edge.callee_expr_text, p) {
                        file.to_string_lossy().to_string()
                    } else {
                        p.to_string_lossy().to_string()
                    }
                });

                // Extract certificates from call expressions and nested chains
                let mut all_certificates = Self::extract_certificates_from_calls(&call_expressions);
                // Also collect from nested chains
                for nested in &nested_chains {
                    for cert in &nested.certificates {
                        if !all_certificates.contains(cert) {
                            all_certificates.push(cert.clone());
                        }
                    }
                }
                
                // Also add certificates found in the recursive analysis
                for cert in &certificates {
                    if !all_certificates.contains(cert) {
                        all_certificates.push(cert.clone());
                    }
                }
                
                self.call_chains.push(DeepCallChain {
                    external_call: call_edge.callee_expr_text.clone(),
                    library: library.clone(),
                    internal_calls,
                    call_expressions,
                    certificates,
                    source_available: library_path.is_some(),
                    source_file,
                    nested_chains,
                });
            }
        }

        Ok(())
    }

    /// Extract certificates and certificate-related parameters from call expressions.
    fn extract_certificates_from_calls(calls: &[String]) -> Vec<String> {
        let mut certs = Vec::new();
        
        for call in calls {
            // First priority: Look for actual certificate file paths with extensions
            if call.contains(".pem") || call.contains(".crt") || call.contains(".cer") || 
               call.contains(".key") || call.contains(".p12") || call.contains(".pfx") ||
               call.contains(".cert") {
                // Extract the file path
                if let Some(cert_path) = Self::extract_file_path(call) {
                    // Only add if it's an actual file path, not a variable
                    if !cert_path.contains("self.") && !cert_path.contains("print") &&
                       (cert_path.contains("/") || cert_path.contains("\\") || 
                        cert_path.starts_with("\"") || cert_path.starts_with("'")) {
                        let clean_path = cert_path.trim_matches('"').trim_matches('\'').to_string();
                        if !certs.contains(&clean_path) {
                            certs.push(format!("Certificate file: {}", clean_path));
                        }
                    }
                }
            }
            
            // Second priority: Look for certificate-related parameters with actual file paths
            let cert_keywords = vec!["cert=", "verify=", "ca_certs=", "certfile=", "keyfile="];
            for keyword in &cert_keywords {
                if call.contains(keyword) {
                    // Try to extract the certificate path/value
                    if let Some(cert_info) = Self::extract_cert_from_call(call, keyword) {
                        // Only add if it's an actual path/value, not a variable name
                        let value = if cert_info.contains('=') {
                            cert_info.split('=').nth(1).unwrap_or("")
                        } else {
                            &cert_info
                        };
                        
                        // Check if it's an actual file path (not a variable)
                        if (value.contains("/") || value.contains("\\") || 
                            value.contains(".pem") || value.contains(".crt") ||
                            value.contains(".key") || value.starts_with("\"") ||
                            value.starts_with("'") || value == "True" || value == "False") &&
                           !value.contains("self.") && !value.contains("cert") &&
                           !value.contains("verify") && !value.contains("print") {
                            if !certs.contains(&cert_info) {
                                certs.push(cert_info);
                            }
                        }
                    }
                }
            }
            
            // Look for SSL context creation with actual certificate files
            if call.contains("load_cert_chain") || call.contains("load_verify_locations") {
                if let Some(cert_path) = Self::extract_file_path(call) {
                    if !cert_path.contains("self.") && !cert_path.contains("print") {
                        let clean_path = cert_path.trim_matches('"').trim_matches('\'').to_string();
                        if !certs.contains(&clean_path) {
                            certs.push(format!("SSL Certificate: {}", clean_path));
                        }
                    }
                }
            }
        }
        
        certs
    }

    /// Extract certificate information from a call expression.
    fn extract_cert_from_call(call: &str, keyword: &str) -> Option<String> {
        // Look for patterns like cert="path", cert='path', cert=path, etc.
        if let Some(keyword_pos) = call.find(keyword) {
            let after_keyword = &call[keyword_pos + keyword.len()..];
            
            // Handle cert="path" or cert='path' (quoted paths - these are actual file paths)
            if after_keyword.starts_with("=\"") || after_keyword.starts_with("='") {
                let quote_char = if after_keyword.starts_with("=\"") { '"' } else { '\'' };
                if let Some(start) = after_keyword.find(quote_char) {
                    let value_start = start + 1;
                    if let Some(end) = after_keyword[value_start..].find(quote_char) {
                        let value = &after_keyword[value_start..value_start + end];
                        // Only return if it looks like an actual path/value, not a variable
                        if value.contains("/") || value.contains("\\") || 
                           value.contains(".pem") || value.contains(".crt") ||
                           value.contains(".key") || value == "True" || value == "False" ||
                           value.parse::<bool>().is_ok() {
                            return Some(format!("{}={}", keyword.trim_end_matches('='), value));
                        }
                    }
                }
            }
            
            // Handle cert=variable or cert=path (no quotes)
            let trimmed = after_keyword.trim_start_matches('=').trim();
            if !trimmed.is_empty() {
                // Take up to the next comma, closing paren, or whitespace
                let end = trimmed.find(&[',', ')', ' ', '\n', '\t'][..])
                    .unwrap_or(trimmed.len());
                let value = trimmed[..end].trim();
                // Only return if it looks like an actual path/value
                if !value.is_empty() && !value.starts_with('*') &&
                   (value.contains("/") || value.contains("\\") || 
                    value.contains(".pem") || value.contains(".crt") ||
                    value.contains(".key") || value == "True" || value == "False" ||
                    value.parse::<bool>().is_ok() || value.starts_with('"') ||
                    value.starts_with('\'')) {
                    return Some(format!("{}={}", keyword.trim_end_matches('='), value));
                }
            }
        }
        
        None
    }

    /// Extract file path from a call expression.
    fn extract_file_path(call: &str) -> Option<String> {
        // Look for file paths with certificate extensions
        // Simple extraction - look for quoted paths
        if let Some(start) = call.find('"') {
            if let Some(end) = call[start+1..].find('"') {
                let path = &call[start+1..start+1+end];
                if path.contains(".pem") || path.contains(".crt") || path.contains(".cer") || 
                   path.contains(".key") || path.contains(".p12") || path.contains(".pfx") ||
                   path.contains(".cert") {
                    return Some(path.to_string());
                }
            }
        }
        if let Some(start) = call.find('\'') {
            if let Some(end) = call[start+1..].find('\'') {
                let path = &call[start+1..start+1+end];
                if path.contains(".pem") || path.contains(".crt") || path.contains(".cer") || 
                   path.contains(".key") || path.contains(".p12") || path.contains(".pfx") ||
                   path.contains(".cert") {
                    return Some(path.to_string());
                }
            }
        }
        
        // Also look for unquoted paths (e.g., cert=/path/to/cert.pem)
        let cert_extensions = [".pem", ".crt", ".cer", ".key", ".p12", ".pfx", ".cert"];
        for ext in &cert_extensions {
            if let Some(pos) = call.find(ext) {
                // Look backwards to find the start of the path
                let mut start = pos;
                while start > 0 {
                    let ch = call.chars().nth(start - 1).unwrap_or(' ');
                    if ch == ' ' || ch == '=' || ch == '"' || ch == '\'' || ch == '/' || ch == '\\' {
                        break;
                    }
                    start -= 1;
                }
                // Look forwards to find the end
                let mut end = pos + ext.len();
                while end < call.len() {
                    let ch = call.chars().nth(end).unwrap_or(' ');
                    if ch == ' ' || ch == ',' || ch == ')' || ch == '\n' || ch == '\t' {
                        break;
                    }
                    end += 1;
                }
                let path = &call[start..end].trim();
                if !path.is_empty() {
                    return Some(path.to_string());
                }
            }
        }
        
        None
    }

    /// Extract certificates directly from source code.
    fn extract_certificates_from_source(source: &str) -> Vec<String> {
        let mut certs = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        
        for line in lines {
            let trimmed = line.trim();
            
            // Look for certificate file paths in the source (actual file paths, not variables)
            let cert_extensions = [".pem", ".crt", ".cer", ".key", ".p12", ".pfx", ".cert"];
            for ext in &cert_extensions {
                if trimmed.contains(ext) {
                    // Try to extract the path
                    if let Some(path) = Self::extract_file_path(trimmed) {
                        // Only add if it's an actual file path, not a variable
                        if !path.contains("self.") && !path.contains("print") &&
                           (path.contains("/") || path.contains("\\") || 
                            path.starts_with("\"") || path.starts_with("'")) {
                            let clean_path = path.trim_matches('"').trim_matches('\'').to_string();
                            if !certs.contains(&clean_path) {
                                certs.push(format!("Certificate file in source: {}", clean_path));
                            }
                        }
                    }
                }
            }
            
            // Look for certifi or default certificate bundle references
            if trimmed.contains("certifi") || trimmed.contains("where()") ||
               trimmed.contains("DEFAULT_CA_BUNDLE") || trimmed.contains("REQUESTS_CA_BUNDLE") ||
               trimmed.contains("SSL_CERT_FILE") || trimmed.contains("CURL_CA_BUNDLE") ||
               trimmed.contains("from certifi") || trimmed.contains("import certifi") {
                // Try to find the actual certificate bundle path
                if let Some(cert_path) = Self::find_default_certificate_bundle() {
                    let cert_entry = format!("Default certificate bundle: {}", cert_path);
                    if !certs.contains(&cert_entry) {
                        certs.push(cert_entry);
                    }
                }
            }
            
            // Look for cert= or verify= with actual file paths (not variables)
            if trimmed.contains("cert=") {
                if let Some(cert_info) = Self::extract_cert_from_call(trimmed, "cert=") {
                    let value = cert_info.split('=').nth(1).unwrap_or("");
                    // Only add if it's an actual path, not a variable
                    if (value.contains("/") || value.contains("\\") || 
                        value.contains(".pem") || value.contains(".crt") ||
                        value.contains(".key") || value.starts_with("\"") ||
                        value.starts_with("'")) && !value.contains("self.") &&
                       !value.contains("cert") && !certs.contains(&cert_info) {
                        certs.push(cert_info);
                    }
                }
            }
            if trimmed.contains("verify=") {
                if let Some(verify_info) = Self::extract_cert_from_call(trimmed, "verify=") {
                    let value = verify_info.split('=').nth(1).unwrap_or("");
                    // Only add if it's an actual path or boolean, not a variable
                    if (value == "True" || value == "False" || 
                        value.contains("/") || value.contains("\\") ||
                        value.starts_with("\"") || value.starts_with("'")) &&
                       !value.contains("self.") && !value.contains("verify") &&
                       !certs.contains(&verify_info) {
                        certs.push(verify_info);
                    }
                }
            }
        }
        
        certs
    }

    /// Find default certificate bundle location used by requests/urllib3.
    fn find_default_certificate_bundle() -> Option<String> {
        // Check environment variables first
        if let Ok(ca_bundle) = std::env::var("REQUESTS_CA_BUNDLE") {
            if std::path::Path::new(&ca_bundle).exists() {
                return Some(ca_bundle);
            }
        }
        if let Ok(ssl_cert) = std::env::var("SSL_CERT_FILE") {
            if std::path::Path::new(&ssl_cert).exists() {
                return Some(ssl_cert);
            }
        }
        if let Ok(curl_ca) = std::env::var("CURL_CA_BUNDLE") {
            if std::path::Path::new(&curl_ca).exists() {
                return Some(curl_ca);
            }
        }
        
        // Try to execute Python to get certifi location (most reliable method)
        // Try python, python3, py commands
        let python_commands = vec!["python", "python3", "py"];
        for cmd in python_commands {
            if let Ok(output) = std::process::Command::new(cmd)
                .arg("-c")
                .arg("import certifi; print(certifi.where())")
                .output() {
                if output.status.success() {
                    if let Ok(cert_path) = String::from_utf8(output.stdout) {
                        let cert_path = cert_path.trim();
                        if !cert_path.is_empty() && std::path::Path::new(cert_path).exists() {
                            return Some(cert_path.to_string());
                        }
                    }
                }
            }
        }
        
        // Fallback: Try common Python installation locations
        let python_versions = ["Python312", "Python311", "Python310", "Python39", "Python38",
                                "python3.12", "python3.11", "python3.10", "python3.9", "python3.8"];
        
        // Windows - check LOCALAPPDATA (standard Python)
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            for version in &python_versions {
                let cert_path = std::path::Path::new(&appdata)
                    .join("Programs")
                    .join("Python")
                    .join(version)
                    .join("lib")
                    .join("site-packages")
                    .join("certifi")
                    .join("cacert.pem");
                if cert_path.exists() {
                    return Some(cert_path.to_string_lossy().to_string());
                }
            }
            
            // Windows Store Python location
            let store_path = std::path::Path::new(&appdata)
                .join("Packages");
            if store_path.exists() {
                use jwalk::WalkDir;
                for entry in WalkDir::new(&store_path).max_depth(6) {
                    if let Ok(entry) = entry {
                        let entry_path = entry.path();
                        let path_str = entry_path.to_string_lossy();
                        if path_str.contains("certifi") && path_str.ends_with("cacert.pem") {
                            if entry_path.exists() {
                                return Some(entry_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Windows - check Program Files
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            for version in &python_versions {
                let cert_path = std::path::Path::new(&program_files)
                    .join("Python")
                    .join(version)
                    .join("lib")
                    .join("site-packages")
                    .join("certifi")
                    .join("cacert.pem");
                if cert_path.exists() {
                    return Some(cert_path.to_string_lossy().to_string());
                }
            }
        }
        
        // Linux/Mac
        if let Ok(home) = std::env::var("HOME") {
            for version in &python_versions {
                let cert_path = std::path::Path::new(&home)
                    .join(".local")
                    .join("lib")
                    .join(version)
                    .join("site-packages")
                    .join("certifi")
                    .join("cacert.pem");
                if cert_path.exists() {
                    return Some(cert_path.to_string_lossy().to_string());
                }
            }
        }
        
        // Try to find certifi in virtual environments
        if let Ok(current_dir) = std::env::current_dir() {
            let mut search_dir = current_dir.clone();
            for _ in 0..5 {
                let venv_cert = search_dir.join(".venv")
                    .join("lib")
                    .join("site-packages")
                    .join("certifi")
                    .join("cacert.pem");
                if venv_cert.exists() {
                    return Some(venv_cert.to_string_lossy().to_string());
                }
                
                // Also check Windows venv location
                let venv_cert_win = search_dir.join(".venv")
                    .join("Lib")
                    .join("site-packages")
                    .join("certifi")
                    .join("cacert.pem");
                if venv_cert_win.exists() {
                    return Some(venv_cert_win.to_string_lossy().to_string());
                }
                
                if let Some(parent) = search_dir.parent() {
                    search_dir = parent.to_path_buf();
                } else {
                    break;
                }
            }
        }
        
        None
    }

    /// Extract library name from call expression (e.g., "requests.get" -> "requests").
    fn extract_library_name(call_expr: &str) -> Option<String> {
        // Handle nested calls like "print(requests.get(url).text)" -> "requests"
        // Look for patterns like "requests.get", "requests.get(", etc.
        
        // For Python: look for "module.function" or "module.function(" patterns
        // Common Python libraries: requests, numpy, pandas, os, sys, etc.
        let common_libs = ["requests", "numpy", "pandas", "os", "sys", "json", "math", "random", "datetime", "collections"];
        
        // First, try to find a common library name in the expression
        for lib in &common_libs {
            // Look for patterns like "lib.function" or "lib.function("
            let pattern1 = format!("{}.", lib);
            let pattern2 = format!("{}.get", lib);
            let pattern3 = format!("{}.post", lib);
            let pattern4 = format!("{}.put", lib);
            
            if call_expr.contains(&pattern1) || 
               call_expr.contains(&pattern2) || 
               call_expr.contains(&pattern3) || 
               call_expr.contains(&pattern4) {
                return Some(lib.to_string());
            }
        }
        
        // Fallback: try to extract from dot notation
        // For "requests.get(url)" -> "requests"
        // For "print(requests.get(url).text)" -> find "requests.get"
        if let Some(dot_pos) = call_expr.find('.') {
            // Look backwards from the dot to find the start of the identifier
            let before_dot = &call_expr[..dot_pos];
            // Find the last non-alphanumeric character before the dot
            let mut start = 0;
            for (i, ch) in before_dot.char_indices().rev() {
                if ch.is_alphanumeric() || ch == '_' {
                    start = i;
                } else {
                    break;
                }
            }
            // Extract the library name
            let lib_name = &before_dot[start..];
            if !lib_name.is_empty() && lib_name.chars().next().unwrap().is_alphabetic() {
                return Some(lib_name.to_string());
            }
        }
        
        // For Rust: "std::vec::Vec::new" -> "std"
        if call_expr.contains("::") {
            return call_expr.split("::").next().map(|s| s.trim().to_string());
        }
        
        None
    }

    /// Find library source path (checks common locations including virtual environments).
    fn find_library_source(&self, library: &str) -> Option<PathBuf> {
        // Check if already indexed
        if let Some(path) = self.library_paths.get(library) {
            if path.exists() {
                return Some(path.clone());
            }
        }

        // Try virtual environment locations first (.venv, venv, env)
        // Start with workspace path if available, then current directory
        let search_dirs: Vec<PathBuf> = if let Some(ref workspace) = self.workspace_path {
            vec![workspace.clone(), std::env::current_dir().ok()?]
        } else {
            vec![std::env::current_dir().ok()?]
        };

        for start_dir in search_dirs {
            // Check for .venv in the start directory and parent directories
            let mut search_dir = start_dir.clone();
            for _ in 0..5 {
                // Check .venv
                let venv_path = search_dir.join(".venv");
                if venv_path.exists() {
                    if let Some(lib_path) = Self::check_venv_site_packages(&venv_path, library) {
                        return Some(lib_path);
                    }
                }
                
                // Check venv
                let venv_path2 = search_dir.join("venv");
                if let Some(lib_path) = Self::check_venv_site_packages(&venv_path2, library) {
                    return Some(lib_path);
                }
                
                // Check env
                let venv_path3 = search_dir.join("env");
                if let Some(lib_path) = Self::check_venv_site_packages(&venv_path3, library) {
                    return Some(lib_path);
                }
                
                // Move up one directory
                if let Some(parent) = search_dir.parent() {
                    search_dir = parent.to_path_buf();
                } else {
                    break;
                }
            }
        }

        // Try PYTHONPATH environment variable
        if let Ok(python_path) = std::env::var("PYTHONPATH") {
            for path_str in python_path.split(':') {
                let lib_path = PathBuf::from(path_str).join(library);
                if lib_path.exists() {
                    return Some(lib_path);
                }
            }
        }

        // Try user site-packages
        if let Ok(home) = std::env::var("HOME") {
            for python_version in ["python3.12", "python3.11", "python3.10", "python3.9", "python3.8"] {
                let site_packages = PathBuf::from(&home)
                    .join(".local/lib")
                    .join(python_version)
                    .join("site-packages")
                    .join(library);
                if site_packages.exists() {
                    return Some(site_packages);
                }
            }
        }

        // Try Windows user site-packages
        if let Ok(appdata) = std::env::var("APPDATA") {
            for python_version in ["Python312", "Python311", "Python310", "Python39", "Python38"] {
                let site_packages = PathBuf::from(&appdata)
                    .join("Python")
                    .join(python_version)
                    .join("site-packages")
                    .join(library);
                if site_packages.exists() {
                    return Some(site_packages);
                }
            }
        }

        None
    }

    /// Check site-packages in a virtual environment directory.
    fn check_venv_site_packages(venv_path: &PathBuf, library: &str) -> Option<PathBuf> {
        if !venv_path.exists() {
            return None;
        }

        // Check common site-packages locations in venv
        let mut site_packages_paths = vec![
            venv_path.join("lib").join("site-packages"),
            venv_path.join("Lib").join("site-packages"), // Windows
        ];

        // Also try to find site-packages by Python version
        for python_version in ["python3.12", "python3.11", "python3.10", "python3.9", "python3.8", "python3"] {
            site_packages_paths.push(venv_path.join("lib").join(python_version).join("site-packages"));
        }

        for site_packages in site_packages_paths {
            if site_packages.exists() {
                let lib_path = site_packages.join(library);
                if lib_path.exists() {
                    return Some(lib_path);
                }
            }
        }

        None
    }

    /// Analyze a library function to find its internal calls recursively.
    fn analyze_library_function_recursive(
        &mut self,
        library: &str,
        function_call: &str,
        library_path: &PathBuf,
        depth: usize,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<DeepCallChain>), NTreeError> {
        // Prevent infinite recursion
        if depth >= self.max_depth {
            return Ok((Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        }

        // Extract function name from call (e.g., "requests.get" -> "get", "requests.request" -> "request")
        let function_name = if let Some(name) = function_call.split('.').nth(1) {
            name.split('(').next().unwrap_or(name).trim()
        } else {
            return Ok((Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        };
        
        // Check if this is likely a method call (e.g., "session.request" suggests a class method)
        // If the original call had a dot and the object name isn't the library name, it's probably a method
        // Also check if function name is common to both module-level and class methods (like "request")
        let is_likely_method = (function_call.contains('.') && 
                               !function_call.starts_with(&format!("{}.", library))) ||
                               // If function name is "request" and library is "requests", it might be Session.request()
                               (function_name == "request" && library == "requests");

        // Create a unique identifier for this function to avoid cycles
        let func_id = format!("{}::{}", library, function_name);
        if self.analyzed_functions.contains(&func_id) {
            return Ok((Vec::new(), Vec::new(), Vec::new(), Vec::new())); // Already analyzed, skip to avoid cycles
        }
        self.analyzed_functions.insert(func_id.clone());

        // Try to find the function definition in the library
        let mut internal_calls = Vec::new();
        let mut call_expressions = Vec::new();
        let mut certificates = Vec::new();
        let mut nested_chains = Vec::new();

        // Try to find the function file using the dedicated method
        // If it's likely a method call, prioritize sessions.py for requests library
        let function_file = if is_likely_method && library == "requests" {
            // For method calls in requests, check sessions.py first
            let sessions_path = library_path.join("sessions.py");
            if sessions_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&sessions_path) {
                    if self.has_function_definition(&content, function_name, false) {
                        Some(sessions_path)
                    } else {
                        self.find_function_file(library, function_call, library_path)
                    }
                } else {
                    self.find_function_file(library, function_call, library_path)
                }
            } else {
                self.find_function_file(library, function_call, library_path)
            }
        } else {
            self.find_function_file(library, function_call, library_path)
        };
        
        if let Some(function_file) = function_file {
            // Also extract certificate information directly from the source file
            if let Ok(content) = std::fs::read_to_string(&function_file) {
                let file_certs = Self::extract_certificates_from_source(&content);
                for cert in file_certs {
                    if !certificates.contains(&cert) {
                        certificates.push(cert);
                    }
                }
                
                // Check if this file uses certifi or default certificate bundles
                // requests uses certifi for default certificates - always check for requests library
                if library == "requests" || content.contains("certifi") || 
                   content.contains("DEFAULT_CA_BUNDLE") || content.contains("from certifi") ||
                   content.contains("import certifi") {
                    if let Some(cert_path) = Self::find_default_certificate_bundle() {
                        let cert_entry = format!("Default certificate bundle: {}", cert_path);
                        if !certificates.contains(&cert_entry) {
                            certificates.push(cert_entry);
                        }
                    }
                }
            }
            
            // Extract calls from this function
            if let Ok(calls) = self.extract_calls_from_file(&function_file, function_name) {
                // Use sets to deduplicate both function names and call expressions
                use std::collections::HashSet;
                let mut seen_calls = HashSet::new();
                let mut seen_expressions = HashSet::new();
                
                // Process each call recursively
                for call in &calls {
                    // Deduplicate call expressions
                    if !seen_expressions.contains(call) {
                        seen_expressions.insert(call.clone());
                        call_expressions.push(call.clone());
                    }
                    
                    // Extract the function name from the call
                    let called_func = self.extract_function_name_from_call(call);
                    if !called_func.is_empty() && !seen_calls.contains(&called_func) {
                        seen_calls.insert(called_func.clone());
                        internal_calls.push(called_func.clone());
                        
                        // Recursively analyze this function if it's in the same library
                        // Check if it's a method call (self.method, session.method, etc.) or module call
                        let is_internal = call.contains("self.") || 
                                          call.starts_with(&format!("{}.", library)) ||
                                          (!call.contains('.') && !call.starts_with("print") && !call.starts_with("len") && !call.starts_with("dict") && !call.starts_with("list") && !call.starts_with("str") && !call.starts_with("int"));
                        
                        // Also check for method calls on objects (e.g., session.request, obj.method)
                        let is_method_call = call.contains('.') && !call.starts_with("print") && !call.starts_with("len");
                        
                        if is_internal || is_method_call {
                            // Try to find and analyze this function recursively
                            let nested_call = if call.contains("self.") {
                                // self.method() -> find method in the same class
                                format!("{}.{}", library, called_func)
                            } else if call.contains('.') && !call.starts_with(&format!("{}.", library)) {
                                // Method call like session.request() or obj.method()
                                // Extract the method name and search for it in the library
                                let method_name = called_func.clone();
                                format!("{}.{}", library, method_name)
                            } else if call.contains('.') {
                                // Already has library prefix
                                call.clone()
                            } else {
                                // Simple function call like "request(...)" - assume it's in the same library
                                format!("{}.{}", library, called_func)
                            };
                            
                            if let Ok((nested_internal, nested_expressions, nested_certs, nested)) = self.analyze_library_function_recursive(
                                library,
                                &nested_call,
                                library_path,
                                depth + 1,
                            ) {
                                // Collect certificates from nested chains
                                for cert in &nested_certs {
                                    if !certificates.contains(cert) {
                                        certificates.push(cert.clone());
                                    }
                                }
                                
                                // Always add the nested chain if we found calls or nested chains
                                if !nested_internal.is_empty() || !nested_expressions.is_empty() || !nested_certs.is_empty() || !nested.is_empty() {
                                    nested_chains.push(DeepCallChain {
                                        external_call: nested_call.clone(),
                                        library: library.to_string(),
                                        internal_calls: nested_internal,
                                        call_expressions: nested_expressions,
                                        certificates: nested_certs,
                                        source_available: true,
                                        source_file: Some(function_file.to_string_lossy().to_string()),
                                        nested_chains: nested,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Fallback: search all files if find_function_file didn't find it
            if library_path.is_dir() {
                use jwalk::WalkDir;
                for entry in WalkDir::new(library_path).max_depth(3) {
                    if let Ok(entry) = entry {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "py" {
                                // Check if this file contains the function at module level
                                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                                    // Check for function definition at module level (not indented)
                                    if self.has_function_definition(&content, function_name, true) {
                                        // Extract calls from this function
                                        if let Ok(calls) =
                                            self.extract_calls_from_file(&entry.path(), function_name)
                                        {
                                            // Use sets to deduplicate both function names and call expressions
                                            use std::collections::HashSet;
                                            let mut seen_calls = HashSet::new();
                                            let mut seen_expressions = HashSet::new();
                                            
                                            // Process each call recursively
                                            for call in &calls {
                                                // Deduplicate call expressions
                                                if !seen_expressions.contains(call) {
                                                    seen_expressions.insert(call.clone());
                                                    call_expressions.push(call.clone());
                                                }
                                                
                                                // Extract the function name from the call
                                                let called_func = self.extract_function_name_from_call(call);
                                                if !called_func.is_empty() && !seen_calls.contains(&called_func) {
                                                    seen_calls.insert(called_func.clone());
                                                    internal_calls.push(called_func.clone());
                                                    
                                                    // Recursively analyze this function if it's in the same library
                                                    let is_internal = call.contains("self.") || 
                                                                      call.starts_with(&format!("{}.", library)) ||
                                                                      (!call.contains('.') && !call.starts_with("print") && !call.starts_with("len") && !call.starts_with("dict") && !call.starts_with("list") && !call.starts_with("str") && !call.starts_with("int"));
                                                    
                                                    // Also check for method calls on objects (e.g., session.request, obj.method)
                                                    let is_method_call = call.contains('.') && !call.starts_with("print") && !call.starts_with("len");
                                                    
                                                    if is_internal || is_method_call {
                                                        let nested_call = if call.contains("self.") {
                                                            // self.method() -> find method in the same class
                                                            format!("{}.{}", library, called_func)
                                                        } else if call.contains('.') && !call.starts_with(&format!("{}.", library)) {
                                                            // Method call like session.request() or obj.method()
                                                            // Extract the method name and search for it in the library
                                                            let method_name = called_func.clone();
                                                            format!("{}.{}", library, method_name)
                                                        } else if call.contains('.') {
                                                            call.clone()
                                                        } else {
                                                            format!("{}.{}", library, called_func)
                                                        };
                                                        
                                                        if let Ok((nested_internal, nested_expressions, nested_certs, nested)) = self.analyze_library_function_recursive(
                                                            library,
                                                            &nested_call,
                                                            library_path,
                                                            depth + 1,
                                                        ) {
                                                            // Collect certificates from nested chains
                                                            for cert in &nested_certs {
                                                                if !certificates.contains(cert) {
                                                                    certificates.push(cert.clone());
                                                                }
                                                            }
                                                            
                                                            if !nested_internal.is_empty() || !nested_expressions.is_empty() || !nested_certs.is_empty() || !nested.is_empty() {
                                                                nested_chains.push(DeepCallChain {
                                                                    external_call: nested_call.clone(),
                                                                    library: library.to_string(),
                                                                    internal_calls: nested_internal,
                                                                    call_expressions: nested_expressions,
                                                                    certificates: nested_certs,
                                                                    source_available: true,
                                                                    source_file: Some(entry.path().to_string_lossy().to_string()),
                                                                    nested_chains: nested,
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        break; // Found the function, no need to search further
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Extract certificates from call expressions
        certificates = Self::extract_certificates_from_calls(&call_expressions);
        
        // Remove from analyzed set after processing (to allow re-analysis in different contexts)
        self.analyzed_functions.remove(&func_id);

        Ok((internal_calls, call_expressions, certificates, nested_chains))
    }

    /// Find the file containing a specific function in a library.
    fn find_function_file(
        &self,
        _library: &str,
        function_call: &str,
        library_path: &PathBuf,
    ) -> Option<PathBuf> {
        // Extract function name from call (e.g., "requests.get" -> "get")
        let function_name = if let Some(name) = function_call.split('.').nth(1) {
            name.split('(').next().unwrap_or(name).trim()
        } else {
            return None;
        };

        // For requests library, check api.py first (where get() and request() are defined)
        // Also check sessions.py for Session class methods like request()
        let preferred_files = vec!["api.py", "sessions.py", "__init__.py"];
        
        // Search for the function in library files
        if library_path.is_dir() {
            use jwalk::WalkDir;
            
            // First, try preferred files - check both module-level functions and class methods
            for preferred in &preferred_files {
                let preferred_path = library_path.join(preferred);
                if preferred_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&preferred_path) {
                        // Look for function definition at module level (not indented)
                        if self.has_function_definition(&content, function_name, true) {
                            return Some(preferred_path);
                        }
                        // Also check for class methods (indented functions)
                        if self.has_function_definition(&content, function_name, false) {
                            return Some(preferred_path);
                        }
                    }
                }
            }
            
            // Then search all files
            for entry in WalkDir::new(library_path).max_depth(3) {
                if let Ok(entry) = entry {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "py" {
                            // Skip if already checked
                            if preferred_files.iter().any(|f| entry.path().ends_with(f)) {
                                continue;
                            }
                            
                            // Quick check: read file and see if it contains the function
                            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                                // Look for function definition at module level OR as class method
                                if self.has_function_definition(&content, function_name, true) {
                                    return Some(entry.path().to_path_buf());
                                }
                                if self.has_function_definition(&content, function_name, false) {
                                    return Some(entry.path().to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Check if a file contains a function definition at module level or in a class.
    fn has_function_definition(&self, content: &str, function_name: &str, module_level: bool) -> bool {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_class = false;
        
        for (_i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Track if we're inside a class definition
            if trimmed.starts_with("class ") {
                in_class = true;
            }
            
            // Look for "def function_name" or "def function_name("
            if trimmed.starts_with(&format!("def {}", function_name)) ||
               trimmed.starts_with(&format!("def {}(", function_name)) {
                if module_level {
                    // Module-level function (not indented)
                    if line.starts_with("def ") {
                        return true;
                    }
                } else {
                    // Class method (indented with 4 spaces or tab) - only if we're in a class
                    if (line.starts_with("    def ") || line.starts_with("\tdef ")) && in_class {
                        return true;
                    }
                    // Also allow module-level if module_level is false (for fallback search)
                    if line.starts_with("def ") {
                        return true;
                    }
                }
            }
            
            // Reset class flag if we hit a blank line or another top-level definition
            if trimmed.is_empty() && in_class {
                // Check if next non-empty line is at module level
                if let Some(next_line) = lines.get(_i + 1) {
                    if !next_line.trim().is_empty() && !next_line.starts_with("    ") && !next_line.starts_with("\t") {
                        in_class = false;
                    }
                }
            }
        }
        false
    }

    /// Extract function calls from a Python file.
    fn extract_calls_from_file(
        &self,
        file_path: &PathBuf,
        target_function: &str,
    ) -> Result<Vec<String>, NTreeError> {
        use crate::core::read_file;
        use crate::language::detect_language_config;
        use tree_sitter::Parser;

        let content = read_file(file_path)?;
        let config = detect_language_config(file_path)?;

        let mut parser = Parser::new();
        parser.set_language(&config.language).map_err(|e| {
            NTreeError::ParseError(format!("Failed to set language: {:?}", e))
        })?;

        let tree = parser.parse(&content, None).ok_or_else(|| {
            NTreeError::ParseError("Failed to parse file".to_string())
        })?;

        let root_node = tree.root_node();
        let mut calls = Vec::new();

        // Find the target function definition
        self.find_function_and_extract_calls(&root_node, &content, target_function, &mut calls);

        Ok(calls)
    }

    /// Find a function definition and extract its internal calls.
    fn find_function_and_extract_calls(
        &self,
        node: &tree_sitter::Node,
        source: &str,
        target_function: &str,
        calls: &mut Vec<String>,
    ) {
        if node.kind() == "function_definition" {
            // Check if this is the target function
            let mut cursor = node.walk();
            let mut found_function = false;
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    let func_name = source[child.start_byte()..child.end_byte()].trim();
                    if func_name == target_function {
                        found_function = true;
                        break;
                    }
                }
            }
            
            if found_function {
                // Find the function body (block node) - only extract from the body, not all children
                let mut body_cursor = node.walk();
                for child in node.children(&mut body_cursor) {
                    if child.kind() == "block" {
                        // Extract calls from this function's body only
                        self.extract_calls_from_node(&child, source, calls);
                        return; // Found the body, we're done
                    }
                }
                // Fallback: if no block found, check for expression_statement (single-line functions)
                let mut expr_cursor = node.walk();
                for child in node.children(&mut expr_cursor) {
                    if child.kind() == "expression_statement" {
                        self.extract_calls_from_node(&child, source, calls);
                        return;
                    }
                }
                return;
            }
        }

        // Recursively search children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.find_function_and_extract_calls(&child, source, target_function, calls);
        }
    }

    /// Extract call expressions from a node.
    fn extract_calls_from_node(
        &self,
        node: &tree_sitter::Node,
        source: &str,
        calls: &mut Vec<String>,
    ) {
        // Handle different call patterns
        match node.kind() {
            "call" => {
                // Extract the call expression
                let call_text = source[node.start_byte()..node.end_byte()].trim().to_string();
                if !call_text.is_empty() && !call_text.starts_with("print") && !call_text.starts_with("len") {
                    // Extract just the function being called
                    let func_name = self.extract_function_name_from_call(&call_text);
                    if !func_name.is_empty() {
                        calls.push(call_text);
                    }
                }
            }
            "attribute" => {
                // Handle attribute access that might be part of a call
                let _attr_text = source[node.start_byte()..node.end_byte()].trim().to_string();
                // Check if parent is a call
                if let Some(parent) = node.parent() {
                    if parent.kind() == "call" {
                        let call_text = source[parent.start_byte()..parent.end_byte()].trim().to_string();
                        if !call_text.is_empty() && !call_text.starts_with("print") {
                            calls.push(call_text);
                        }
                    }
                }
            }
            _ => {}
        }

        // Recursively visit children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_calls_from_node(&child, source, calls);
        }
    }

    /// Extract function name from a call expression.
    fn extract_function_name_from_call(&self, call_expr: &str) -> String {
        // Handle patterns like:
        // "self._find_no_duplicates(name, domain, path)" -> "_find_no_duplicates"
        // "session.request(method=method, url=url, **kwargs)" -> "request"
        // "requests.get(url)" -> "get"
        
        // Remove leading "self." or similar
        let cleaned = call_expr.trim_start_matches("self.").trim();
        
        // Find the function name (everything before the first '(')
        if let Some(paren_pos) = cleaned.find('(') {
            let before_paren = &cleaned[..paren_pos].trim();
            // Get the last part after any dots (for method calls)
            if let Some(dot_pos) = before_paren.rfind('.') {
                before_paren[dot_pos + 1..].to_string()
            } else {
                before_paren.to_string()
            }
        } else {
            cleaned.to_string()
        }
    }

    /// Get all deep call chains.
    pub fn get_call_chains(&self) -> &[DeepCallChain] {
        &self.call_chains
    }

    /// Get call chain for a specific external call.
    pub fn get_call_chain(&self, external_call: &str) -> Option<&DeepCallChain> {
        self.call_chains
            .iter()
            .find(|chain| chain.external_call == external_call)
    }
}

impl Default for DeepCallTracker {
    fn default() -> Self {
        Self::new()
    }
}

