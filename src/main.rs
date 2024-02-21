//use inkwell::context::Context;


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

pub enum Opcode {
    Mov
}

pub struct MODR {
    pub mod_ : u32,
    pub opcode: MODROpcode,
    pub r_m: u32
}

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

pub struct RexBlock {
    pub b: bool,
    pub s: bool,
    pub r: bool,
    pub w: bool
}

impl RexBlock {
    pub fn parseable(ins: u32) -> bool {
        static MASK: u32 = 0b1111_0000;
        (ins & MASK) == 0x40
    }

    pub fn parse(ins: u32) -> Self {
        let b: bool = (ins >> 0 & 1) != 0;  // extend register code
        let s: bool = (ins >> 1 & 1) != 0;
        let r: bool = (ins >> 2 & 1) != 0;
        let w: bool = (ins >> 3 & 1) != 0; // is 64-bit

        Self { b, s, r, w }
    }
}

fn main() {
    let mut pc: usize = 0;
    let instructions: Vec<u32> = vec![0x49, 0x89, 0xc8];

    let mut pick = || -> u32 {
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

    // b >> n & 1
}
