//use inkwell::context::Context;

use std::fmt::Debug;

use crate::x86_doc::{AmbitiousSyntax, Welcome10};

mod x86_doc;


#[derive(Debug)]
pub enum Register {
    Eax = 0x0000,
    Ecx = 0x0001,
    Edx = 0x0010,
    Ebx = 0x0011,
    Rsp = 0x0100,
    Ebp = 0x0101,
    Esi = 0x0110,
    Edi = 0x0111
}

#[derive(Debug)]
pub enum Opcode {
    Mov
}

#[derive(Debug)]
pub struct MODR {
    pub mod_ : u32,
    pub opcode: MODROpcode,
    pub r_m: u32
}

#[derive(Debug)]
pub enum MODROpcode {
    Reg(Register),
    Opcode(Opcode)
}

const MODR_M_MOD: u32 = 0b1100_0000;
const MODR_M_REG_OPCODE: u32 = 0b0011_1000;
const MODR_M_R_M: u32 = 0b0000_0111;

impl MODR {
    pub fn parse(ins: u32) -> Self {
        let mod_: u32 = (MODR_M_MOD & ins) >> 6;
        //let opcode = (MODR_M_REG_OPCODE & ins) >> 3;
        let opcode = MODROpcode::Opcode(Opcode::Mov);
        let r_m: u32 = MODR_M_R_M & ins;

        Self { mod_, opcode, r_m }
    }
}

#[derive(Debug)]
pub struct RexBlock {
    pub b: bool,
    pub s: bool,
    pub r: bool,
    pub w: bool
}

trait Parse : Debug {
    fn parseable(ins: u32) -> bool;
    fn parse(ins: u32) -> Self;
}

impl Parse for RexBlock {
    fn parseable(ins: u32) -> bool {
        static MASK: u32 = 0b1111_0000;
        (ins & MASK) == 0x40
    }

    fn parse(ins: u32) -> Self {
        let b: bool = (ins >> 0 & 1) != 0;  // extend register code
        let s: bool = (ins >> 1 & 1) != 0;
        let r: bool = (ins >> 2 & 1) != 0;
        let w: bool = (ins >> 3 & 1) != 0; // is 64-bit

        Self { b, s, r, w }
    }
}

pub enum OpcodeSize {
    One,
    Two
}

fn main() {
    let json_data: Welcome10 = serde_json::from_str(x86_doc::JSON_DATA).unwrap();

    let mut pc: usize = 0;
    let instructions: Vec<u32> = vec![0x0f, 0x05];

    let mut pick     = || -> u32 {
        let ins = instructions[pc];
        pc += 1;
        ins
    };

    const MODR_M_MOD: u32 = 0b1100_0000;
    const MODR_M_REG_OPCODE: u32 = 0b0011_1000;
    const MODR_M_R_M: u32 = 0b0000_0111;

    const SIB_SCALE: u32 = 0b1100_0000;
    const SIB_INDEX: u32 = 0b0011_1000;
    const SIB_BASE: u32 = 0b0000_0111;


    let ins : u32 = pick();

    if RexBlock::parseable(ins) {
        let rex = RexBlock::parse(ins);
        println!("rex: {:?}", rex);
    }

    let mod_ = (MODR_M_MOD & ins) >> 6;
    let reg_opcode = (MODR_M_REG_OPCODE & ins) >> 3;
    let r_m: u32 = MODR_M_R_M & ins;

    let scale = (SIB_SCALE & ins) >> 6;
    let index = (SIB_INDEX & ins) >> 3;
    let base: u32 = SIB_BASE & ins;



    let opcode_test : u32 = 0b0000_1111;

    println!("Value: {:08b}", (MODR_M_REG_OPCODE & opcode_test) >> 3);

    let rex = 0x49;
    let b: bool = (rex >> 0 & 1) != 0;  // extend register code
    let s: bool = (rex >> 1 & 1) != 0;
    let r: bool = (rex >> 2 & 1) != 0;
    let w: bool = (rex >> 3 & 1) != 0; // is 64-bit

    println!("Value: {:08b} B: {} S: {} R: {} W: {}", rex, b, s, r, w);

    let ins : u32 = pick();

    for one_byte in json_data.one_byte.iter() {
        if ins == u32::from_str_radix(&one_byte.value, 16).unwrap() {

            match &one_byte.entry {
                x86_doc::OneByteEntry::FluffyEntry(entry) => {

                    let full_mod_r_m = entry.r.is_some();

                    println!("{}", match &entry.syntax {
                        AmbitiousSyntax::TentacledSyntax(syntax) => syntax.mnem.clone(),
                        AmbitiousSyntax::PurpleSyntaxArray(syntax_list) => {
                            syntax_list.first().unwrap().mnem.clone()
                        }
                    });
                },
                x86_doc::OneByteEntry::PurpleEntryArray(entries) => todo!(),
            };
        }
    }

    // b >> n & 1
}
