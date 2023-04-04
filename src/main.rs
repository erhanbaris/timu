//use inkwell::context::Context;

use crate::parser::parser;

mod parser;
mod ast;
mod codegen;


fn main() {
    let code = "@use(std) as s;
fun test() {
    let a = 123;
    @print(\"merhaba dünya\" * 2);
    print(true);
}
";
    let ast = parser(code);

    if let Err(error) = ast {
        error.print(code);
    }

    // print!("AST: {:#?}", &ast);

    // let compiler = CodeGen { };

    //compiler.compile(ast);
    //compiler.dump();
}
