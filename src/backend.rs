use std::{borrow::Borrow, fmt::Display};

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum NewRegister {
    AL,
    BL,
    CL,
    DL,
    AH,
    BH,
    CH,
    DH,
    DIL,
    SIL,
    BPL,
    SPL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B, // Byte Registers
    AX,
    BX,
    CX,
    DX,
    DI,
    SI,
    BP,
    SP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W, // Word Registers
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    EBP,
    ESP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D, // Doubleword Registers
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RBP,
    RSP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15, // Quadword Registers

    LASTELEMENT,
}

impl Display for NewRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Number {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Float(f32),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::I8(num) => write!(f, "{}", num),
            Number::U8(num) => write!(f, "{}", num),
            Number::I16(num) => write!(f, "{}", num),
            Number::U16(num) => write!(f, "{}", num),
            Number::I32(num) => write!(f, "{}", num),
            Number::U32(num) => write!(f, "{}", num),
            Number::I64(num) => write!(f, "{}", num),
            Number::U64(num) => write!(f, "{}", num),
            Number::Float(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Location {
    Memory(i64),
    Register(NewRegister),
    Imm(Number),
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add { source: Location, target: Location },
    Sub { source: Location, target: Location },
    Mov { source: Location, target: Location },

    Push(NewRegister),
    Pop(NewRegister),
    Ret,
}

impl AsmGenerate for Operation {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String) {
        match self {
            Operation::Add {
                source: source,
                target,
            } => self.do_add(source, target, buffer),
            Operation::Sub { source, target } => self.do_sub(source, target, buffer),
            Operation::Mov { source, target } => self.do_mov(source, target, buffer),
            Operation::Ret => self.do_ret(buffer),
            Operation::Push(register) => self.do_push(register, buffer),
            Operation::Pop(register) => self.do_pop(register, buffer),
        };
        buffer.push_str("\r\n");
    }
}

impl Operation {
    fn do_ret(&self, buffer: &mut String) {
        buffer.push_str("ret");
    }

    fn do_push(&self, register: &NewRegister, buffer: &mut String) {
        buffer.push_str(&format!("push %{}", register.to_string().to_lowercase()));
    }

    fn do_pop(&self, register: &NewRegister, buffer: &mut String) {
        buffer.push_str(&format!("pop %{}", register.to_string().to_lowercase()));
    }

    fn do_add(&self, source: &Location, target: &Location, buffer: &mut String) {
        match (source, target) {
            (Location::Imm(imm), Location::Register(register)) => buffer.push_str(&format!(
                "add ${}, %{}",
                imm,
                register.to_string().to_lowercase()
            )),
            (Location::Register(source_reg), Location::Register(target_reg)) => {
                buffer.push_str(&format!(
                    "add %{}, %{}",
                    source_reg.to_string().to_lowercase(),
                    target_reg.to_string().to_lowercase()
                ))
            }
            _ => panic!("unsupported"),
        }
    }

    fn do_sub(&self, source: &Location, target: &Location, buffer: &mut String) {
        match (source, target) {
            (Location::Imm(imm), Location::Register(register)) => buffer.push_str(&format!(
                "sub ${}, %{}",
                imm,
                register.to_string().to_lowercase()
            )),
            (Location::Register(source_reg), Location::Register(target_reg)) => {
                buffer.push_str(&format!(
                    "sub %{}, %{}",
                    source_reg.to_string().to_lowercase(),
                    target_reg.to_string().to_lowercase()
                ))
            }
            _ => panic!("unsupported"),
        }
    }

    fn do_mov(&self, source: &Location, target: &Location, buffer: &mut String) {
        match (source, target) {
            (Location::Imm(imm), Location::Register(register)) => buffer.push_str(&format!(
                "mov ${}, %{}",
                imm,
                register.to_string().to_lowercase()
            )),
            (Location::Register(source_reg), Location::Register(target_reg)) => {
                buffer.push_str(&format!(
                    "mov %{}, %{}",
                    source_reg.to_string().to_lowercase(),
                    target_reg.to_string().to_lowercase()
                ))
            }
            _ => panic!("unsupported"),
        }
    }
}

pub trait OsSpecificDefs {
    fn main_function_name(&self) -> &'static str;
    fn end_of_file_instructions(&self) -> &'static str;
}

#[derive(Debug, Clone, Default)]
struct MacSpecificDefs;

#[derive(Debug, Clone, Default)]
struct LinuxSpecificDefs;

impl OsSpecificDefs for MacSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "_main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ""
    }
}

impl OsSpecificDefs for LinuxSpecificDefs {
    fn main_function_name(&self) -> &'static str {
        "main"
    }

    fn end_of_file_instructions(&self) -> &'static str {
        ".section .note.GNU-stack,\"\",@progbits"
    }
}

pub struct ApplicationContext {
    pub os_specific_defs: Box<dyn OsSpecificDefs>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        let info = os_info::get();

        Self {
            os_specific_defs: match info.os_type() {
                os_info::Type::Linux => Box::new(LinuxSpecificDefs::default()),
                os_info::Type::Macos => Box::new(MacSpecificDefs::default()),
                os => panic!("Unsupported OS ({}", os),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Application {
    pub functions: Vec<Function>,
}

impl AsmGenerate for Application {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String) {
        buffer.push_str(&format!(
            ".globl {}\r\n",
            context.os_specific_defs.main_function_name()
        ));
        for func in self.functions.iter() {
            func.generate(context, buffer);
        }

        buffer.push_str(context.os_specific_defs.end_of_file_instructions());
    }
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Operation>,
}

pub trait AsmGenerate {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String);
}

impl AsmGenerate for Function {
    fn generate(&self, context: &mut ApplicationContext, buffer: &mut String) {
        buffer.push_str(&self.name);
        buffer.push_str(":");
        buffer.push_str("\r\n");

        // Function begin
        self.print_inst(Operation::Push(NewRegister::RBP), context, buffer);
        self.print_inst(
            Operation::Mov {
                source: Location::Register(NewRegister::RSP),
                target: Location::Register(NewRegister::RBP),
            },
            context,
            buffer,
        );

        buffer.push_str("    # function body begin\r\n");

        for instruction in self.instructions.iter() {
            self.print_inst(instruction, context, buffer);
        }
        buffer.push_str("    # function body end\r\n");

        // Function end
        self.print_inst(
            Operation::Mov {
                source: Location::Register(NewRegister::RBP),
                target: Location::Register(NewRegister::RSP),
            },
            context,
            buffer,
        );
        self.print_inst(Operation::Pop(NewRegister::RBP), context, buffer);
        self.print_inst(Operation::Ret, context, buffer);
    }
}

impl Function {
    fn print_inst<T: Borrow<Operation>>(
        &self,
        inst: T,
        context: &mut ApplicationContext,
        buffer: &mut String,
    ) {
        buffer.push_str("    ");
        inst.borrow().generate(context, buffer);
    }
}
