# Timu Programming Language Compiler

[![codecov](https://codecov.io/gh/erhanbaris/timu/graph/badge.svg?token=Q8RBT56K07)](https://codecov.io/gh/erhanbaris/timu)

A modern programming language designed for compile-time problem solving with nullable types, object-oriented features, and interface-based extensions.

## 🚧 Project Status

**Early Development Stage** - The Timu compiler is currently in active development. Core parsing and type checking are implemented, but many language features are still being developed.

### ✅ **Currently Implemented**
- **Parser & AST**: Complete parsing infrastructure using nom combinators
- **Type System**: Two-phase resolution with comprehensive type checking
- **Classes & Interfaces**: Object-oriented programming with inheritance
- **Module System**: Import/export functionality with qualified names
- **Error Reporting**: Rich error messages with source location information
- **Nullable Types**: Compile-time null safety with `?Type` syntax

### 🚧 **In Development**
- Code generation backend
- Control flow statements (loops, match expressions)
- Macro system (`@` symbols)
- Standard library

### 📋 **Planned Features**
- Array/list types and operations
- Generic type system
- Memory management with compile-time guarantees
- Concurrent programming features
- Advanced macro system

## 🏗️ **Architecture**

The Timu compiler consists of several key components:

```
Source Code → Parser → AST → TIR (Type Resolution) → [Code Generation - TBD]
```

### **Core Components**
- **`libtimu`**: Core compiler library with parser, AST, and TIR
- **`libtimu-macros`**: Procedural macros for error handling
- **`timuc`**: Main compiler executable

## 🚀 **Getting Started**

### **Prerequisites**
- Rust 1.70+ with Cargo
- LLVM (for future code generation)

### **Building**
```bash
# Clone the repository
git clone https://github.com/erhanbaris/timu
cd timu

# Build the compiler
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### **Running the Compiler**
```bash
# Current state - runs type checking on example code
cargo run

# Note: Code generation is not yet implemented
```

## 📖 **Language Features**

### **Basic Syntax**

#### **Classes and Objects**
```timu
class Person {
    name: string;
    age: i32;
    
    func init(this, name: string, age: i32): Person {
        this.name = name;
        this.age = age;
    }
    
    func greet(this): string {
        // Function implementation
    }
}
```

#### **Interfaces and Extensions**
```timu
interface Greeter {
    func greet(): string;
}

extend Person: Greeter {
    func greet(): string {
        // Implementation of interface requirement
    }
}
```

#### **Nullable Types**
```timu
class Example {
    func process(this, data: ?string): string {
        // Compiler enforces null checking
        // Implementation here
    }
}
```

#### **Module System**
```timu
// In lib.tim
interface IProcessor {
    func process(data: string): string;
}

func utility_function(input: string): string {
    // Implementation
}

// In main.tim
use lib.IProcessor;
use lib.utility_function;

class DataProcessor {
    func init(this): string {
        utility_function("test");
    }
}

extend DataProcessor: IProcessor {
    func process(data: string): string {
        // Implementation
    }
}
```

### **Type System**

#### **Primitive Types**
- **Integers**: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`
- **Floating Point**: `float` (single precision), `double` (double precision)
- **Other**: `bool`, `string`, `void`

#### **Nullable Types**
Any type can be made nullable with the `?` prefix:
```timu
var maybe_text: ?string;  // Can be string or null
var definitely_number: i32;  // Cannot be null
```

#### **Reference Types**
```timu
func example(data: ref string): void {
    // data is passed by reference
}
```

## 🧪 **Testing**

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test parser::  # Parser tests
cargo test tir::     # Type resolution tests

# Run with output
cargo test -- --nocapture

# Generate test coverage
grcov . --binary-path ./target/debug/deps/ --source-dir . \
  --excl-start 'mod test* \{' --ignore '*test*' --ignore "*test.rs" \
  --ignore "*main.rs" --ignore "*tests.rs" --ignore "*github.com*" \
  --ignore "*libcore*" --ignore "*rustc*" --ignore "*liballoc*" \
  --ignore "*cargo*" -t html -o ./coverage
```

## 📚 **Documentation**

Comprehensive documentation is available for all components:

```bash
# Generate and open documentation
cargo doc --open

# View specific module documentation
cargo doc --open --package libtimu
```

### **Internal Documentation**

- **[Error Handling](internals/error_handling.md)**: Comprehensive guide to using the TimuError macro system for rich error reporting

## 🔧 **Development**

### **Project Structure**
```
timu/
├── crates/
│   ├── libtimu/           # Core compiler library
│   │   ├── src/
│   │   │   ├── parser/    # Parsing logic
│   │   │   ├── ast/       # Abstract syntax tree
│   │   │   ├── tir/       # Type intermediate representation
│   │   │   └── error/     # Error handling
│   │   └── tests/         # Integration tests
│   ├── libtimu-macros/    # Procedural macros
│   ├── libtimu-macros-core/ # Macro core utilities
│   └── timuc/             # Compiler executable
├── CLAUDE.md              # Development notes
└── README.md              # This file
```

### **Key Commands**
```bash
# Development workflow
cargo check              # Quick syntax check
cargo test               # Run test suite
cargo clippy             # Linting
cargo fmt                # Code formatting
cargo doc                # Generate documentation
```

### **Known Issues**
- Code generation backend not yet implemented
- Some language features from original design not yet available
- LLVM integration temporarily disabled

## 🤝 **Contributing**

Contributions are welcome! The project is in active development and there are many opportunities to contribute:

1. **Language Features**: Implement missing constructs (loops, arrays, etc.)
2. **Code Generation**: Help build the backend compiler
3. **Standard Library**: Create built-in functions and types
4. **Testing**: Add more comprehensive test cases
5. **Documentation**: Improve user and developer documentation

### **Development Notes**
- The `CLAUDE.md` file contains detailed development guidance
- All major modules have comprehensive documentation
- The type system is the most mature part of the compiler
- Parser infrastructure is solid and extensible

## 📄 **License**

[License information to be added]

## 🔗 **Links**

- **Repository**: https://github.com/erhanbaris/timu
- **Issues**: https://github.com/erhanbaris/timu/issues
- **Documentation**: [Generated docs via cargo doc]

---

**Note**: This is an ambitious programming language project in early development. The foundations are solid with excellent parsing and type checking, but many advanced features are still being implemented. Contributions and feedback are highly valued!