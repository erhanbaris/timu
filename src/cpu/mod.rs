use std::collections::HashMap;

use bus::Bus;

pub mod test;
pub mod bus;
pub mod memory;

static REX_MASK: u8 = 0b1111_0000;

const MODR_M_MOD: u8 = 0b1100_0000;
const MODR_M_REG_OPCODE: u8 = 0b0011_1000;
const MODR_M_R_M: u8 = 0b0000_0111;

const REGISTER_RAX: usize = 0;
const REGISTER_RCX: usize = 1;
const REGISTER_RDX: usize = 2;
const REGISTER_RBX: usize = 3;
const REGISTER_RSP: usize = 4;
const REGISTER_RBP: usize = 5;
const REGISTER_RSI: usize = 6;
const REGISTER_RDI: usize = 7;

const REGISTER_R8: usize = 8;
const REGISTER_R9: usize = 9;
const REGISTER_R10: usize = 10;
const REGISTER_R11: usize = 11;
const REGISTER_R12: usize = 12;
const REGISTER_R13: usize = 13;
const REGISTER_R14: usize = 14;
const REGISTER_R15: usize = 15;

#[derive(Debug)]
pub enum Operand {
    Register(u8),
    Immediate(u64),
    Memory(u8)
}

#[derive(Debug, Copy, Clone)]
pub enum OperandCount {
    None,
    One,
    Two
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Add,
    Mov
}

struct ModRM {
    pub mod_: u8,
    pub reg_opcode: u8,
    pub r_m: u8
}

#[derive(Debug, Clone)]
struct OpcodeFormat {
    pub opcode: Opcode,
    pub code: u8,
    pub has_modrm: bool,
    pub operator_count: OperandCount,
    pub reg_in_opcode: bool
}

type HookFn = fn(cpu: &Cpu);

#[derive(Debug)]
pub struct Cpu {
    pub rip: u64, // Instruction pointer
    pub rflags: u64, // Flag registers

    pub registers: [u64; 16],
    pub bus: Bus,

    hooks: Vec<HookFn>,

    opcode_formats: HashMap<u8, OpcodeFormat>,

    rex_b: bool, // extend r/m
    rex_x: bool,
    rex_r: bool,
    rex_w: bool, // is 64-bit
}

impl Default for Cpu {
    fn default() -> Self {
        let mut cpu = Self {
            bus: Bus::default(),
            hooks: Default::default(),
            opcode_formats: Default::default(),
            registers: Default::default(),
            rex_b: false,
            rex_r: false,
            rex_w: false,
            rex_x: false,
            rflags: 0,
            rip: 0
        };
        cpu.initialize();
        cpu
    }
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let mut cpu = Self::default();
        cpu.bus = bus;
        cpu
    }

    pub fn initialize(&mut self) {
        self.add_opcode_format(Opcode::Add, 0x01, false, true, OperandCount::Two);
        self.add_opcode_format(Opcode::Add, 0x48, false, true, OperandCount::Two);

        self.add_opcode_format(Opcode::Mov, 0xb8, true, false, OperandCount::Two);
        self.add_opcode_format(Opcode::Mov, 0x89, false, true, OperandCount::Two);
        self.add_opcode_format(Opcode::Mov, 0xc7, false, true, OperandCount::Two);
        self.add_opcode_format(Opcode::Mov, 0x41, false, true, OperandCount::Two);
    }

    fn add_opcode_format(&mut self, opcode: Opcode, code: u8, contain_reg: bool, has_modrm: bool, operator_count: OperandCount) {
        if contain_reg  {
            self.opcode_formats.insert(code + 0, OpcodeFormat { code: code + 0, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 1, OpcodeFormat { code: code + 1, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 2, OpcodeFormat { code: code + 2, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 3, OpcodeFormat { code: code + 3, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 4, OpcodeFormat { code: code + 4, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 5, OpcodeFormat { code: code + 5, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 6, OpcodeFormat { code: code + 6, opcode, has_modrm, operator_count, reg_in_opcode: true });
            self.opcode_formats.insert(code + 7, OpcodeFormat { code: code + 7, opcode, has_modrm, operator_count, reg_in_opcode: true });

        } else {
            self.opcode_formats.insert(code, OpcodeFormat { code, opcode, has_modrm, operator_count, reg_in_opcode: false });
        }
    }
    
    #[inline(always)]
    fn fetch(&mut self) -> u8 {
        let rip = self.rip;
        self.rip += 1;
        self.bus.read8(rip as usize)
    }

    pub fn boot(&mut self) {
        let memory_len = self.bus.len() as u64;
        while self.rip < memory_len {
            let opcode = self.fetch();

            if opcode == 0 { break; }
            self.execute(opcode);
            self.execute_hooks();
        }
    }

    #[inline(always)]
    fn modrm(&mut self) -> ModRM {
        let ins = self.fetch();

        ModRM {
            mod_: (MODR_M_MOD & ins) >> 6,
            reg_opcode: (MODR_M_REG_OPCODE & ins) >> 3,
            r_m: MODR_M_R_M & ins
        }
    }

    #[inline(always)]
    fn read8(&mut self) -> u8 {
        let value = self.bus.read8(self.rip as usize);
        self.rip += 1;
        value
    }

    #[inline(always)]
    fn read16(&mut self) -> u16 {
        let value = self.bus.read16(self.rip as usize);
        self.rip += 2;
        value
    }

    #[inline(always)]
    fn read32(&mut self) -> u32 {
        let value = self.bus.read32(self.rip as usize);
        self.rip += 4;
        value
    }

    #[inline(always)]
    fn read64(&mut self) -> u64 {
        let value = self.bus.read64(self.rip as usize);
        self.rip += 8;
        value
    }

    #[inline(always)]
    fn opcode_reset(&mut self) {
        self.rex_b = false; // extend register code
        self.rex_x = false;
        self.rex_r = false;
        self.rex_w = false; // is 64-bit
    }

    fn build_reg(&self, reg: &mut u8) {
        if self.rex_b {
            *reg |= 0b0000_1000;
        }
    }

    fn read_rex_memory(&mut self) -> u64 {
        match self.rex_w {
            true => self.read64(),
            false => self.read32() as u64
        }
    }

    pub fn add_hook(&mut self, hook: HookFn) {
        self.hooks.push(hook);
    }

    fn execute_hooks(&self) {
        for hook in self.hooks.iter() {
            hook(self)
        }
    }

    fn process(&mut self, opcode: u8) -> (Opcode, Option<Operand>, Option<Operand>) {
        let mut source = None;
        let mut target = None;

        println!("Searching: {}", opcode);
        let format = &self.opcode_formats[&opcode].clone();

        if format.reg_in_opcode {
            target = Some(Operand::Register(0b0000_0111 & opcode));
        }

        match format.operator_count {
            OperandCount::None => { /* Nothing to do */ },
            OperandCount::One => { /* Later */ },
            OperandCount::Two => {
                if format.has_modrm {
                    /*let modrm = self.modrm();
                    match modrm.mod_ {
                        0x03 /* 0011 */ => {
                            source = Some(Operand::Register(modrm.reg_opcode));
                            target = Some(Operand::Register(modrm.r_m));
                        }
                        _ => todo!("not implemented")
                    }*/
                }
            }
        }

        (format.opcode.clone(), source, target)
    }

    pub fn execute(&mut self, opcode: u8) {
        let mut opcode = opcode;
        self.opcode_reset();

        // Rex opcode
        if (opcode & REX_MASK) == 0x40 {
            self.rex_b = (opcode >> 0 & 1) != 0; // extend register code
            self.rex_x = (opcode >> 1 & 1) != 0;
            self.rex_r = (opcode >> 2 & 1) != 0;
            self.rex_w = (opcode >> 3 & 1) != 0; // is 64-bit
            opcode = self.fetch();
        }

        let (new_opcode, source, target) = self.process(opcode);
        println!("Format: {:?}", (new_opcode, source, target));
        
        match opcode {
            0 => {
                // Empty
            }

            0x01 => { // Add
                let modrm = self.modrm();
                match modrm.mod_ {
                    0x03 /* 0011 */ => {
                        if self.rex_w {
                            self.registers[modrm.r_m as usize] += self.registers[modrm.reg_opcode as usize]
                        } else {
                            self.registers[modrm.r_m as usize] = (self.registers[modrm.r_m as usize] as u32 + self.registers[modrm.reg_opcode as usize] as u32) as u64
                        }
                    }
                    _ => todo!("not implemented")
                }
            }

            0xb8..=0xbf => { // Mov
                let mut reg = 0b0000_0111 & opcode;

                if self.rex_b {
                    reg |= 0b0000_1000;
                }

                self.registers[reg as usize] = self.read64();
            }

            0x89 => { // Mov
                let mut modrm = self.modrm();
                match modrm.mod_ {
                    0x03 /* 0011 */ => {

                        if self.rex_b {
                            modrm.r_m |= 0b0000_1000;
                        }

                        if self.rex_w {
                            self.registers[modrm.r_m as usize] = self.registers[modrm.reg_opcode as usize]
                        } else {
                            self.registers[modrm.r_m as usize] = (self.registers[modrm.r_m as usize] as u32 + self.registers[modrm.reg_opcode as usize] as u32) as u64
                        }
                    }
                    _ => todo!("not implemented")
                }
            }

            0xc7 => { // Mov
                let mut modrm = self.modrm();
                match modrm.mod_ {
                    0x03 /* 0011 */ => {

                        let mut reg = 0b0000_0111 & modrm.r_m;

                        if self.rex_b {
                            modrm.r_m |= 0b0000_1000;
                        }

                        if self.rex_w {
                            self.registers[modrm.r_m as usize] = self.read64()
                        } else {
                            self.registers[modrm.r_m as usize] = (self.registers[modrm.r_m as usize] as u32 + self.read32()) as u64
                        }
                    }
                    _ => todo!("not implemented")
                }
            }

            0x48 => { // Add
                let mut modrm = self.modrm();
                match modrm.mod_ {
                    0x03 /* 0011 */ => {

                        let mut reg = 0b0000_0111 & modrm.r_m;

                        if self.rex_b {
                            modrm.r_m |= 0b0000_1000;
                        }

                        if self.rex_w {
                            self.registers[modrm.r_m as usize] += self.read64()
                        } else {
                            self.registers[modrm.r_m as usize] = (self.registers[modrm.r_m as usize] as u32 + self.read32()) as u64
                        }
                    }
                    _ => todo!("not implemented")
                }
            }

            0xF4 => {
                // Halt
                self.rip = self.bus.len() as u64
            }
            
            _ => { println!("Not implemented yet") }
        }
    }

    pub fn dump(&self) {
        println!("rip: {}", self.rip);
        for reg in self.registers.iter() {
            println!("{}", reg);
        }
    }
}
