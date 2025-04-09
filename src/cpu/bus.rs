use super::memory::Memory;

#[derive(Debug, Clone, Default)]
pub struct Bus {
    memory: Memory,
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
    #[allow(dead_code)]
    pub fn write8(&mut self, address: usize, value: u8) {
        self.memory.write8(address, value)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn read16(&self, address: usize) -> u16 {
        self.memory.read16(address)
    }

    #[inline(always)]
    #[allow(dead_code)]
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
