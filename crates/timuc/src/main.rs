//! Timu Language Compiler Executable
//!
//! This is the main executable for the Timu programming language compiler.
//! It demonstrates the compilation pipeline by processing example Timu code
//! through parsing and type checking stages.
//!
//! # Current Implementation
//!
//! The current implementation serves as a test harness and demonstration of
//! the compiler's capabilities. It:
//!
//! 1. Sets up logging for error reporting
//! 2. Defines sample Timu code for two modules (`lib` and `main`)
//! 3. Parses both modules into ASTs
//! 4. Performs type checking and builds the TIR
//! 5. Reports any compilation errors with detailed diagnostics
//!
//! # Sample Code
//!
//! The compiler processes a multi-module example that demonstrates:
//! - Interface definitions
//! - Class implementations
//! - Function definitions
//! - Module imports and usage
//! - Cross-module type references
//!
//! # Error Reporting
//!
//! All compilation errors are displayed using rich diagnostic output
//! via the [`CodeSpanReportGenerator`] which provides:
//! - Source code location highlighting
//! - Error context and suggestions
//! - Color-coded terminal output
//!
//! # Future Development
//!
//! This executable will evolve to:
//! - Accept command-line arguments for input files
//! - Support various output formats
//! - Provide compilation options and flags
//! - Generate target code or bytecode

use std::process::exit;

use libtimu::{
    error::{CodeSpanReportGenerator, ReportGenerator}, 
    file::SourceFile, 
    nom_tools::State, 
    process_ast, 
    process_code, 
    tir::TirError
};
use log::LevelFilter;
use simplelog::{
    ColorChoice, 
    CombinedLogger, 
    ConfigBuilder, 
    LevelPadding, 
    TermLogger, 
    TerminalMode, 
    ThreadLogMode
};

/// Main entry point for the Timu compiler.
///
/// Sets up logging, processes sample Timu code, and demonstrates the
/// complete compilation pipeline from source code to type-checked TIR.
///
/// # Process Flow
///
/// 1. **Logging Setup**: Configure error-level logging with color output
/// 2. **Sample Code**: Create two modules with interface and class definitions
/// 3. **Parsing**: Convert source code to ASTs for both modules
/// 4. **Type Checking**: Build TIR with cross-module type resolution
/// 5. **Error Handling**: Display rich diagnostics for any compilation errors
///
/// # Error Handling
///
/// The compiler exits with status code 1 if any compilation errors occur.
/// All errors are displayed with detailed source location information.
///
/// # Returns
///
/// * `Ok(())` - Compilation completed successfully
/// * `Err(TirError)` - Should not occur as errors are handled internally
fn main() -> Result<(), TirError> {
    // Configure logging for error reporting
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Off)
        .set_thread_level(LevelFilter::Off)
        .build();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Error, 
        config, 
        TerminalMode::Mixed, 
        ColorChoice::Auto
    )]).unwrap();

    // Create the first module (lib) with interface and function definitions
    let state1 = State::new(SourceFile::new(vec!["lib".into()], r#"
    interface ITest {
        func test(a: string): string;
        a: main.TestClass;
    }
    func abc(a:string): string {
    }

    "#.to_string()));

    // Create the second module (main) that uses the lib module
    let state2 = State::new(SourceFile::new(vec!["main".into()], r#"
    use lib.ITest;

    extend TestClass: ITest {
        func test(a: string): string { }
        a: main.TestClass;
    }

    class TestClass {
        func init(this): string {
            lib.abc();
        }
    }

    "#.to_string()));

    // Parse the first module into an AST
    let ast1 = match process_code(&state1) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };

    // Parse the second module into an AST
    let ast2 = match process_code(&state2) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };

    // Perform type checking on both modules together
    match process_ast(vec![ast1.into(), ast2.into()]) {
        Ok(_tir_context) => {
            // Type checking succeeded - TIR context contains all type information
        },
        Err(error) => {
            // Type checking failed - display diagnostic information
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };
    
    Ok(())
}
