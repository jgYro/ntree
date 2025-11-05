# ntree

A language-agnostic library for parsing and analyzing source code, featuring Control Flow Graph generation, cyclomatic complexity analysis, and a unified builder-pattern API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ntree = "0.1.0"
```

## Features

- **Unified API**: Single `SourceCode` entry point for files and workspaces
- **Workspace Analysis**: Analyze entire codebases with file discovery and caching
- **Cross-File Symbol Search**: Find functions, constructors, and patterns across projects
- **Language Agnostic**: Support for Rust, Python, JavaScript, TypeScript, Java, C, C++
- **Regex Pattern Matching**: Powerful symbol search with parameterized queries
- **Control Flow Graphs**: Generate CFGs with proper exception handling (try/catch)
- **Complexity Analysis**: Calculate cyclomatic complexity and detect unreachable code
- **Multiple Export Formats**: JSONL, Mermaid diagrams, structured data
- **Builder Pattern**: Fluent, discoverable API with method chaining

## Quick Start

### Simple Analysis

```rust
use ntree::SourceCode;

// Analyze everything with default settings
let analysis = SourceCode::new("src/main.rs")?.analyze()?;

// Access results
println!("Functions found: {}", analysis.functions().len());
println!("High complexity functions:");

for result in analysis.complexity().filter_by_complexity(5) {
    println!("  {}: complexity {}", result.function, result.cyclomatic);
    if !result.unreachable.is_empty() {
        println!("    Unreachable blocks: {:?}", result.unreachable);
    }
}
```

### Selective Analysis

```rust
use ntree::SourceCode;

// Enable only specific analyses
let analysis = SourceCode::new("src/lib.rs")?
    .with_complexity_analysis(true)
    .with_cfg_generation(true)
    .with_basic_blocks(false)
    .analyze()?;

// Export results
println!("{}", analysis.to_jsonl()?);
```

### Workspace Analysis

```rust
use ntree::SourceCode;

// Analyze entire workspace/directory
let analysis = SourceCode::new("src/")?
    .search_workspace(true)
    .with_complexity_analysis(true)
    .analyze()?;

// Get files by language
if let Some(files_by_lang) = analysis.files_by_language() {
    for (lang, files) in files_by_lang {
        println!("{}: {} files", lang, files.len());
    }
}

// Workspace statistics
if let Some(stats) = analysis.workspace_stats() {
    println!("Total files: {}", stats.total_files);
    println!("Languages: {}", stats.languages);
}
```

### Symbol Search (Workspace & Single File)

```rust
use ntree::SourceCode;

let analysis = SourceCode::new("src/")?.search_workspace(true).analyze()?;

// Find exact constructors (not new_iterator, new_panic, etc.)
let constructors = analysis.symbols()
    .named("new")
    .regex(true)
    .kind("function")
    .search()?;

// Find getter methods
let getters = analysis.symbols()
    .named("^get_\\w+")
    .regex(true)
    .search()?;

// Find functions in specific files
let cfg_functions = analysis.symbols()
    .in_file("cfg")
    .kind("function")
    .search()?;

// Find test functions across all languages
let tests = analysis.symbols()
    .named(".*test.*")
    .regex(true)
    .search()?;

println!("Found {} constructors, {} getters, {} tests",
    constructors.len(), getters.len(), tests.len());
```

### Advanced Single File Usage

```rust
use ntree::SourceCode;

let analysis = SourceCode::new("src/main.rs")?.analyze()?;

// Find specific functions
if let Some(main_fn) = analysis.functions().find_by_name("main") {
    println!("Found main function: {}", main_fn.span);
}

// Get CFG for specific function
if let Some(cfg) = analysis.cfgs().for_function("calculate") {
    println!("CFG Mermaid:\n{}", cfg.mermaid);
}

// Symbol search works on single files too
let local_constructors = analysis.symbols()
    .named("new")
    .regex(true)
    .search()?;
```

## Configuration Options

The `SourceCode` builder supports these configuration methods:

**Analysis Configuration:**
- `.with_complexity_analysis(bool)` - Enable/disable cyclomatic complexity analysis
- `.with_cfg_generation(bool)` - Enable/disable Control Flow Graph generation
- `.with_early_exit_analysis(bool)` - Enable/disable early exit pattern analysis
- `.with_loop_analysis(bool)` - Enable/disable loop structure analysis
- `.with_basic_blocks(bool)` - Enable/disable basic block generation

**Workspace Configuration:**
- `.search_workspace(bool)` - Enable/disable workspace-wide analysis

**Presets:**
- `.minimal()` - Only complexity and CFG analysis
- `.none()` - Disable all analyses (useful for custom configuration)

## Result Access

### Complexity Analysis

```rust
let complexity = analysis.complexity();

// Filter by complexity threshold
let high_complexity = complexity.filter_by_complexity(5);

// Filter by function name pattern
let test_functions = complexity.filter_by_name("test_");

// Find functions with unreachable code
let with_dead_code = complexity.with_unreachable_code();

// Export to JSONL
let jsonl = complexity.to_jsonl()?;
```

### Control Flow Graphs

```rust
let cfgs = analysis.cfgs();

// Get CFG for specific function
let main_cfg = cfgs.for_function("main");

// Export all CFGs to Mermaid
let mermaid_diagrams = cfgs.to_mermaid();

// Export to JSONL
let jsonl = cfgs.to_jsonl();
```

### Functions

```rust
let functions = analysis.functions();

// Filter by name pattern
let getters = functions.filter_by_name("get_");

// Get all function names
let names = functions.names();
```

### Symbol Search

```rust
// Basic symbol search
let symbols = analysis.symbols();

// Parameterized search with fluent API
let exact_constructors = symbols
    .named("new")
    .regex(true)           // Use regex matching
    .kind("function")      // Only functions
    .search()?;

// Language-agnostic constructor detection
let all_constructors = symbols
    .named("^(new|__init__|constructor)$")
    .regex(true)
    .search()?;

// Search in specific files
let api_functions = symbols
    .in_file("api")
    .kind("function")
    .search()?;

// Complex regex patterns
let test_functions = symbols
    .named("^(test_|.*_test|should_|it_).*")
    .regex(true)
    .search()?;
```

## Output Formats

### JSONL Format
Each analysis produces structured JSONL output:

**Complexity Analysis:**
```json
{"function":"calculate","cyclomatic":3,"unreachable":["N7","N9"]}
{"function":"process","cyclomatic":1,"unreachable":[]}
```

**CFG Nodes and Edges:**
```json
{"type":"CFGNode","func":"main","id":"N1","label":"ENTRY","span":"1:1-1:1"}
{"type":"CFGEdge","func":"main","from":"N1","to":"N2","kind":"next"}
```

### Mermaid Diagrams
CFGs export as Mermaid flowcharts:

```mermaid
graph TD
    N1["ENTRY"]
    N2["let x = 5;"]
    N3["if x > 0"]
    N4["println!(\"positive\")"]
    N5["EXIT"]
    N1 --> N2
    N2 --> N3
    N3 -->|true| N4
    N3 -->|false| N5
    N4 --> N5
```

## Complexity Analysis (CFG-13)

The complexity analyzer implements cyclomatic complexity calculation and unreachable code detection:

- **Formula**: `E - N + 2` (where E = edges, N = nodes)
- **Reachability**: DFS traversal from ENTRY node
- **Output**: Function name, complexity score, list of unreachable node IDs

Example for a function with complexity 3 and unreachable blocks:
```json
{"function":"check","cyclomatic":3,"unreachable":["N7","N9"]}
```

## API Reference

### Core API
- `SourceCode::new(path)` - Create analyzer for file or directory
- `SourceCode::analyze()` - Execute configured analyses
- `AnalysisResult` - Unified container for all analysis results

### Configuration Methods
- `.with_complexity_analysis(bool)` - Configure complexity analysis
- `.with_cfg_generation(bool)` - Configure CFG generation
- `.search_workspace(bool)` - Enable workspace-wide analysis
- `.minimal()` - Preset for essential analyses only

### Result Access Methods
- `.complexity()` - Access complexity analysis results
- `.cfgs()` - Access Control Flow Graph results
- `.functions()` - Access function information
- `.symbols()` - Access symbol search interface
- `.files_by_language()` - Access workspace file groupings (workspace mode)
- `.workspace_stats()` - Access workspace statistics (workspace mode)

### Symbol Search Methods (Parameterized)
- `.named(pattern)` - Set name pattern for search
- `.regex(bool)` - Enable/disable regex matching
- `.kind(type)` - Filter by symbol type (function, class, etc.)
- `.in_file(pattern)` - Filter by file path pattern
- `.search()` - Execute search and return results

### Data Types
- `AnalysisResult` - Unified analysis container
- `ComplexityResult` - Complexity analysis with cyclomatic complexity and unreachable blocks
- `CfgResult` - CFG with Mermaid and JSONL representations
- `TopLevelSymbol` - Cross-file symbol information
- `WorkspaceStats` - Statistics about workspace analysis

## Error Handling

All functions return `Result<T, NTreeError>`:

```rust
use ntree::{SourceCode, NTreeError};

match SourceCode::new("src/main.rs") {
    Ok(source) => {
        match source.analyze() {
            Ok(analysis) => {
                // Process results
            }
            Err(NTreeError::ParseError(msg)) => {
                eprintln!("Analysis failed: {}", msg);
            }
            Err(NTreeError::IoError(err)) => {
                eprintln!("File error: {}", err);
            }
        }
    }
    Err(e) => eprintln!("Setup error: {}", e),
}
```

## Language Support

**Fully Supported Languages:**
- **Rust** (.rs) - Complete feature support
- **Python** (.py) - Complete feature support
- **JavaScript** (.js, .mjs) - Complete feature support
- **TypeScript** (.ts) - Complete feature support
- **Java** (.java) - Function detection and basic analysis
- **C** (.c, .h) - Function detection and complexity analysis
- **C++** (.cpp, .cc, .cxx, .hpp, .hxx) - Function detection and complexity analysis

**Automatic Language Detection:**
ntree automatically detects the programming language based on file extension and uses the appropriate Tree-sitter parser.

```rust
// Works with any supported language
let rust_analysis = SourceCode::new("src/main.rs")?.analyze()?;
let python_analysis = SourceCode::new("script.py")?.analyze()?;
let js_analysis = SourceCode::new("app.js")?.analyze()?;
let ts_analysis = SourceCode::new("component.ts")?.analyze()?;
let java_analysis = SourceCode::new("Main.java")?.analyze()?;
let c_analysis = SourceCode::new("program.c")?.analyze()?;
let cpp_analysis = SourceCode::new("program.cpp")?.analyze()?;
```

## License

MIT