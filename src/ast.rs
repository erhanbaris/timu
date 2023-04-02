use inkwell::{types::BasicMetadataTypeEnum, module::Linkage, AddressSpace};

use crate::codegen::CodeGen;

pub trait Ast {
    fn generate<'ctx>(&self, compile: &CodeGen<'ctx>);
}

#[derive(Default, Debug)]
pub struct FuncArg {
    pub name: String,
    pub arg_type: String
}

#[derive(Default, Debug)]
pub enum AccessType {
    #[default]
    Public,
    Private
}

#[derive(Default, Debug)]
pub enum PrimativeAst {
    #[default]
    None,
    String(String),
    I32(i32)
}

#[derive(Default)]
pub struct FunctionCallAst {
    pub compiler: bool,
    pub name: String,
    pub args: Vec<Box<dyn Ast>>
}

#[derive(Default)]
pub struct FunctionDefinitionAst {
    pub access: AccessType,
    pub name: String,
    pub args: Vec<FuncArg>,
    pub return_type: String,
    pub body: Box<BlockAst>
}

#[derive(Default)]
pub struct BlockAst {
    pub statements: Vec<Box<dyn Ast>>
}

#[derive(Default)]
pub struct FileAst {
    pub functions: Vec<Box<FunctionDefinitionAst>>
}

impl Ast for PrimativeAst {
    fn generate<'ctx>(&self, compile: &CodeGen<'ctx>) {
        
    }
}

impl Ast for FunctionCallAst {
    fn generate<'ctx>(&self, compile: &CodeGen<'ctx>) {
        let f64_type = compile.context.f64_type();
        let str_type = compile
            .context
            .i8_type()
            .ptr_type(inkwell::AddressSpace::default());
        let printf_args_type = vec![BasicMetadataTypeEnum::PointerType(str_type)];

        // printf needs to return a double to be used in compile_call
        let printf_type = f64_type.fn_type(printf_args_type.as_slice(), true);

        let printf_fn = compile
            .module
            .add_function(&self.name, printf_type, Some(Linkage::External));
        
        let string = "Hello, World!\n\0";
        
        let ty = compile.context.i8_type().array_type(string.len() as u32);
        let gv = compile.module.add_global(ty, Some(AddressSpace::default()), "message");
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&compile.context.const_string(string.as_ref(), false));

        let pointer_value = compile.builder.build_pointer_cast(
            gv.as_pointer_value(),
            compile.context.i8_type().ptr_type(AddressSpace::default()),
            "message",
        );

        compile.builder.build_call(printf_fn, &[pointer_value.into()], "");
 
    }
}

impl Ast for FunctionDefinitionAst {
    fn generate<'ctx>(&self, compile: &CodeGen<'ctx>) {
        let return_type = match &self.return_type[..] {
            "_" => compile.context.void_type(),
            return_type => panic!("no support for other types yet")
        };

        let mut function_args = Vec::<BasicMetadataTypeEnum<'_>>::new();
        for arg in self.args.iter() {
            
        }

        let func = return_type.fn_type(&function_args, false);

        let function = compile.module.add_function(&self.name, func, None);
        let basic_block = compile.context.append_basic_block(function, "entry");

        compile.builder.position_at_end(basic_block);
        self.body.generate(compile);
    }
}

impl Ast for FileAst {
    fn generate<'ctx>(&self, compile: &CodeGen<'ctx>) {
        for function in self.functions.iter() {
            function.generate(compile);
        }
    }
}

impl Ast for BlockAst {
    fn generate<'ctx>(&self, compile: &CodeGen<'ctx>) {
        for statement in self.statements.iter() {
            statement.generate(compile);
        }
    }
}
