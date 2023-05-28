//use inkwell::{context::Context, module::{Module, Linkage}, builder::Builder, types::BasicMetadataTypeEnum, AddressSpace};
//use cranelift_codegen;

use std::{collections::HashMap};

use codegen::Context;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Module, DataId};

use crate::{ast::{TimuAst, VariableType, AccessType, FuncArg, TimuAstType}, parser::TimuParserError};

#[derive(Default)]
pub struct CodeGen {
}

#[derive(Default)]
pub struct TimuContext {
    pub types: HashMap<String, Type>
}

pub struct TimuTool;

impl TimuTool {
    pub fn build_full_type_path(ctx: &mut TimuContext, return_type: &TimuAstType) -> Option<String> {
        Some(return_type.join("."))
    }

    pub fn is_type_contains(ctx: &mut TimuContext, name: &str) -> bool {
        ctx.types.contains_key(name)
    }

    pub fn get_type(ctx: &mut TimuContext, name: &str) -> Option<Type> {
        ctx.types.get(name).cloned()
    }

    pub fn add_type(ctx: &mut TimuContext, name: &str, new_type: Type) {
        ctx.types.insert(name.to_string(), new_type);
    }

    pub fn get_type_from_path(ctx: &mut TimuContext, type_path: &TimuAstType) -> Result<Type, TimuParserError> {
        let full_type_name = match TimuTool::build_full_type_path(ctx, type_path) {
            Some(type_path) => type_path,
            None => return Err(TimuParserError::new_with_info(0, 0, format!("'{}' type is unknown", type_path.join("."))))
        };

        let found_type = match TimuTool::get_type(ctx, &full_type_name) {
            Some(found_type) => found_type,
            None => return Err(TimuParserError::new_with_info(0, 0, format!("'{}' type is unknown", type_path.join("."))))
        };

        Ok(found_type)
    }
}

pub struct TimuModule {
    pub module: JITModule,
    pub data_ctx: DataContext,
    pub codegen_ctx: codegen::Context,
    pub function_builder_ctx: FunctionBuilderContext,
}

impl TimuModule {

    fn declare_variables(function_builder: &mut FunctionBuilder, params: &[String], the_return: &str, stmts: &[TimuAst], entry_block: Block) -> HashMap<String, Variable> {
        let mut variables = HashMap::new();
        let mut index = 0;
        
        /*
        for (index, name) in params.iter().enumerate() {
            let val = function_builder.block_params(entry_block)[i];
            let var = declare_variable(int, builder, &mut variables, &mut index, name);
            function_builder.def_var(var, val);
        }

        let zero = function_builder.ins().iconst(int, 0);
        let return_variable = declare_variable(int, builder, &mut variables, &mut index, the_return);
        function_builder.def_var(return_variable, zero);
        for expr in stmts {
            declare_variables_in_stmt(int, builder, &mut variables, &mut index, expr);
        }*/
    
        variables
    }

    fn build_variable(&mut self, ctx: &mut TimuContext, name: String, data: Box<TimuAst>, variable_type: VariableType) -> Result<(), TimuParserError> {
        let mutable = match variable_type {
            VariableType::Immutable => false,
            VariableType::Mutable => true,
        };

        let data_id = self.module.declare_data(&name, cranelift_module::Linkage::Export, mutable, false).unwrap();
        self.data_ctx.define_zeroinit(0);
        self.module.define_data(data_id, &self.data_ctx).unwrap();

        Ok(())
    }

    fn build_static_variable(&mut self, ctx: &mut TimuContext, name: String, data: Box<TimuAst>, variable_type: VariableType) -> Result<(), TimuParserError> {
        /* Create static variable */
        let mutable = match variable_type {
            VariableType::Immutable => false,
            VariableType::Mutable => true,
        };

        let data_id = self.module.declare_data(&name, cranelift_module::Linkage::Export, mutable, false).unwrap();
        self.data_ctx.define_zeroinit(0);
        self.module.define_data(data_id, &self.data_ctx).unwrap();

        Ok(())
    }

    fn build_define_assignment(&mut self, ctx: &mut TimuContext, name: String, data: Box<TimuAst>, variable_type: VariableType, is_module: bool) -> Result<(), TimuParserError> {
        match is_module {
            true => self.build_static_variable(ctx, name, data, variable_type),
            false => self.build_variable(ctx, name, data, variable_type)
        }
    }

    fn build_function_definition(&mut self, ctx: &mut TimuContext, access: AccessType, name: String, args: Vec<FuncArg>, return_type: TimuAstType, body: Box<TimuAst>) -> Result<(), TimuParserError> {

        // Set return type
        let return_type = TimuTool::get_type_from_path(ctx, &return_type)?;
        self.codegen_ctx.func.signature.returns.push(AbiParam::new(return_type));

        // Set arguments
        for arg in args.iter() {
            let argument_type = TimuTool::get_type_from_path(ctx, &arg.arg_type)?;
            self.codegen_ctx.func.signature.params.push(AbiParam::new(argument_type));
        }

        let mut function_builder = FunctionBuilder::new(&mut self.codegen_ctx.func, &mut self.function_builder_ctx);

        // Entry block for function
        let entry_block = function_builder.create_block();

        function_builder.append_block_params_for_function_params(entry_block);
        function_builder.switch_to_block(entry_block);

        // Not now, later change this code
        function_builder.seal_block(entry_block);

        //let variables = self.declare_variables(int, &mut builder, &params, &the_return, &stmts, entry_block);


        Ok(())
    }

    fn build_statement(&mut self, ctx: &mut TimuContext, ast: Box<TimuAst>, is_module: bool) -> Result<(), TimuParserError> {
        match *ast {
            TimuAst::Import { path, name } => todo!(),
            TimuAst::File { statements } => todo!(),
            TimuAst::Ident(_) => todo!(),
            TimuAst::Primative(_) => todo!(),
            TimuAst::Unary(_, _) => todo!(),
            TimuAst::FunctionCall { compiler, name, args } => todo!(),
            TimuAst::BinaryOperation { left, operator, right } => todo!(),
            TimuAst::FunctionDefinition { access, name, args, return_type, body } => self.build_function_definition(ctx, access, name, args, return_type, body),
            TimuAst::Block { statements } => todo!(),
            TimuAst::DefAssignment { r#type, type_annotation, name, data } => self.build_define_assignment(ctx, name, data, r#type, is_module),
            TimuAst::Assignment { name, data } => todo!(),
        }
    }

    pub fn build(&mut self, ctx: &mut TimuContext, statements: Vec<Box<TimuAst>>) -> Result<(), TimuParserError> {
        for statement in statements {
            self.build_statement(ctx, statement, true)?;
        }
        Ok(())
    }
}

impl CodeGen {
    pub fn compile(&self, ast: TimuAst) -> Result<(), TimuParserError> {
        
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        if let TimuAst::File { statements } = ast {

            let mut ctx = TimuContext::default();

            let mut module = TimuModule {
                module: JITModule::new(builder),
                data_ctx: DataContext::new(),
                codegen_ctx: codegen::Context::new(),
                function_builder_ctx: FunctionBuilderContext::new()
            };

            TimuTool::add_type(&mut ctx, "i8", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "i16", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "i32", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "i64", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "u8", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "u16", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "u32", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "u64", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "bool", module.module.target_config().pointer_type());
            TimuTool::add_type(&mut ctx, "string", module.module.target_config().pointer_type());
            
            module.build(&mut ctx, statements)?;
            Ok(())
        } else {
            Err(TimuParserError::new_with_info(0, 0, "".to_string()))
        }

    }

    fn declare_variables(&self, ast: &TimuAst, params: Vec<String>, builder: &mut FunctionBuilder, block: &mut Block) {
        let mut variables = HashMap::<String, Variable>::new();
        let mut index = 0;
    }
}
