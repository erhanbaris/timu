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

struct ModRM {
    pub mod_: u8,
    pub reg_opcode: u8,
    pub r_m: u8
}

struct OpcodeFormat {
    pub name: &'static str,
    pub code: u8,
    pub contain_reg: bool,
    pub has_modrm: bool
}

#[derive(Debug, Clone, Default)]
pub struct Cpu {
    pub rip: u64, // Instruction pointer
    pub rflags: u64, // Flag registers

    pub registers: [u64; 16],
    pub bus: Bus,

    rex_b: bool, // extend r/m
    rex_x: bool,
    rex_r: bool,
    rex_w: bool, // is 64-bit
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let mut cpu = Self::default();
        cpu.bus = bus;
        cpu
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
    fn read8(&mut self, address: usize) -> u8 {
        let value = self.bus.read8(self.rip as usize);
        self.rip += 1;
        value
    }

    #[inline(always)]
    fn read16(&mut self, address: usize) -> u16 {
        let value = self.bus.read16(self.rip as usize);
        self.rip += 2;
        value
    }

    #[inline(always)]
    fn read32(&mut self, address: usize) -> u32 {
        let value = self.bus.read32(self.rip as usize);
        self.rip += 4;
        value
    }

    #[inline(always)]
    fn read64(&mut self, address: usize) -> u64 {
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
            true => self.read64(self.rip as usize),
            false => self.read32(self.rip as usize) as u64
        }
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
        
        match opcode {
            0 => {
                // Empty
            }

            0x01 => { // Add
                let modrm = self.modrm();
                match modrm.mod_ {
                    0x03 /* 0011 */ => {
                        if self.rex_w {
                            self.registers[modrm.reg_opcode as usize] += self.registers[modrm.r_m as usize]
                        } else {
                            self.registers[modrm.reg_opcode as usize] = (self.registers[modrm.reg_opcode as usize] as u32 + self.registers[modrm.r_m as usize] as u32) as u64
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

                self.registers[reg as usize] = self.read_rex_memory();
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
                            self.registers[modrm.r_m as usize] += self.read64(self.rip as usize)
                        } else {
                            self.registers[modrm.r_m as usize] = (self.registers[modrm.r_m as usize] as u32 + self.read32(self.rip as usize)) as u64
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

#[derive(Debug, Clone, Default)]
pub struct Bus {
    memory: Memory
}

impl Bus {
    pub fn new(memory: Memory) -> Self {
        Self { memory }
    }
    
    pub fn len(&self) -> usize {
        self.memory.len()
    }

    #[inline(always)]
    pub fn read8(&self, address: usize) -> u8 {
        self.memory.read8(address)
    }

    #[inline(always)]
    pub fn write8(&mut self, address: usize, value: u8) {
        self.memory.write8(address, value)
    }

    #[inline(always)]
    pub fn read16(&self, address: usize) -> u16 {
        self.memory.read16(address)
    }

    #[inline(always)]
    pub fn write16(&mut self, address: usize, value: u16) {
        self.memory.write16(address, value)
    }

    #[inline(always)]
    pub fn read32(&self, address: usize) -> u32 {
        self.memory.read32(address)
    }

    #[inline(always)]
    pub fn write32(&mut self, address: usize, value: u32) {
        self.memory.write32(address, value)
    }

    #[inline(always)]
    pub fn read64(&self, address: usize) -> u64 {
        self.memory.read64(address)
    }

    #[inline(always)]
    pub fn write64(&mut self, address: usize, value: u64) {
        self.memory.write64(address, value)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Memory {
    data: Vec<u8>,
    len: usize
}

impl Memory {
    pub fn new(len: usize) -> Self {
        let mut memory = Self {
            data: Vec::new(),
            len
        };
        memory.data.resize(len, 0);
        memory
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    #[inline(always)]
    pub fn read8(&self, address: usize) -> u8 {
        self.data[address]
    }

    #[inline(always)]
    pub fn write8(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    #[inline(always)]
    pub fn read16(&self, address: usize) -> u16 {
        let mut value: u16 = 0;
        value |= self.data[address] as u16;
        value |= (self.data[address + 1] as u16) << 8;
        value
    }

    #[inline(always)]
    pub fn write16(&mut self, address: usize, value: u16) {
        self.data[address] = (value as u8) & 0xFF;
        self.data[address + 1] = (value >> 8) as u8 & 0xFF;
    }

    #[inline(always)]
    pub fn read32(&self, address: usize) -> u32 {
        let mut value: u32 = 0;
        value |= self.data[address] as u32;
        value |= (self.data[address + 1] as u32) << 8;
        value |= (self.data[address + 2] as u32) << (2 * 8);
        value |= (self.data[address + 3] as u32) << (3 * 8);
        value
    }

    #[inline(always)]
    pub fn write32(&mut self, address: usize, value: u32) {
        self.data[address] = (value as u8) & 0xFF;
        self.data[address + 1] = (value >> 8) as u8 & 0xFF;
        self.data[address + 2] = (value >> (2 * 8)) as u8 & 0xFF;
        self.data[address + 3] = (value >> (3 * 8)) as u8 & 0xFF;
    }

    #[inline(always)]
    pub fn read64(&self, address: usize) -> u64 {
        let mut value: u64 = 0;
        value |= self.data[address] as u64;
        value |= (self.data[address + 1] as u64) << 8;
        value |= (self.data[address + 2] as u64) << (2 * 8);
        value |= (self.data[address + 3] as u64) << (3 * 8);
        value |= (self.data[address + 4] as u64) << (4 * 8);
        value |= (self.data[address + 5] as u64) << (5 * 8);
        value |= (self.data[address + 6] as u64) << (6 * 8);
        value |= (self.data[address + 7] as u64) << (7 * 8);
        value
    }

    #[inline(always)]
    pub fn write64(&mut self, address: usize, value: u64) {
        self.data[address] = (value as u8) & 0xFF;
        self.data[address + 1] = (value >> 8) as u8 & 0xFF;
        self.data[address + 2] = (value >> (2 * 8)) as u8 & 0xFF;
        self.data[address + 3] = (value >> (3 * 8)) as u8 & 0xFF;
        self.data[address + 4] = (value >> (4 * 8)) as u8 & 0xFF;
        self.data[address + 5] = (value >> (5 * 8)) as u8 & 0xFF;
        self.data[address + 6] = (value >> (6 * 8)) as u8 & 0xFF;
        self.data[address + 7] = (value >> (7 * 8)) as u8 & 0xFF;
    }
}


#[derive(Debug, Clone, Default)]
pub struct MemoryBuilder {
    memory: Memory,
    index: usize
}

impl MemoryBuilder {
    pub fn new(len: usize) -> Self {
        Self {
            memory: Memory::new(len),
            index: 0
        }
    }

    pub fn generate(self) -> Memory {
        self.memory
    }

    pub fn write8(&mut self, value: u8) {
        self.memory.write8(self.index, value);
        self.index += 1;
    }

    pub fn write16(&mut self, value: u16) {
        self.memory.write16(self.index, value);
        self.index += 2;
    }

    pub fn write32(&mut self, value: u32) {
        self.memory.write32(self.index, value);
        self.index += 4;
    }

    pub fn write64(&mut self, value: u64) {
        self.memory.write64(self.index, value);
        self.index += 8;
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{REGISTER_R10, REGISTER_R9, REGISTER_RAX, REGISTER_RCX};

    use super::{Bus, Cpu, MemoryBuilder};

    #[test]
    fn rex_1() {
        let mut memory = MemoryBuilder::new(100);
        /* mov $16, %rax */
        memory.write8(0x48);
        memory.write8(0xB8);
        memory.write64(0x10);
    
        /* mov $16, %rcx */
        memory.write8(0x48);
        memory.write8(0xB9);
        memory.write64(0x05);
    
        /* add %eax, %ecx */
        memory.write8(0x01);
        memory.write8(0xC8);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_RAX], 16);
        assert_eq!(cpu.registers[REGISTER_RCX], 21);
    }

    #[test]
    fn rex_2() {
        let mut memory = MemoryBuilder::new(100);
        /* mov $16, %r10d */
        memory.write8(0x41);
        memory.write8(0xB8);
        memory.write64(0x10);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();
        
        assert_eq!(cpu.registers[8], 0x10);
    }

    #[test]
    fn max_u64_check() {
        let mut memory = MemoryBuilder::new(100);
        /* mov $16, %r10d */
        memory.write8(0x49);
        memory.write8(0xC7);
        memory.write8(0xC2);
        memory.write64(u64::MAX);
    
        let bus = Bus::new(memory.generate());
        let mut cpu = Cpu::new(bus);
        cpu.boot();
        
        assert_eq!(cpu.registers[REGISTER_R10], u64::MAX);
    }
}