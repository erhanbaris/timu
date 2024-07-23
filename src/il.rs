use std::borrow::Borrow;


const MODR_M_MOD: u8 = 0b1100_0000;
const MODR_M_REG_OPCODE: u8 = 0b0011_1000;
const MODR_M_R_M: u8 = 0b0000_0111;

pub const OPERAND_SIZE_OVERRIDE: u8 = 0x66;
pub const ADDRESS_SIZE_OVERRIDE: u8 = 0x67;

#[derive(Copy, Clone, PartialEq)]
pub enum RegisterType {
    _8Bit,
    _16Bit,
    _32Bit,
    _64Bit
}

#[repr(usize)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum NewRegister {
    AL, BL, CL, DL, AH, BH, CH, DH, DIL, SIL, BPL, SPL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B, // Byte Registers
    AX, BX, CX, DX, DI, SI, BP, SP, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W, // Word Registers
    EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D, // Doubleword Registers
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15, // Quadword Registers

    LASTELEMENT
}

pub const REGISTER_OPCODES: [u8; 69] = [
    // Byte Registers
    0, 3, 1, 2, /* Fix it later */ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, /* Fix it later */
    
    // Word Registers
    0, 3, 1, 2, 6, 7, 5, 4, 8, 9, 10, 11, 12, 13, 14, 15,
    
    // Doubleword Registers
    0, 3, 1, 2, 6, 7, 5, 4, 8, 9, 10, 11, 12, 13, 14, 15,
    
    // Quadword Registers
    0, 3, 1, 2, 6, 7, 5, 4, 8, 9, 10, 11, 12, 13, 14, 15,
    
    // LASTELEMENT
    255
];

pub const REGISTER_TYPES: [RegisterType; 69] = [
    // Byte Registers
    RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit, RegisterType::_8Bit,
    
    // Word Registers
    RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit, RegisterType::_16Bit,
    
    // Doubleword Registers
    RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit, RegisterType::_32Bit,  RegisterType::_32Bit,
    
    // Quadword Registers
    RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit, RegisterType::_64Bit,  RegisterType::_64Bit,
    
    // LASTELEMENT
    RegisterType::_8Bit
];

pub const REGISTER_REX_NEED: [bool; 69] = [
    // Byte Registers
    false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true,
    
    // Word Registers
    false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true,
    
    // Doubleword Registers
    false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true,
    
    // Quadword Registers
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    
    // LASTELEMENT
    false
];

// Compile time checks
const _: () = assert!(REGISTER_TYPES.len() == NewRegister::LASTELEMENT as usize + 1, "Missing register types");
const _: () = assert!(REGISTER_REX_NEED.len() == NewRegister::LASTELEMENT as usize + 1, "Missing register rex need information");
const _: () = assert!(REGISTER_OPCODES.len() == NewRegister::LASTELEMENT as usize + 1, "Missing register opcode");

#[derive(Debug)]
#[derive(Default)] 
pub struct Register (pub u8);

#[derive(Debug)]
#[derive(Clone)]
pub struct RegisterRef (pub NewRegister);

impl Default for RegisterRef {
    fn default() -> Self {
        RegisterRef(NewRegister::LASTELEMENT)
    }
}

#[derive(Debug)]
pub enum MODROpcode {
    Reg(Register),
    Opcode(u8)
}

#[derive(Default)] pub struct InsPrefix<'a>(pub &'a [u8]);
#[derive(Default)] pub struct InsOpcode<'a>(pub &'a [u8]);

#[derive(Clone)]
pub enum InsModRMMode {
    RegisterToDirect(NewRegister, NewRegister),
    RegisterRefToRegister(RegisterRef, NewRegister)
}

impl Default for InsModRMMode {
    fn default() -> Self {
        InsModRMMode::RegisterToDirect(NewRegister::LASTELEMENT, NewRegister::LASTELEMENT)
    }
}

#[derive(Default)] pub struct InsModRM {
    pub mode: InsModRMMode,
    pub reg: u8,
    pub rm: u8
}

#[derive(Default)] pub struct InsSIB;
#[derive(Default)] pub struct InsDisplacement;

#[derive(Default)] pub struct InsDataElement;

#[derive(Default)]
pub struct Instruction<'a> {
    pub prefix: Option<InsPrefix<'a>>,
    pub opcode: InsOpcode<'a>,
    pub modrm: Option<InsModRM>,
    pub sib: Option<InsSIB>,
    pub displacement: Option<InsDisplacement>,
    pub data_element: Option<InsDataElement>,
}

impl<'a> Instruction<'a> {
    pub fn build(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut has_modrm_buffer = false;
        let mut modrm_buffer: u8 = 0b0000_0000;
        let mut is_64bit = false;

        if let Some(InsPrefix(prefix)) = self.prefix {
            assert!(prefix.len() > 0 && prefix.len() < 5, "Prefix must 1-4 bytes");

            buffer.extend_from_slice(prefix)
        }

        assert!(self.opcode.0.len() > 0 && self.opcode.0.len() < 5, "Opcode must 1-4 bytes");
        if let Some(modrm) = self.modrm.as_ref() {
            has_modrm_buffer = true;

            match modrm.mode.clone() {
                InsModRMMode::RegisterToDirect(reg1, reg2) => {
                    let reg1_type = REGISTER_TYPES[reg1 as usize];
                    let reg2_type = REGISTER_TYPES[reg2 as usize];
                    is_64bit = reg1_type == RegisterType::_64Bit || reg2_type == RegisterType::_64Bit;

                    modrm_buffer |= 0b1100_0000;
                    modrm_buffer |= REGISTER_OPCODES[reg1 as usize] << 3;
                    modrm_buffer |= REGISTER_OPCODES[reg2 as usize];
                }
                InsModRMMode::RegisterRefToRegister(RegisterRef(reg_ref), reg) => {
                    let reg1_type = REGISTER_TYPES[reg_ref as usize];
                    let reg2_type = REGISTER_TYPES[reg as usize];
                    is_64bit = reg1_type == RegisterType::_64Bit || reg2_type == RegisterType::_64Bit;
                    
                    modrm_buffer |= 0b0000_0000;
                    modrm_buffer |= REGISTER_OPCODES[reg as usize] << 3;
                    modrm_buffer |= REGISTER_OPCODES[reg_ref as usize];
                }
            }
        }

        if (is_64bit) {
            buffer.push(0x48);
        }

        buffer.extend_from_slice(self.opcode.0);

        assert!(buffer.len() < 16, "Buffer exceed 15 bytes");

        if (has_modrm_buffer) {
            buffer.push(modrm_buffer);
        }


        //0000 1110
        //0000 1111

        buffer
    }

    pub fn print(&self) {

    }
}

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

pub enum Location {
    Memory(i64),
    Register(Register),
    Number(Number)
}

pub enum Operation {
    Add,
    Sub
}

pub enum OpCode {
    Operation {
        operation: Operation,
        target: Location,
        left: Location,
        right: Location
    }
}


pub fn generate(opcode: OpCode) {
    match opcode {
        OpCode::Operation { operation, target, left, right } => do_operation(operation, target, left, right),
    }
}

fn do_operation(operation: Operation, target: Location, left: Location, right: Location) {
    match operation {
        Operation::Add => do_add(target, left, right),
        Operation::Sub => do_sub(target, left, right),
    }   
}

fn do_add(target: Location, left: Location, right: Location) {

}

fn do_sub(target: Location, left: Location, right: Location) {
}