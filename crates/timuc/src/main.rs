use libtimu::{file::SourceFile, nom_tools::State, process_ast, process_code};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};

fn main() -> miette::Result<()> {
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Debug)
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Off)
        .set_thread_level(LevelFilter::Off)
        .build();
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Debug, config, TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();

    let state_1 = State::new(SourceFile::new(vec!["source".into()], " class testclass {} ".to_string()));
    let state_2 = State::new(SourceFile::new(vec!["lib".into()], "use source.testclass; func abc(a: testclass): source.testclass { } func abc(a: testclass): source.testclass { }".to_string()));

    let ast_1 = process_code(&state_1)?;
    let ast_2 = process_code(&state_2)?;
   
    process_ast(vec![ast_1.into(), ast_2.into()])?;
    Ok(())
}
