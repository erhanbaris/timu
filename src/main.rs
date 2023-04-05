//use inkwell::context::Context;

use crate::parser::parser;

mod parser;
mod ast;
mod codegen;

fn main() {
    let code = "
@use(std);    
@use(std) as s;

func data(data: i32): test.test {

}
";
    let ast = parser(code);

    if let Err(error) = &ast {
        error.print(code);
    }

    print!("AST: {:#?}", &ast);

    // let compiler = CodeGen { };

    //compiler.compile(ast);
    //compiler.dump();
}
