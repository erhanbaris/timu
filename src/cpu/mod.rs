use bitmask_enum::bitmask;
use std::collections::HashMap;
use std::sync::LazyLock;

use bus::Bus;

use crate::format::BitMode;

pub mod bus;
pub mod memory;
pub mod tests;

static REX_MASK: u8 = 0b1111_0000;

const MODR_M_MOD: u8 = 0b1100_0000;
const MODR_M_REG_OPCODE: u8 = 0b0011_1000;
const MODR_M_R_M: u8 = 0b0000_0111;

const SIB_SCALE: u8 = 0b1100_0000;
const SIB_INDEX: u8 = 0b0011_1000;
const SIB_BASE: u8 = 0b0000_0111;

const OPERAND_SIZE_OVERWRITE_PREFIX: u8 = 0x66;

#[allow(dead_code)]
pub const REGISTER_RAX: usize = 0;
#[allow(dead_code)]
pub const REGISTER_RCX: usize = 1;
#[allow(dead_code)]
pub const REGISTER_RDX: usize = 2;
#[allow(dead_code)]
pub const REGISTER_RBX: usize = 3;
#[allow(dead_code)]
pub const REGISTER_RSP: usize = 4;
#[allow(dead_code)]
pub const REGISTER_RBP: usize = 5;
#[allow(dead_code)]
pub const REGISTER_RSI: usize = 6;
#[allow(dead_code)]
pub const REGISTER_RDI: usize = 7;

#[allow(dead_code)]
pub const REGISTER_R8: usize = 8;
#[allow(dead_code)]
pub const REGISTER_R9: usize = 9;
#[allow(dead_code)]
pub const REGISTER_R10: usize = 10;
#[allow(dead_code)]
pub const REGISTER_R11: usize = 11;
#[allow(dead_code)]
pub const REGISTER_R12: usize = 12;
#[allow(dead_code)]
pub const REGISTER_R13: usize = 13;
#[allow(dead_code)]
pub const REGISTER_R14: usize = 14;
#[allow(dead_code)]
pub const REGISTER_R15: usize = 15;

#[bitmask(u32)]
enum OperatorType {
    None,
    AX,
    EAX,
    RAX,
    FromModrmRM,
    FromModrmREG,
    FromOpcode,
    CanImmediate8,
    CanImmediate16,
    CanImmediate32,
    CanImmediate64,
    Reg8,
    Reg16,
    Reg32,
    Reg64,
}

const OPERATOR_TYPE_REG_32_64: LazyLock<OperatorType> =
    LazyLock::new(|| OperatorType::Reg32 | OperatorType::Reg64);
const OPERATOR_TYPE_REG_16_32_64: LazyLock<OperatorType> =
    LazyLock::new(|| OperatorType::Reg16 | OperatorType::Reg32 | OperatorType::Reg64);

const OPERATOR_TYPE_IMM_32_64: LazyLock<OperatorType> =
    LazyLock::new(|| OperatorType::CanImmediate32 | OperatorType::CanImmediate64);
const OPERATOR_TYPE_IMM_16_32_64: LazyLock<OperatorType> = LazyLock::new(|| {
    OperatorType::CanImmediate16 | OperatorType::CanImmediate32 | OperatorType::CanImmediate64
});

const OPERATOR_TYPE_REG_ECX_REX: LazyLock<OperatorType> =
    LazyLock::new(|| OperatorType::EAX | OperatorType::RAX);
const OPERATOR_TYPE_REG_AX_ECX_REX: LazyLock<OperatorType> =
    LazyLock::new(|| OperatorType::AX | OperatorType::EAX | OperatorType::RAX);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterType {
    _8Bit,
    _16Bit,
    _32Bit,
    _64Bit,
}

#[derive(Debug)]
pub enum TargetOperand {
    Register(u8, RegisterType),
    RegisterMemory(u8),
    Memory(u64),
}

#[derive(Debug)]
pub enum SourceOperand {
    Register(u8, RegisterType),
    Immediate(u64),
    RegisterMemory(u8),
    Memory(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Add,
    Mov,
    Nop,
}

struct ModRM {
    pub mod_: u8,
    pub reg_opcode: u8,
    pub r_m: u8,
}

struct Sib {
    #[allow(dead_code)]
    pub scale: u8,
    #[allow(dead_code)]
    pub index: u8,
    pub base: u8,
}

#[derive(Debug, Clone)]
struct OpcodeFormat {
    pub opcode: Opcode,
    pub target_info: OperatorType,
    pub source_info: OperatorType,
    pub _is8bit: bool,
}

type HookFn = fn(cpu: &Cpu);

#[derive(Debug)]
pub struct Cpu {
    pub rip: u64, // Instruction pointer

    #[allow(dead_code)]
    pub rflags: u64, // Flag registers

    pub registers: [u64; 16],
    pub bus: Bus,

    hooks: Vec<HookFn>,

    opcode_formats: HashMap<u8, OpcodeFormat>,
    target_operand: TargetOperand,
    source_operand: SourceOperand,

    rex_b: bool, // extend r/m
    rex_x: bool,
    rex_r: bool,
    rex_w: bool, // is 64-bit
    rex_used: bool,

    operand_16bit: bool,
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
            rex_used: false,
            rflags: 0,
            rip: 0,
            target_operand: TargetOperand::RegisterMemory(0),
            source_operand: SourceOperand::Memory(0),
            operand_16bit: false,
        };
        cpu.initialize();
        cpu
    }
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            bus,
            ..Default::default()
        }
    }

    pub fn initialize(&mut self) {
        /* Add */
        self.add_8bit_opcode(
            Opcode::Add,
            0x04,
            OperatorType::RAX,
            OperatorType::CanImmediate8,
        );
        self.add_opcode(
            Opcode::Add,
            0x05,
            OperatorType::RAX,
            OperatorType::CanImmediate16 | OperatorType::CanImmediate32,
        );
        self.add_8bit_opcode(
            Opcode::Add,
            0x80,
            OperatorType::FromModrmRM,
            OperatorType::CanImmediate8,
        );
        self.add_opcode(
            Opcode::Add,
            0x81,
            OperatorType::FromModrmRM,
            OperatorType::CanImmediate32,
        );
        self.add_8bit_opcode(
            Opcode::Add,
            0x83,
            OperatorType::FromModrmRM,
            OperatorType::CanImmediate8,
        );

        self.add_8bit_opcode(
            Opcode::Add,
            0x00,
            OperatorType::FromModrmRM,
            OperatorType::FromModrmREG,
        );
        self.add_opcode(
            Opcode::Add,
            0x01,
            OperatorType::FromModrmRM,
            OperatorType::FromModrmREG,
        );
        self.add_8bit_opcode(
            Opcode::Add,
            0x02,
            OperatorType::FromModrmREG,
            OperatorType::FromModrmRM,
        );
        self.add_opcode(
            Opcode::Add,
            0x03,
            OperatorType::FromModrmREG,
            OperatorType::FromModrmRM,
        );

        /* Mov */
        self.add_8bit_opcode(
            Opcode::Mov,
            0x88,
            OperatorType::FromModrmRM | OperatorType::Reg8,
            OperatorType::Reg8 | OperatorType::FromModrmREG,
        );
        self.add_opcode(
            Opcode::Mov,
            0x89,
            OperatorType::FromModrmRM | *OPERATOR_TYPE_REG_16_32_64,
            *OPERATOR_TYPE_REG_16_32_64 | OperatorType::FromModrmREG,
        );
        self.add_opcode(
            Opcode::Mov,
            0xc7,
            OperatorType::FromModrmRM | *OPERATOR_TYPE_REG_16_32_64,
            *OPERATOR_TYPE_IMM_16_32_64,
        );
        self.add_opcode_list(
            Opcode::Mov,
            0xb8,
            OperatorType::FromOpcode,
            *OPERATOR_TYPE_IMM_16_32_64,
        );
        self.add_8bit_opcode_list(
            Opcode::Mov,
            0xb0,
            OperatorType::FromOpcode,
            OperatorType::CanImmediate8,
        );

        self.add_opcode(Opcode::Nop, 0xF4, OperatorType::None, OperatorType::None);
        self.add_opcode(Opcode::Nop, 0x90, OperatorType::None, OperatorType::None);
    }

    fn add_opcode(
        &mut self,
        opcode: Opcode,
        code: u8,
        target_info: OperatorType,
        source_info: OperatorType,
    ) {
        self.add_opcode_format(opcode, code, false, target_info, source_info, false)
    }

    fn add_8bit_opcode(
        &mut self,
        opcode: Opcode,
        code: u8,
        target_info: OperatorType,
        source_info: OperatorType,
    ) {
        self.add_opcode_format(opcode, code, false, target_info, source_info, true)
    }

    fn add_opcode_list(
        &mut self,
        opcode: Opcode,
        code: u8,
        target_info: OperatorType,
        source_info: OperatorType,
    ) {
        self.add_opcode_format(opcode, code, true, target_info, source_info, false)
    }

    fn add_8bit_opcode_list(
        &mut self,
        opcode: Opcode,
        code: u8,
        target_info: OperatorType,
        source_info: OperatorType,
    ) {
        self.add_opcode_format(opcode, code, true, target_info, source_info, true)
    }

    fn add_opcode_format(
        &mut self,
        opcode: Opcode,
        code: u8,
        contain_reg: bool,
        target_info: OperatorType,
        source_info: OperatorType,
        _is8bit: bool,
    ) {
        if contain_reg {
            self.opcode_formats.insert(
                code,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 1,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 2,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 3,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 4,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 5,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 6,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
            self.opcode_formats.insert(
                code + 7,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
        } else {
            self.opcode_formats.insert(
                code,
                OpcodeFormat {
                    opcode,
                    target_info,
                    source_info,
                    _is8bit,
                },
            );
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
            r_m: match self.rex_b {
                true => (MODR_M_R_M & ins) | 0b0000_1000,
                false => MODR_M_R_M & ins,
            },
        }
    }

    #[inline(always)]
    fn sib(&mut self) -> Sib {
        let ins = self.fetch();

        Sib {
            scale: (SIB_SCALE & ins) >> 6,
            index: (SIB_INDEX & ins) >> 3,
            base: SIB_BASE & ins,
        }
    }

    #[inline(always)]
    fn opcode_reset(&mut self) {
        self.rex_b = false; // extend register code
        self.rex_x = false;
        self.rex_r = false;
        self.rex_w = false; // is 64-bit
        self.rex_used = false;

        self.operand_16bit = false;
    }

    #[allow(dead_code)]
    fn build_reg(&self, reg: &mut u8) {
        if self.rex_b {
            *reg |= 0b0000_1000;
        }
    }

    #[allow(dead_code)]
    fn read_rex_memory(&mut self) -> u64 {
        match self.rex_w {
            true => self.read_next64(),
            false => self.read_next32() as u64,
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

    fn process_sib_byte(
        &mut self,
        bit_mode: RegisterType,
        sib: Sib,
        modrm: &ModRM,
        target_info: OperatorType,
        source_info: OperatorType,
    ) {
        if sib.base == 0b0000_0101 {
            let address = self.read_next32() as u64;

            if target_info.intersects(OperatorType::FromModrmRM) {
                self.target_operand = TargetOperand::Memory(address);
            } else if source_info.intersects(OperatorType::FromModrmRM) {
                self.source_operand = SourceOperand::Memory(address);
            }

            if target_info.intersects(OperatorType::FromModrmREG) {
                self.target_operand = TargetOperand::Register(modrm.reg_opcode, bit_mode);
            } else if source_info.intersects(OperatorType::FromModrmREG) {
                self.source_operand = SourceOperand::Register(modrm.reg_opcode, bit_mode);
            }
        } else {
        }
    }

    fn build_target_operator(&self, operator: u8) -> u8 {
        match self.rex_b {
            true => operator | 0b0000_1000,
            false => operator,
        }
    }

    fn process(&mut self, byte: u8) -> Opcode {
        println!("Searching opcode: 0x{:X?}", byte);

        let OpcodeFormat {
            opcode,
            target_info,
            source_info,
            _is8bit,
        } = self.opcode_formats[&byte];

        let bit_mode = if _is8bit {
            RegisterType::_8Bit
        } else if self.operand_16bit {
            RegisterType::_16Bit
        } else if self.rex_w {
            RegisterType::_64Bit
        } else {
            RegisterType::_32Bit
        };

        if target_info.intersects(OperatorType::FromModrmRM.or(OperatorType::FromModrmREG))
            || source_info.intersects(OperatorType::FromModrmRM.or(OperatorType::FromModrmREG))
        {
            println!("Reg1 or reg2 has modrm");

            let modrm = self.modrm();
            match modrm.mod_ {
                0x00 |
                0x01 |
                0x10
                 => {

                    if modrm.r_m == 0b0000_0100 { // SIB calculation
                        println!("Need SIB opcode");
                        let sib = self.sib();
                        self.process_sib_byte(bit_mode, sib, &modrm, target_info, source_info);
                    } else if modrm.r_m == 0b0000_0101 { // RIP/EIP
                        println!("Need IMMEDIATE opcode")
                    } else {
                        if target_info.intersects(OperatorType::FromModrmREG) {
                            self.target_operand = TargetOperand::Register(modrm.reg_opcode, bit_mode);

                        } else if target_info.intersects(OperatorType::FromModrmRM) {
                            self.target_operand = TargetOperand::RegisterMemory(modrm.r_m);
                        }

                        if source_info.intersects(OperatorType::FromModrmREG) {
                            self.source_operand = SourceOperand::Register(modrm.reg_opcode, bit_mode);

                        } else if source_info.intersects(OperatorType::FromModrmRM) {
                            self.source_operand = SourceOperand::RegisterMemory(modrm.r_m);
                        }
                    }
                }

                0x03 /* 0011 */ => { // reg and reg
                    if target_info.intersects(OperatorType::FromModrmRM) {
                        self.target_operand = TargetOperand::Register(modrm.r_m, bit_mode);

                    } else if target_info.intersects(OperatorType::FromModrmREG) {
                        self.target_operand = TargetOperand::Register(modrm.reg_opcode, bit_mode);
                    }

                    if source_info.intersects(OperatorType::FromModrmRM) {
                        self.source_operand = SourceOperand::Register(modrm.r_m, bit_mode);

                    } else if source_info.intersects(OperatorType::FromModrmREG) {
                        self.source_operand = SourceOperand::Register(modrm.reg_opcode, bit_mode);
                    }
                }

                _ => todo!("not implemented (0x{:X?})", modrm.mod_)
            }
        }

        if target_info.intersects(OperatorType::FromOpcode) {
            self.target_operand =
                TargetOperand::Register(self.build_target_operator(0b0000_0111 & byte), bit_mode);
        }

        // TODO: maybe not need it
        if source_info.intersects(OperatorType::FromOpcode) {
            self.source_operand = SourceOperand::Register(0b0000_0111 & byte, bit_mode);
        }

        if source_info.intersects(OperatorType::CanImmediate8) && !self.operand_16bit {
            self.source_operand = SourceOperand::Immediate(self.read_next8() as u64)
        }

        if source_info.intersects(OperatorType::CanImmediate16) && self.operand_16bit {
            self.source_operand = SourceOperand::Immediate(self.read_next16() as u64)
        }

        if source_info.intersects(OperatorType::CanImmediate32) && !self.operand_16bit {
            self.source_operand = match self.rex_w {
                true => SourceOperand::Immediate(self.read_next64() as u64),
                false => SourceOperand::Immediate(self.read_next32() as u64),
            }
        }

        if target_info.intersects(OperatorType::RAX) {
            self.target_operand = TargetOperand::Register(self.build_target_operator(0), bit_mode);
        }

        opcode
    }

    fn get_source_operator(&mut self) -> u64 {
        match self.source_operand {
            SourceOperand::Register(source_register, bit_mode) => {
                if bit_mode == RegisterType::_8Bit {
                    if source_register > 3 && !self.rex_used {
                        let value = self.registers[(source_register % 4) as usize]; // 8bit high register
                        (value & 0x0000_0000_0000_ff00) >> 8 // We only need high bits
                    } else {
                        let value = self.registers[(source_register) as usize]; // 8bit low register
                        value & 0x0000_0000_0000_00ff // We only need low bits
                    }
                } else {
                    self.registers[source_register as usize]
                }
            }
            SourceOperand::Immediate(immediate) => immediate,
            SourceOperand::Memory(memory) => self.read64(memory),
            SourceOperand::RegisterMemory(pointer) => self.read64(self.registers[pointer as usize]),
        }
    }

    fn get_target_register(&mut self, register: u8, bit_mode: RegisterType) -> (u8, u64) {
        let mut register = register;
        let mut value = self.registers[register as usize];
        let value = match bit_mode {
            RegisterType::_8Bit => {
                match register > 3 && !self.rex_used {
                    true => {
                        register = register % 4; // First four register are real ones
                        value = self.registers[register as usize];
                        value
                    } // High byte
                    false => value, // Low byte
                }
            }
            RegisterType::_16Bit => value,
            RegisterType::_32Bit => value,
            RegisterType::_64Bit => value,
        };

        (register, value)
    }

    pub fn overflow_checked_add(
        &mut self,
        is_high_bits: bool,
        left: u64,
        right: u64,
        register_type: RegisterType,
    ) -> u64 {
        let (sum, overflowed) = match register_type {
            RegisterType::_8Bit => {
                match is_high_bits {
                    true => {
                        let (sum, overflowed) = (((left & 0x0000_0000_0000_ff00) >> 8) as u8)
                            .overflowing_add(right as u8);
                        (
                            (left & 0xffff_ffff_ffff_00ff) | ((sum as u64) << 8),
                            overflowed,
                        )
                    } // High byte
                    false => {
                        let (sum, overflowed) = (left as u8).overflowing_add(right as u8);
                        ((left & 0xffff_ffff_ffff_ff00) | sum as u64, overflowed)
                    } // Low byte
                }
            }
            RegisterType::_16Bit => {
                let (sum, overflowed) = (left as u16).overflowing_add(right as u16);
                ((left & 0xffff_ffff_ffff_0000) | sum as u64, overflowed)
            }
            RegisterType::_32Bit => {
                let (sum, overflowed) = (left as u32).overflowing_add(right as u32);
                ((left & 0xffff_ffff_0000_0000) | sum as u64, overflowed)
            }
            RegisterType::_64Bit => left.overflowing_add(right),
        };
        sum
    }

    pub fn move_data(
        &mut self,
        is_high_bits: bool,
        left: u64,
        right: u64,
        register_type: RegisterType,
    ) -> u64 {
        match register_type {
            RegisterType::_8Bit => {
                match is_high_bits {
                    true => (left & 0xffff_ffff_ffff_00ff) | ((right as u64) << 8), // High byte
                    false => (left & 0xffff_ffff_ffff_ff00) | right as u64,         // Low byte
                }
            }
            RegisterType::_16Bit => (left & 0xffff_ffff_ffff_0000) | right as u64,
            RegisterType::_32Bit => (left & 0xffff_ffff_0000_0000) | right as u64,
            RegisterType::_64Bit => right,
        }
    }

    pub fn execute(&mut self, opcode: u8) {
        let mut opcode = opcode;
        self.opcode_reset();

        // Rex opcode
        if (opcode & REX_MASK) == 0x40 {
            self.rex_b = opcode & 1 != 0; // extend register code
            self.rex_x = (opcode >> 1 & 1) != 0;
            self.rex_r = (opcode >> 2 & 1) != 0;
            self.rex_w = (opcode >> 3 & 1) != 0; // is 64-bit
            self.rex_used = true;
            opcode = self.fetch();
        }

        if opcode == OPERAND_SIZE_OVERWRITE_PREFIX {
            self.operand_16bit = true;
            opcode = self.fetch();
        }

        let opcode = self.process(opcode);

        match opcode {
            Opcode::Add => {
                let source_value = self.get_source_operator();

                match self.target_operand {
                    TargetOperand::RegisterMemory(_) => todo!(),
                    TargetOperand::Register(register, bit_mode) => {
                        let (new_register, target_value) =
                            self.get_target_register(register, bit_mode);
                        self.registers[new_register as usize] = self.overflow_checked_add(
                            register != new_register,
                            target_value,
                            source_value,
                            bit_mode,
                        )
                    }

                    TargetOperand::Memory(address) => {
                        let current = self.read64(address);
                        let sum = self.overflow_checked_add(
                            false,
                            current,
                            source_value,
                            RegisterType::_64Bit,
                        );
                        self.write64(address, sum)
                    }
                }
            }
            Opcode::Mov => {
                let source_value = self.get_source_operator();

                match self.target_operand {
                    TargetOperand::RegisterMemory(_) => todo!(),
                    TargetOperand::Register(register, bit_mode) => {
                        let (new_register, target_value) =
                            self.get_target_register(register, bit_mode);
                        self.registers[new_register as usize] = self.move_data(
                            register != new_register,
                            target_value,
                            source_value,
                            bit_mode,
                        )
                    }
                    TargetOperand::Memory(address) => self.write64(address, source_value),
                }
            }
            Opcode::Nop => self.rip = self.bus.len() as u64,
        };
    }

    /* Memory Functions */

    #[inline(always)]
    fn read_next8(&mut self) -> u8 {
        let value = self.bus.read8(self.rip as usize);
        self.rip += 1;
        value
    }

    #[inline(always)]
    fn read_next16(&mut self) -> u16 {
        let value = self.bus.read16(self.rip as usize);
        self.rip += 2;
        value
    }

    #[inline(always)]
    fn read_next32(&mut self) -> u32 {
        let value = self.bus.read32(self.rip as usize);
        self.rip += 4;
        value
    }

    #[inline(always)]
    fn read_next64(&mut self) -> u64 {
        let value = self.bus.read64(self.rip as usize);
        self.rip += 8;
        value
    }

    #[inline(always)]
    #[allow(dead_code)]
    fn read8(&mut self, address: u64) -> u8 {
        self.bus.read8(address as usize)
    }

    #[inline(always)]
    #[allow(dead_code)]
    fn read16(&mut self, address: u64) -> u16 {
        self.bus.read16(address as usize)
    }

    #[inline(always)]
    #[allow(dead_code)]
    fn read32(&mut self, address: u64) -> u32 {
        self.bus.read32(address as usize)
    }

    #[inline(always)]
    fn read64(&mut self, address: u64) -> u64 {
        self.bus.read64(address as usize)
    }

    fn write64(&mut self, address: u64, value: u64) {
        self.bus.write64(address as usize, value)
    }

    pub fn dump(&mut self) {
        let memory_len = self.bus.len() as u64;
        while self.rip < memory_len {
            let mut opcode = self.fetch();
            self.opcode_reset();

            // Rex opcode
            if (opcode & REX_MASK) == 0x40 {
                self.rex_b = opcode & 1 != 0; // extend register code
                self.rex_x = (opcode >> 1 & 1) != 0;
                self.rex_r = (opcode >> 2 & 1) != 0;
                self.rex_w = (opcode >> 3 & 1) != 0; // is 64-bit
                self.rex_used = true;
                opcode = self.fetch();
            }

            if opcode == OPERAND_SIZE_OVERWRITE_PREFIX {
                self.operand_16bit = true;
                opcode = self.fetch();
            }

            let opcode = self.process(opcode);

            match opcode {
                Opcode::Add => {
                    println!("add")
                }
                Opcode::Mov => {
                    println!("mov")
                }
                Opcode::Nop => {
                    println!("nop");
                    break;
                }
            }
        }
    }
}
