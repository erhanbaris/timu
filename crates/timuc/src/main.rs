use std::process::exit;

use libtimu::{error::{CodeSpanReportGenerator, ReportGenerator}, file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};

fn main() -> Result<(), TirError> {
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Debug)
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Off)
        .set_thread_level(LevelFilter::Off)
        .build();
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, config, TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();

    let state1 = State::new(SourceFile::new(vec!["lib".into()], r#"
    interface ITest {
        func test(a: string): string;
        a: main.TestClass;
    }
    func abc(a:string): string {
    }

    "#.to_string()));


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

    let ast1 = match process_code(&state1) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };

    let ast2 = match process_code(&state2) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };

    match process_ast(vec![ast1.into(), ast2.into()]) {
        Ok(ast) => ast,
        Err(error) => {
            CodeSpanReportGenerator::generate(error);
            exit(1);
        }
    };
    Ok(())
}
