#![allow(warnings)]

use backend::{
    Application, ApplicationContext, AsmGenerate, Function, Location, NewRegister, Number,
    Operation,
};
use cpu::{bus::Bus, memory::MemoryBuilder, Cpu};

mod ast;
mod backend;
mod cpu;
mod format;
mod jit;
mod parser;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;

use std::error::Error;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum").unwrap();
        let sum = self.builder.build_int_add(sum, z, "sum").unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        unsafe { self.execution_engine.get_function("sum").ok() }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parser::parser(
        r#"type MyType {
    pub a: ?string,
    pub b: ?string,
    pub c: string,
    pub d: string.v1ddasdd,
    func test() {
    }
}"#,
    );
    println!("{:?}", parsed);

    let context = Context::create();
    let module = context.create_module("sum");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let sum = codegen
        .jit_compile_sum()
        .ok_or("Unable to JIT compile `sum`")?;

    let x = 1u64;
    let y = 2u64;
    let z = 3u64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }

    let mut context = ApplicationContext::new();
    let mut application = Application::default();
    let mut main = Function::default();
    main.name = "_main".to_string();
    main.instructions.push(Operation::Mov {
        source: Location::Imm(Number::U64(20)),
        target: Location::Register(NewRegister::EAX),
    });
    application.functions.push(main);
    let mut buffer = String::new();

    application.generate(&mut context, &mut buffer);

    println!("{}", buffer);

    let mut memory = MemoryBuilder::new(100);
    return Ok(());
    println!("Instructions:");

    let mut memory = MemoryBuilder::new(100);
    /*memory.write8(0x48);
    memory.write8(0xB8);
    memory.write64(0x10);

    memory.write8(0x48);
    memory.write8(0xB9);
    memory.write64(0x05);

    memory.write8(0x01);
    memory.write8(0xC8);*/

    memory.write8(0x48);
    memory.write8(0xC7);
    memory.write8(0xC0);
    memory.write8(0x01);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x48);
    memory.write8(0xC7);
    memory.write8(0xC3);
    memory.write8(0x02);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x00);
    memory.write8(0x48);
    memory.write8(0x01);
    memory.write8(0xD8);
    memory.write8(0x48);
    memory.write8(0x01);
    memory.write8(0xC8);
    memory.write8(0x90);

    println!("");
    let bus = Bus::new(memory.generate());
    let mut cpu = Cpu::new(bus);
    //cpu.boot();
    println!("RAX: {}", cpu.registers[0]);
    println!("RCX: {}", cpu.registers[1]);
    cpu.dump();
}
