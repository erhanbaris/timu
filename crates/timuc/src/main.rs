use std::{ops::Range, process::exit};

use libtimu::{error::{CodeSpanReportGenerator, ReportGenerator}, file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};
use libtimu_macros::TimuError;
use libtimu_macros_core::SourceCode;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};


#[derive(thiserror::Error, TimuError, Debug, Clone, PartialEq)]
#[error("{ty}")]
pub struct TypeWithSpan {
    /// Type to show
    pub ty: String,

    /// Span of expected type
    #[label("this has `{ty}`")]
    pub at: Range<usize>,

    // TODO: change to `Option<SourceFile>`
    /// Source code of the module, this type is located at
    #[source_code]
    pub source_code: SourceCode,
}

/// Diagnostic for not convertible types
#[derive(thiserror::Error, TimuError, Debug, Clone, PartialEq)]
#[error("expected `{expected}` type, got `{got}`")]
#[diagnostic(code("semantics::type_mismatch"))]
pub struct TypeMismatch {
    /// Expected type
    #[reference]
    pub expected: TypeWithSpan,
    /// Real type
    #[reference]
    pub got: TypeWithSpan,
}

fn main() -> Result<(), TirError> {
   
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Debug)
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Off)
        .set_thread_level(LevelFilter::Off)
        .build();
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, config, TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();

let state = State::new(SourceFile::new(vec!["source".into()], r#"
interface ITest {
    func test(a: string): string;
    a: TestClass;
}
 asd
extend TestClass: ITest {
    func test(a: string, b: string): string {
        
    }
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test("erhanbaris");
        this.a.test("baris");
        abc(abc("erhan"));
    }
}

func abc(a:string): string {
}
"#.to_string()));
    let ast = match process_code(&state) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };

    match process_ast(vec![ast.into()]) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };
    Ok(())
}
