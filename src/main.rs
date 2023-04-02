use inkwell::context::Context;

use crate::{parser::parser, codegen::CodeGen};

mod parser;
mod ast;
mod codegen;


fn main() {
    let ast = parser("fun test() {
    @print(\"merhaba dünya\");
}
");

    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("__general__");
    let compiler = CodeGen {
        context: &context,
        builder,
        module
    };

    compiler.compile(ast);
    compiler.dump();
}
