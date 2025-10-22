# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Timu is a programming language compiler written in Rust. The goal is to design a language that can solve many problems at compile time, with strong guarantees about nullable types and comprehensive macro support.

## Build and Test Commands

### Build
```bash
cargo build
```

### Run the compiler
```bash
cargo run -p timuc
```

### Run all tests
```bash
cargo test
```

### Run tests for a specific crate
```bash
cargo test -p libtimu
```

### Run a specific test
```bash
cargo test <test_name>
```

### Format code (strict formatting rules apply)
```bash
cargo fmt
```

### Code coverage
```bash
grcov .  --binary-path ./target/debug/deps/ --source-dir . --excl-start 'mod test* \{' --ignore '*test*'  --ignore "*test.rs" --ignore "*main.rs" --ignore "*tests.rs" --ignore "*github.com*" --ignore "*libcore*" --ignore "*rustc*" --ignore "*liballoc*" --ignore "*cargo*" -t html  -o ./coverage
```

## Architecture

### Crate Structure

The project uses a Cargo workspace with 4 main crates:

- **libtimu**: Core compiler library containing the parser, AST, and TIR (Typed Intermediate Representation) system
- **timuc**: Compiler executable that uses libtimu
- **libtimu-macros**: Procedural macros for code generation
- **libtimu-macros-core**: Core utilities for the macro system

### Compilation Pipeline

The compiler follows a multi-phase approach:

1. **Parsing Phase** (`process_code`): Converts source text into AST (Abstract Syntax Tree)
   - Located in `crates/libtimu/src/parser/`
   - Uses nom parser combinators
   - Produces a `FileAst` for each source file

2. **TIR Building Phase** (`process_ast`): Type resolution and semantic analysis
   - Located in `crates/libtimu/src/tir/`
   - Converts AST into TIR (Typed Intermediate Representation)
   - Performs module resolution, type checking, and scope analysis
   - Produces a `TirContext` containing all resolved types and signatures

### Key Components

**AST Layer** (`crates/libtimu/src/ast.rs`):
- Defines the abstract syntax tree structure
- Main node types: `ClassDefinitionAst`, `FunctionDefinitionAst`, `InterfaceDefinitionAst`, `ExtendDefinitionAst`, `UseAst`
- Each file produces a `FileAst` containing multiple `FileStatementAst` nodes

**TIR Layer** (`crates/libtimu/src/tir/`):
- **Context** (`context.rs`): Central data structure holding all resolved types, modules, and signatures
- **Module System** (`module.rs`, `ast_signature.rs`): Manages hierarchical module structure and imports
- **Resolver** (`resolver/`): Converts AST nodes into typed representations
- **Signatures** (`signature.rs`): Generic signature system for tracking definitions
- **Scope** (`scope.rs`): Manages variable scoping and visibility

**Parser** (`crates/libtimu/src/parser/`):
- Modular parser with separate files for each language construct
- Key parsers: `class.rs`, `function_definition.rs`, `interface.rs`, `extend.rs`, `module_use.rs`, `expression.rs`
- Uses nom's `Span` for error reporting with source locations

### Type System

The TIR implements a sophisticated type system with:
- Primitive types (i8, u8, i16, u16, i32, u32, i64, u64, float, bool, string, void)
- Nullable types (denoted with `?` prefix)
- Class and interface types
- Module-scoped type resolution
- Reference and mutability tracking

### Module Resolution

Modules are resolved hierarchically:
- Each source file becomes a module (based on file path)
- `use` statements import types/functions from other modules
- Modules can be aliased with `as` keyword
- Full module paths like `module.submodule.Type` are supported

## Coding Conventions

### Rustfmt Configuration
The project uses custom rustfmt settings defined in `rustfmt.toml`:
- Max width: 160 characters
- Imports are reordered and derives are merged
- Function parameters use compressed layout

### Error Handling
- Parser errors use `TirError` enum with rich source location tracking
- Errors are collected and reported with codespan-reporting for user-friendly output
- The `CodeSpanReportGenerator` generates formatted error messages

### Testing
- Unit tests are colocated with source code in `mod tests` blocks
- Parser tests are in `crates/libtimu/src/tests/parser/`
- Test utilities use `State::new()` to create parser state from source strings
- Use `rstest` for parameterized tests and `pretty_assertions` for better test output

## Language Features

The Timu language supports:
- Classes with fields and methods
- Interfaces and interface implementation via `extend`
- Nullable types with compile-time guarantees
- Module system with `use` statements
- Function definitions with type annotations
- Expressions with operators and function calls
- Compiler macros (denoted with `@`)

## VSCode Extension

A basic VSCode extension for Timu syntax highlighting is available in `.vscode/extensions/timu-lang/`.
