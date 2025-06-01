use libtimu::{file::SourceFile, nom_tools::State, process_ast, process_code};
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

    let code = r#"
interface ITest {
    func test(a: string): string;
    a: TestClass;
}

class TestClass {
    func init(this): string {
        this.test("erhan");
        this.test("baris");
        this.test("timucin");
        this.a.test("baris");
        abc();
    }
}

extend TestClass: ITest {
    func test(a: string): string {
        
    }
    a: TestClass;
}

func abc(): TestClass {
}
    "#;
    let file = SourceFile::new(vec!["main".into()], code.to_string());
    let state = State {
        file,
        indexer: Default::default(),
    };

    let source = process_code(&state)?;
    process_ast(vec![source.into()])?;
    Ok(())
}
