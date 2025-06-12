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

let state = State::new(SourceFile::new(vec!["main".into()], r#"
func init(a: &?string): string {}

interface ITest {
    func hello(): string;
}


extend TestClass: ITest {
    func hello(): string { }
}

class TestClass {
    func call(this, a: &string): string {
        echo1(this, a);
        echo2(this);
    }
}

func echo1(a: ITest, b: string): string {
    echo2(a);
}

func echo2(a: ITest): string {
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
