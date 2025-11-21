# How Call Chains are Constructed in ntree

This document explains how the ntree library constructs call chains from source code analysis.

## Overview

The call chain construction process happens in two main phases:

1. **Initial Call Graph Construction** - Extracting call edges from workspace files
2. **Deep Call Chain Analysis** - Recursively analyzing external library calls

## Phase 1: Initial Call Graph Construction

### Step 1: Workspace Analysis

During workspace analysis (`unified_analysis.rs`), the system:

1. Iterates through all files in the workspace
2. For each Python file:
   - Parses the file using tree-sitter
   - Finds all function definitions
   - Extracts call sites from each function's body

### Step 2: Call Site Extraction

For each function, the `PythonCallExtractor` extracts call sites by:

1. Recursively visiting AST nodes looking for `"call"` nodes
2. For each call expression:
   - Extracting the call text (e.g., `"requests.get(url)"`)
   - Creating a `CallEdge` with:
     - `caller_sym`: The calling function's symbol ID
     - `callee_expr_text`: The full call expression text
     - `site_span`: Location information
     - `confidence`: Set to `Dynamic` for Python (since Python calls are dynamic)

### Step 3: Building the Call Graph

Each extracted `CallEdge` is added to the `CallGraph` structure. The `CallGraph` is a `HashMap<SymbolId, Vec<CallEdge>>` mapping each caller function to all its call sites.

## Phase 2: Deep Call Chain Analysis

### Step 4: External Call Detection

The `DeepCallTracker` analyzes external library calls:

1. Iterates through all call edges from the call graph
2. Identifies external library calls by:
   - Extracting library names from call expressions (e.g., `"requests.get"` → `"requests"`)
   - Skipping built-in functions like `print`, `len`, etc.
3. For each external call:
   - Attempts to find the library source code
   - Recursively analyzes the library function

### Step 5: Recursive Function Analysis

The recursive analysis process:

1. **Finds the function definition** in the library source:
   - Searches for the function file (e.g., `api.py` for `requests.get`)
   - Handles both module-level functions and class methods

2. **Extracts internal calls** from the function:
   - Parses the function body using tree-sitter
   - Extracts all call expressions within the function
   - Identifies which calls are internal to the library vs external

3. **Recursively processes internal calls**:
   - For each internal call, recursively analyzes that function
   - Tracks depth to prevent infinite recursion (default max depth: 5)
   - Uses `analyzed_functions` HashSet to avoid cycles

4. **Builds nested call chains**:
   - Creates `DeepCallChain` structures with:
     - External call name
     - Library name
     - Internal function calls
     - Full call expressions
     - Nested chains (recursive structure)

### Step 6: Certificate Extraction

During analysis, the system also extracts certificate-related information:
- Searches for certificate file paths in call expressions
- Finds certificate bundles from libraries like `certifi`
- Tracks SSL/TLS certificate usage throughout the call chain

## Data Structures

### CallEdge

Represents a single call site in the code:

```rust
pub struct CallEdge {
    /// Symbol ID of the calling function
    pub caller_sym: SymbolId,
    /// Location of the call site
    pub site_span: String,
    /// Original call expression text
    pub callee_expr_text: String,
    /// Resolved target symbols (empty if unresolved)
    pub targets: Vec<SymbolId>,
    /// Resolution confidence
    pub confidence: CallConfidence,
    /// Type of call
    pub call_type: CallType,
    /// Module hints for dynamic calls
    pub module_hints: Vec<String>,
}
```

### DeepCallChain

Represents a complete call chain for an external library function:

```rust
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
```

### CallGraph

The main data structure storing all call relationships:

```rust
pub struct CallGraph {
    /// caller_sym -> list of call edges
    call_edges: HashMap<SymbolId, Vec<CallEdge>>,
}
```

## Process Flow Summary

The call chain construction follows this flow:

1. **Parse workspace files** → Extract function definitions
2. **For each function** → Extract call sites using tree-sitter
3. **Build CallGraph** → Map callers to their call edges
4. **Identify external calls** → Detect library calls (e.g., `requests.get`)
5. **Find library source** → Locate library code in virtual environments
6. **Recursively analyze** → Extract internal calls from library functions
7. **Build nested chains** → Create hierarchical call chain structures

## Key Files

- `ntree/src/api/core/unified_analysis.rs` - Main analysis orchestration
- `ntree/src/storage/call_graph_table.rs` - CallGraph data structure
- `ntree/src/storage/call_edge.rs` - CallEdge data structure
- `ntree/src/api/analysis/deep_call_tracker.rs` - Deep call chain analysis
- `ntree/src/analyzers/language_specific/python/call_extractor.rs` - Python call extraction

## Result

This process produces:
- A complete graph of function calls within the workspace
- Deep call chains showing how external library calls propagate through library internals
- Certificate tracking for security analysis
- Nested hierarchical structures representing recursive call relationships

