//use inkwell::context::Context;

use std::fmt::Debug;

use il::{generate, InsModRM, InsModRMMode, InsOpcode, InsPrefix, Instruction};

use crate::x86_doc::{AmbitiousSyntax, Welcome10};

mod x86_doc;
mod il;


#[derive(Debug)]
#[repr(u8)]
pub enum Register {
    Eax, // 0b0000_0000
    Ecx, // 0b0000_0001
    Edx, // 0b0000_0010
    Ebx, // 0b0000_0011
    Rsp, // 0b0000_0100
    Ebp, // 0b0000_0101
    Esi, // 0b0000_0110
    Edi  // 0b0000_0111
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0b0000_0000 => Register::Eax,
            0b0000_0001 => Register::Ecx,
            0b0000_0010 => Register::Edx,
            0b0000_0011 => Register::Ebx,
            0b0000_0100 => Register::Rsp,
            0b0000_0101 => Register::Ebp,
            0b0000_0110 => Register::Esi,
            0b0000_0111 => Register::Edi,
            
            // Default
            _ => Register::Eax,
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Mov
}

#[derive(Debug)]
pub struct MODR {
    pub mod_ : u8,
    pub opcode: MODROpcode,
    pub r_m: u8
}

#[derive(Debug)]
pub enum MODROpcode {
    Reg(Register),
    Opcode(Opcode)
}

const MODR_M_MOD: u8 = 0b1100_0000;
const MODR_M_REG_OPCODE: u8 = 0b0011_1000;
const MODR_M_R_M: u8 = 0b0000_0111;

impl MODR {
    pub fn parse(ins: u8) -> Self {
        let mod_: u8 = (MODR_M_MOD & ins) >> 6;

        println!("mod_: {:#8b}", mod_);

        match mod_ {
            0x00 => println!("mod is 00"),
            0x01 => println!("mod is 01"),
            0x02 => println!("mod is 10"),
            0x03 => {
                println!("mod is 11");
                let reg1 = Register::from((MODR_M_REG_OPCODE & ins) >> 3);
                let reg2 = Register::from(MODR_M_R_M & ins);
            },
            _ => {
                println!("mod is other");
            }
        };

        //let opcode = (MODR_M_REG_OPCODE & ins) >> 3;
        let opcode = MODROpcode::Opcode(Opcode::Mov);
        let r_m: u8 = MODR_M_R_M & ins;

        Self { mod_, opcode, r_m }
    }
}

#[derive(Debug, Default)]
pub struct RexBlock {
    pub b: bool, // extension to the MODRM.rm field or the SIB.base field
    pub x: bool, // extension to the SIB.index
    pub r: bool, // extension to the MODRM.reg
    pub w: bool  // 64-bit operand size is used
}

trait Parse : Debug {
    fn parseable(ins: u8) -> bool;
    fn parse(ins: u8) -> Self;
}

impl Parse for RexBlock {
    fn parseable(ins: u8) -> bool {
        static MASK: u8 = 0b1111_0000;
        (ins & MASK) == 0x40
    }

    fn parse(ins: u8) -> Self {
        let b: bool = (ins >> 0 & 1) != 0; // extend register code
        let x: bool = (ins >> 1 & 1) != 0;
        let r: bool = (ins >> 2 & 1) != 0;
        let w: bool = (ins >> 3 & 1) != 0; // is 64-bit

        Self { b, x, r, w }
    }
}

pub enum OpcodeSize {
    One,
    Two
}

fn main() {

    let mut inst = Instruction::default();
    let opcode = vec![0x03];
    inst.opcode = InsOpcode(&opcode);
    
    let opcode = vec![0x03];
    // inst.prefix = InsPrefix()

    let mut modrm = InsModRM::default();
    modrm.mode = InsModRMMode::RegisterRefToRegister(il::RegisterRef(il::NewRegister::RBX), il::NewRegister::RCX);
    inst.modrm = Some(modrm);
    println!("Buffer: {:02X?}", inst.build());

    // generate(il::OpCode::Operation { operation: il::Operation::Add, target: il::Location::Register(il::Register::Eax), left: il::Location::Number(il::Number::I32(12345)), right: il::Location::Number(il::Number::I32(12345)) });
    let json_data: Welcome10 = serde_json::from_str(x86_doc::JSON_DATA).unwrap();

    let mut pc: usize = 0;
    let instructions: Vec<u8> = vec![0x66, 0xb8, 0x12, 0x00];

    let mut pick = || -> u8 {
        let ins = instructions[pc];
        pc += 1;
        ins
    };

    const MODR_M_MOD: u8 = 0b1100_0000;
    const MODR_M_REG_OPCODE: u8 = 0b0011_1000;
    const MODR_M_R_M: u8 = 0b0000_0111;

    const SIB_SCALE: u8 = 0b1100_0000;
    const SIB_INDEX: u8 = 0b0011_1000;
    const SIB_BASE: u8 = 0b0000_0111;


    let mut ins : u8 = pick();
    let mut rex_block = RexBlock::default();

    if RexBlock::parseable(0x67) {
        rex_block = RexBlock::parse(0x48);
        println!("rex: {:?}", rex_block);

        ins = pick();
    }

    let mod_ = (MODR_M_MOD & ins) >> 6;
    let reg_opcode = (MODR_M_REG_OPCODE & ins) >> 3;
    let r_m: u8 = MODR_M_R_M & ins;

    let scale = (SIB_SCALE & ins) >> 6;
    let index = (SIB_INDEX & ins) >> 3;
    let base: u8 = SIB_BASE & ins;



    let opcode_test : u8 = 0x67;

    println!("Value: {:08b}", (MODR_M_REG_OPCODE & opcode_test) >> 3);

    let rex = 0x67;
    let b: bool = (rex >> 0 & 1) != 0;  // extend register code
    let s: bool = (rex >> 1 & 1) != 0;
    let r: bool = (rex >> 2 & 1) != 0;
    let w: bool = (rex >> 3 & 1) != 0; // is 64-bit

    println!("Value: {:08b} B: {} S: {} R: {} W: {}", rex, b, s, r, w);

    

    println!("------------------------------------------------");

    if rex_block.r || rex_block.b {
        ins = pick();
        MODR::parse(ins);

    }

    // b >> n & 1
}
