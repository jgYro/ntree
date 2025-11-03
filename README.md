# ntree

A minimal Tree-sitter based library for parsing and analyzing Rust source code.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ntree = "0.1.0"
```

## API Usage

### Parse a Rust file

```rust
use ntree::create_tree_from_file;

let root_node = create_tree_from_file("src/main.rs")?;
println!("Root node kind: {}", root_node.kind());
```

### List top-level items

```rust
use ntree::{list_top_level_items, items_to_jsonl};

let items = list_top_level_items("src/lib.rs")?;
let jsonl = items_to_jsonl(&items)?;
println!("{}", jsonl);
```

Output:
```json
{"file":"src/lib.rs","kind":"use_declaration","identifier":null,"start_line":0,"start_column":0,"end_line":0,"end_column":30}
{"file":"src/lib.rs","kind":"function_item","identifier":"process_data","start_line":2,"start_column":0,"end_line":4,"end_column":1}
{"file":"src/lib.rs","kind":"struct_item","identifier":"DataProcessor","start_line":6,"start_column":0,"end_line":8,"end_column":1}
```

### Extract functions with body spans

```rust
use ntree::{list_functions, functions_to_jsonl};

let functions = list_functions("src/main.rs")?;
let jsonl = functions_to_jsonl(&functions)?;
println!("{}", jsonl);
```

Output:
```json
{"function":"calculate","span":"2:1–6:2","body":"2:37–6:2"}
{"function":"check","span":"8:1–20:2","body":"8:35–20:2"}
```

## API Reference

### Core Functions

- `create_tree_from_file(path)` - Parse a file and return the root Tree-sitter node
- `list_top_level_items(path)` - Extract all top-level declarations from a file
- `list_functions(path)` - Extract all functions with their spans and body spans

### Serialization

- `items_to_jsonl(items)` - Convert top-level items to JSONL format
- `functions_to_jsonl(functions)` - Convert function spans to JSONL format

### Types

- `NTreeError` - Error type for all operations
- `TopLevelItem` - Represents a top-level declaration with position information
- `FunctionSpan` - Represents a function with full span and body span

## License

MIT