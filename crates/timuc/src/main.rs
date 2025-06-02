use libtimu::{file::SourceFile, nom_tools::State, process_ast, process_code};
use libtimu_macros::TimuError;
use libtimu_macros_core::{SourceOffset, SourceSpan};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};
use libtimu_macros_core::traits::TimuErrorTrait;

#[derive(Clone, Debug, TimuError)]
#[source_code]
pub struct ExtraFieldInExtend { 
    #[label(message="Already imported here")]
    pub position1: SourceSpan,
    
    #[label(message = "asd")]
    pub position2: SourceSpan,
}

fn main() -> miette::Result<()> {
    let test = ExtraFieldInExtend {
        position1: SourceSpan::new(SourceOffset(0), 10),
        position2: SourceSpan::new(SourceOffset(10), 10),
    };

    println!("{:?}", test.labels());

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

extend TestClass: ITest {
    func test(a: string): string {
        
    }
    a: string;
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
    let ast = process_code(&state)?;
   
    process_ast(vec![ast.into()])?;
    Ok(())
}
