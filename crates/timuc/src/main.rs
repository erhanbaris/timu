use libtimu::{process_ast, process_code};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};


fn main() -> Result<(), ()> {
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Debug)
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Off)
        .set_thread_level(LevelFilter::Off)
        .build();
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, config, TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();
        let ast = process_code(vec!["source".into()], r#"
interface ITest { func test(): TestClass; a: TestClass; }
extend TestClass: ITest { func test(): TestClass { } a: TestClass; }
class TestClass { }
        "#)?;
    
    process_ast(vec![ast.into()])?;
    Ok(())
}
