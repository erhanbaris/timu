//use inkwell::context::Context;

use crate::{parser::parser, codegen::CodeGen};

mod parser;
mod ast;
mod codegen;
mod vm;

fn main() {
    let code = "

var test = 123;

func main(a: i32, b: i16): i32 {

}
";
    let ast = parser(code);
    if let Err(error) = &ast {
        error.print(code);
    }

    //print!("AST: {:#?}", &ast);

    let compiler = CodeGen { };

    compiler.compile(ast.unwrap()).unwrap();
}
