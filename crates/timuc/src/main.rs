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
/// Processes Timu source files from command line arguments and demonstrates the
/// complete compilation pipeline from source code to type-checked TIR.
///
/// # Process Flow
///
/// 1. **Logging Setup**: Configure error-level logging with color output
/// 2. **Argument Processing**: Read source files from command line arguments
/// 3. **Parsing**: Convert source code to ASTs for all modules
/// 4. **Type Checking**: Build TIR with cross-module type resolution
/// 5. **Error Handling**: Display rich diagnostics for any compilation errors
///
/// # Error Handling
///
/// The compiler exits with status code 1 if any compilation errors occur.
/// All errors are displayed with detailed source location information.
///
/// # Usage
///
/// ```
/// timuc file1.tim file2.tim ...
/// ```
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

    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} file1.tim file2.tim ...", args[0]);
        exit(1);
    }

    let mut asts = Vec::new();
    let mut states = Vec::new();

    // Process each source file
    for file_path in &args[1..] {
        // Read the file content
        let content = match std::fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Error reading file {file_path}: {error}");
                exit(1);
            }
        };

        // Extract module name from file path (remove .tim extension)
        let module_name = file_path.strip_suffix(".tim")
            .or_else(|| file_path.strip_suffix("/"))
            .unwrap_or(file_path)
            .split('/')
            .next_back()
            .unwrap_or(file_path)
            .to_string();

        // Create state and parse the file
        let state = State::new(SourceFile::new(vec![module_name], content));
        states.push(state);
    }

    // Parse all states into ASTs
    for state in &states {
        let ast = match process_code(state) {
            Ok(ast) => ast,
            Err(error) => {
                CodeSpanReportGenerator::generate(error);
                exit(1);
            }
        };

        asts.push(ast.into());
    }

    // Perform type checking on all modules together
    match process_ast(asts) {
        Ok(_tir_context) => {
            println!("Compilation successful!");
        },
        Err(error) => {
            // Type checking failed - display diagnostic information
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };
    
    Ok(())
}
