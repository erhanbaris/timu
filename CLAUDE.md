# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

### Testing
```bash
cargo test                    # Run all tests
cargo test -p libtimu --lib   # Run tests for specific crate
cargo test func_call_13       # Run specific test
RUST_BACKTRACE=full cargo test # Full backtrace for debugging
```

### Building
```bash
cargo build                   # Build all crates
cargo run                     # Run the timuc compiler
```

### Code Coverage
```bash
grcov . --binary-path ./target/debug/deps/ --source-dir . --excl-start 'mod test* \{' --ignore '*test*' --ignore "*test.rs" --ignore "*main.rs" --ignore "*tests.rs" --ignore "*github.com*" --ignore "*libcore*" --ignore "*rustc*" --ignore "*liballoc*" --ignore "*cargo*" -t html -o ./coverage
```

## Architecture Overview

This is the **timu programming language compiler** - a language designed to solve problems at compile time with nullable types, compiler macros (@symbols), and type inference.

### Workspace Structure
- **`libtimu`** - Core compiler library (AST, parser, TIR)
- **`libtimu-macros`** - Procedural macros for code generation  
- **`libtimu-macros-core`** - Core traits and utilities for macros
- **`timuc`** - Main compiler executable

### Compilation Pipeline
1. **Parser** (`/crates/libtimu/src/parser/`) - Uses `nom` combinators to parse `.tim` files into AST
2. **AST** (`/crates/libtimu/src/ast.rs`) - Abstract syntax tree representation
3. **TIR** (`/crates/libtimu/src/tir/`) - Type Intermediate Representation with two-phase resolution:
   - **Resolve phase**: Declare types and build signature tables
   - **Finish phase**: Validate implementations and cross-references

### Key TIR Components
- **`context.rs`** - Global state management (`TirContext`)
- **`resolver/`** - Multi-phase semantic analysis
- **`scope.rs`** - Hierarchical scope management for variables/types
- **`module.rs`** - File-based module system with imports
- **`signature.rs`** - Type signature lookup tables

### Language Features
- **Nullable types**: `?string` syntax with compile-time guarantees
- **Reference types**: `ref` keyword for references
- **Classes/Interfaces**: Object-oriented programming with extensions
- **Module system**: `use module.path as alias` imports
- **Compiler macros**: `@` symbols for compile-time operations

### Known Issues
- **Failing tests**: `func_call_13` and `func_call_14` in `function_call.rs` panic at `scope.rs:140` with "Found type location in module: TypeLocation(12)" - related to scope variable resolution
- **cargo r fails** - mentioned in README todo section

### Development Notes
- Uses `indexmap` for deterministic compilation order
- `codespan-reporting` provides rich error diagnostics
- Tests extensively cover parser and TIR functionality
- Location-based references (`TypeLocation`, `ScopeLocation`) enable efficient lookups