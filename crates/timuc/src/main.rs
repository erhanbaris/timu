use libtimu::{file::SourceFile, nom_tools::State, process_ast, process_code, tir::TirError};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger, TerminalMode, ThreadLogMode};
use libtimu_macros::TimuError;
use libtimu_macros_core::{traits::TimuErrorTrait, *};

#[derive(Clone, Debug, TimuError)]
#[diagnostic(code("code error222"), help("This is a help message"))]
pub struct ExtraFieldInExtend { 
    #[label("Already imported here")]
    pub position1: SourceSpan,
    
    #[label("asd")]
    pub position2: SourceSpan,

    #[source_code]
    pub source_code: String,
}

#[derive(Clone, Debug, thiserror::Error, TimuError)]
pub enum EnumTest { 
    #[error("Temporary error")]
    TemporaryError(ExtraFieldInExtend),

     
    #[error("Temporary error2")]
    TemporaryError2(ExtraFieldInExtend),
}

fn main() -> miette::Result<()> {
       let _test = ExtraFieldInExtend {
        position1: SourceSpan::new(SourceOffset(0), 10),
        position2: SourceSpan::new(SourceOffset(10), 10),
        source_code: "test".to_string(),
    };

    println!("{:#?}", TirError::extra_accessibility_identifier(0..25, SourceFile::new(vec!["test".to_string()], "merhaba dunya merhaba".to_string())).to_string());

    let ttt = EnumTest::TemporaryError(ExtraFieldInExtend { position1: (0..10).into(), position2: (0..20).into(), source_code: "asdasdasdasdadjnsdkjfnsdkjfnskdjfnsjkdnfsjdf".to_string() });

    println!("{:?}", ttt.labels());
    println!("{}", match ttt.source_code() { Some(code) => code.to_string(), None => "No source code".to_string() });
    println!("{}", ttt.error_code().unwrap());
    println!("{}", ttt.help().unwrap());
    println!("{:?}", ttt.to_string());

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
