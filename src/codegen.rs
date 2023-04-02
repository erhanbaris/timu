use inkwell::{context::Context, module::Module, builder::Builder};

use crate::ast::{FileAst, Ast};

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl CodeGen<'_> {
    pub fn dump(&self) {
        let _result = self.module.print_to_string();
        println!(">> {}", _result);
    }

    pub fn compile(&self, ast: Box<FileAst>) {
        ast.generate(self);
    }

}
