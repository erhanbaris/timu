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
        self.data[address] = value as u8;
        self.data[address + 1] = (value >> 8) as u8;
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
        self.data[address] = value as u8;
        self.data[address + 1] = (value >> 8) as u8;
        self.data[address + 2] = (value >> (2 * 8)) as u8;
        self.data[address + 3] = (value >> (3 * 8)) as u8;
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
        self.data[address] = value as u8;
        self.data[address + 1] = (value >> 8) as u8;
        self.data[address + 2] = (value >> (2 * 8)) as u8;
        self.data[address + 3] = (value >> (3 * 8)) as u8;
        self.data[address + 4] = (value >> (4 * 8)) as u8;
        self.data[address + 5] = (value >> (5 * 8)) as u8;
        self.data[address + 6] = (value >> (6 * 8)) as u8;
        self.data[address + 7] = (value >> (7 * 8)) as u8;
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

    #[allow(dead_code)]
    pub fn write16(&mut self, value: u16) {
        self.memory.write16(self.index, value);
        self.index += 2;
    }
    
    #[allow(dead_code)]
    pub fn write32(&mut self, value: u32) {
        self.memory.write32(self.index, value);
        self.index += 4;
    }

    #[allow(dead_code)]
    pub fn write64(&mut self, value: u64) {
        self.memory.write64(self.index, value);
        self.index += 8;
    }
}