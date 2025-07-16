# TimuError Usage Documentation

This document provides comprehensive documentation on using the `TimuError` procedural macro for creating rich, user-friendly error types in the Timu compiler. The `TimuError` macro integrates with the `miette` diagnostic framework to provide detailed error reporting with source code context, helpful suggestions, and precise location tracking.

## Table of Contents

1. [Overview](#overview)
2. [Basic Usage](#basic-usage)
3. [Core Attributes](#core-attributes)
4. [Label Types](#label-types)
5. [Source Code Integration](#source-code-integration)
6. [Reference Errors](#reference-errors)
7. [Error Collections](#error-collections)
8. [Best Practices](#best-practices)
9. [Advanced Examples](#advanced-examples)

## Overview

The `TimuError` macro automatically implements the `miette::Diagnostic` trait for your error structs, enabling rich diagnostic reporting with:

- **Source code snippets** with highlighted error locations
- **Multiple error labels** pointing to different code locations
- **Helpful suggestions** and fix recommendations
- **Error codes** for programmatic error handling
- **Multi-file error reporting** for cross-module issues

## Basic Usage

### Simple Error with Single Location

The most basic error type points to a single location in source code:

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("'{type_name}' type not found")]
#[diagnostic(code("timu::error::type_not_found"))]
pub struct TypeNotFound {
    /// The name of the missing type
    pub type_name: String,

    /// Source location of the type reference
    #[label("type is not imported or defined in the current file")]
    pub position: Range<usize>,
    
    /// Source code context for the error
    #[source_code]
    pub code: SourceCode,

    /// Dynamic help message with suggestions
    #[help]
    pub advice: String,
}
```

**Key Components:**
- `#[error(...)]`: The main error message template
- `#[diagnostic(code(...))]`: Unique error code for tooling
- `#[label(...)]`: Message shown at the error location
- `#[source_code]`: Source code context for display
- `#[help]`: Dynamic help text with suggestions
- `#[reference]`: Reference to another error struct for composed errors

### Usage Example:

```rust
// Create a type not found error
let error = TypeNotFound {
    type_name: "UnknownClass".to_string(),
    position: 15..25,  // Character range in source
    code: source_file.into(),
    advice: "Try importing the type or check for typos".to_string(),
};
```

**Output:**
```
error: 'UnknownClass' type not found
  ┌─ example.tim:3:5
  │
3 │     func process(data: UnknownClass) {}
  │                        ^^^^^^^^^^^ type is not imported or defined in the current file
  │
  = help: Try importing the type or check for typos
```

## Core Attributes

### `#[error(...)]` - Main Error Message

Defines the primary error message. Supports field interpolation:

```rust
#[error("Function `{function_name}` expects {expected_size} argument{expected_plural}, but {got_size} {got_plural} provided")]
pub struct FunctionCallArgumentCountMismatch {
    pub function_name: String,
    pub expected_size: usize,
    pub got_size: usize,
    pub expected_plural: String,
    pub got_plural: String,
    // ... other fields
}
```

### `#[diagnostic(...)]` - Error Metadata

Provides error classification and help:

```rust
#[diagnostic(
    code("timu::error::accessibility_violation"), 
    help("mark the item as 'pub' in its definition module or remove the import")
)]
```

**Options:**
- `code(...)`: Unique error identifier for tooling
- `help(...)`: Static help message
- `severity(...)`: Error severity level

### `#[label(...)]` - Error Location Markers

Points to specific locations in source code:

```rust
pub struct AlreadyDefined {
    /// Location of the original definition
    #[label("Already defined here")]
    pub old_position: Range<usize>,

    /// Location of the duplicate definition
    #[label("But it is defined again here")]
    pub new_position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}
```

**Output:**
```
error: Already defined
  ┌─ example.tim:5:1
  │
5 │ class Person {}
  │ ^^^^^^^^^^^^^^^ Already defined here
6 │ class Person {}
  │ ^^^^^^^^^^^^^^^ But it is defined again here
```

## Label Types

### Basic Labels

Simple labels point to a single location:

```rust
#[label("private item cannot be imported")]
pub item_position: Range<usize>,
```

### Label Collections

For multiple related locations:

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Invalid type")]
pub struct InvalidType {
    #[label(collection, "")]
    pub position: Vec<LabeledSpan>,
    
    #[source_code]
    pub code: SourceCode,
}
```

**Usage:**
```rust
let error = InvalidType {
    position: vec![
        LabeledSpan::new("Missing type annotation".to_string(), 10..15),
        LabeledSpan::new("Invalid syntax here".to_string(), 20..25),
    ],
    code: source_file.into(),
};
```

### Dynamic Label Messages

Label messages can use field interpolation:

```rust
#[label("expected {expected_type}, got {actual_type}")]
pub position: Range<usize>,
pub expected_type: String,
pub actual_type: String,
```

## Source Code Integration

### Single Source File

Most errors reference a single source file:

```rust
#[source_code]
pub code: SourceCode,
```

### Multiple Source Files

For cross-module errors, use multiple `#[source_code]` fields:

```rust
pub struct AccessibilityViolation {
    pub item_name: String,
    
    /// Information about the private item definition
    #[reference]
    pub item_info: PrivateItemInfo,
    
    /// Information about the import attempt
    #[reference]
    pub import_info: ImportAttemptInfo,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("private item")]
pub struct PrivateItemInfo {
    #[label("private item cannot be imported")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("import attempt")]
pub struct ImportAttemptInfo {
    #[label("trying to import private item")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}
```

**Output:**
```
error: 'PrivateClass' is private and cannot be imported
  ┌─ main.tim:3:5
  │
3 │ use lib.PrivateClass;
  │         ^^^^^^^^^^^^ trying to import private item
  │
  ┌─ lib.tim:5:1
  │
5 │ class PrivateClass {}
  │ ^^^^^^^^^^^^^^^^^^^^ private item cannot be imported
  │
  = help: mark the item as 'pub' in its definition module or remove the import
```

## Reference Errors

### `#[reference]` Attribute

For composed errors with multiple sub-errors, use `#[reference]`:

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
pub struct FunctionCallArgumentCountMismatch {
    pub function_name: String,
    pub expected_size: usize,
    pub got_size: usize,

    #[reference]
    pub expected: TypeWithSpan,

    #[reference]
    pub got: TypeWithSpan,
}

#[derive(thiserror::Error, TimuError, Debug, Clone)]
#[error("{ty}")]
pub struct TypeWithSpan {
    pub ty: String,

    #[label("{ty}")]
    pub at: Range<usize>,

    #[source_code]
    pub source_code: SourceCode,
}
```

This creates a hierarchical error structure where each referenced error contributes its own labels and source code.

## Error Collections

### Multiple Errors

For collecting multiple related errors:

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("ooops, multiple errors detected")]
pub struct ErrorCollection {
    #[errors]
    pub errors: Vec<TirError>
}
```

### Syntax Errors

Special case for parser errors:

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("{} syntax error(s) detected", .errors.len())]
pub struct SyntaxError {
    #[errors]
    pub errors: Vec<SyntaxErrorItem>
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Syntax error")]
pub struct SyntaxErrorItem {
    #[label("{message}")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,

    pub message: &'static str,
}
```

## Best Practices

### 1. Clear Error Messages

Use descriptive, user-friendly error messages:

```rust
// Good
#[error("'{item_name}' is private and cannot be imported")]

// Bad
#[error("Access violation")]
```

### 2. Helpful Suggestions

Always provide actionable help:

```rust
#[diagnostic(help("mark the item as 'pub' in its definition module or remove the import"))]
```

### 3. Precise Location Tracking

Use specific ranges that highlight the exact problem:

```rust
// Point to the exact identifier, not the whole statement
pub position: Range<usize>,  // Should cover "UnknownType", not "func process(data: UnknownType) {}"
```

### 4. Consistent Error Codes

Use hierarchical error codes:

```rust
#[diagnostic(code("timu::error::type_not_found"))]
#[diagnostic(code("timu::error::accessibility_violation"))]
#[diagnostic(code("timu::error::function_call_argument_count_mismatch"))]
```

### 5. Rich Context

Include relevant context information:

```rust
pub struct TypeNotFound {
    pub type_name: String,     // What was being looked for
    pub position: Range<usize>, // Where the error occurred
    pub code: SourceCode,      // Source context
    pub advice: String,        // Specific suggestions
}
```

## Advanced Examples

### Cross-Module Error with Multiple References

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Type mismatch in assignment")]
#[diagnostic(
    code("timu::error::type_mismatch"),
    help("ensure the assigned value matches the variable type")
)]
pub struct TypeMismatch {
    #[reference]
    pub expected: TypeReference,
    
    #[reference]
    pub actual: TypeReference,
}

#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("expected type")]
pub struct TypeReference {
    pub type_name: String,
    
    #[label("expected {type_name}")]
    pub position: Range<usize>,
    
    #[source_code]
    pub code: SourceCode,
}
```

### Dynamic Error with Context-Aware Help

```rust
impl TirError {
    pub fn type_not_found(
        context: &TirContext, 
        missing_type_name: String, 
        position: Range<usize>, 
        source: SourceFile
    ) -> Self {
        // Find similar types for suggestions
        let mut similar_types = Vec::new();
        for (type_name, _) in context.types_scope.iter() {
            if type_name.ends_with(missing_type_name.as_str()) {
                similar_types.push(type_name.to_string());
            }
        }
        
        let advice = if !similar_types.is_empty() {
            let type_list = similar_types.iter()
                .map(|item| format!(" - {item}"))
                .collect::<Vec<_>>()
                .join("\n");
            format!("The following types are similar:\n{type_list}")
        } else {
            "try to import the type, or maybe you need to define it in the current file".to_string()
        };

        TirError::TypeNotFound(TypeNotFound {
            position,
            code: source.into(),
            type_name: missing_type_name,
            advice,
        }.into())
    }
}
```

### Error with Multiple Validation Points

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error)]
#[error("Interface implementation incomplete")]
#[diagnostic(
    code("timu::error::interface_implementation_incomplete"),
    help("implement all required interface methods and fields")
)]
pub struct InterfaceImplementationIncomplete {
    #[label("interface defined here")]
    pub interface_position: Range<usize>,
    
    #[label("implementation attempted here")]
    pub implementation_position: Range<usize>,
    
    #[label(collection, "missing implementation")]
    pub missing_items: Vec<LabeledSpan>,
    
    #[source_code]
    pub code: SourceCode,
}
```

## Integration with Error Enums

### Main Error Enum

Located in `/app/crates/libtimu/src/tir/error.rs`, lines 401-438:

```rust
#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum TirError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    TypeNotFound(Box<TypeNotFound>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    AccessibilityViolation(Box<AccessibilityViolation>),
    
    #[error(transparent)]
    #[diagnostic(transparent)]
    AlreadyDefined(Box<AlreadyDefined>),
    
    // ... other variants
}
```

### Constructor Methods

```rust
impl TirError {
    pub fn accessibility_violation(
        item_name: String, 
        import_position: Range<usize>, 
        import_source: SourceFile,
        item_position: Range<usize>,
        item_source: SourceFile
    ) -> Self {
        TirError::AccessibilityViolation(AccessibilityViolation {
            item_name,
            item_info: PrivateItemInfo {
                position: item_position,
                code: item_source.into(),
            },
            import_info: ImportAttemptInfo {
                position: import_position,
                code: import_source.into(),
            },
        }.into())
    }
}
```

This comprehensive system provides rich, user-friendly error reporting that helps developers quickly understand and fix issues in their Timu code. The combination of precise location tracking, helpful suggestions, and clear visual presentation makes debugging much more efficient.