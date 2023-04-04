//use inkwell::{context::Context, module::{Module, Linkage}, builder::Builder, types::BasicMetadataTypeEnum, AddressSpace};

use crate::ast::{FileAst, BlockAst, FunctionDefinitionAst, FunctionCallAst, PrimativeAst, StatementAst};

pub struct CodeGen {

}

impl CodeGen {
    pub fn dump(&self) {
        
    }

    pub fn compile(&self, file: Box<FileAst>) {
        file.generate(self);
    }

}

pub trait CompileTrait {
    fn generate(&self, compile: &CodeGen);
}

impl CompileTrait for PrimativeAst {
    fn generate(&self, compile: &CodeGen) {
        
    }
}

impl CompileTrait for FunctionCallAst {
    fn generate(&self, compile: &CodeGen) {
        
 
    }
}

impl CompileTrait for FunctionDefinitionAst {
    fn generate(&self, compile: &CodeGen) {
        self.body.generate(compile);
    }
}

impl CompileTrait for FileAst {
    fn generate(&self, compile: &CodeGen) {
        for function in self.functions.iter() {
            function.generate(compile);
        }
    }
}

impl CompileTrait for BlockAst {
    fn generate(&self, compile: &CodeGen) {
        for statement in self.statements.iter() {
            statement.generate(compile);
        }
    }
}


impl CompileTrait for StatementAst {
    fn generate(&self, compile: &CodeGen) {
        match self {
            StatementAst::FunctionCall(func_call) => func_call.generate(compile),
            StatementAst::Primative(primative) => primative.generate(compile)
        }
    }
}
