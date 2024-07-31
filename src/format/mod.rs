use std::ffi::CStr;

pub mod elf;
pub mod mach_o;

#[derive(Debug, Default)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum BitMode {
    #[default]
    _32,
    _64,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, Default)]
pub enum Size {
    #[default]
    None = 0x00,
    u32(u32),
    u64(u64),
}

impl From<Size> for usize {
    fn from(value: Size) -> Self {
        match value {
            Size::None => todo!("Size could not parsed"),
            Size::u32(size) => size as usize,
            Size::u64(size) => size as usize,
        }
    }
}

#[derive(Debug, Default)]
pub struct BufferReader<'a> {
    pub data: &'a [u8],
    pub index: usize
}

impl<'a> BufferReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, index: 0 }
    }

    pub fn as_u8(&mut self) -> u8 {
        let value = self.data[self.index];
        self.index += 1;
        value
    }

    pub fn as_u16(&mut self) -> u16 {
        let value = ((self.data[self.index + 1] as u16) << 8) | self.data[self.index] as u16;
        self.index += 2;
        value
    }

    pub fn as_u32(&mut self) -> u32 {
        let value = (
               (self.data[self.index + 3] as u32) << 24)
            | ((self.data[self.index + 2] as u32) << 16)
            | ((self.data[self.index + 1] as u32) << 8)
            | self.data[self.index] as u32;
        self.index += 4;
        value
    }

    pub fn as_u64(&mut self) -> u64 {
        let value = (
               (self.data[self.index + 7] as u64) << 56)
            | ((self.data[self.index + 6] as u64) << 48)
            | ((self.data[self.index + 5] as u64) << 40)
            | ((self.data[self.index + 4] as u64) << 32)
            | ((self.data[self.index + 3] as u64) << 24)
            | ((self.data[self.index + 2] as u64) << 16)
            | ((self.data[self.index + 1] as u64) << 8)
            | self.data[self.index] as u64;
        self.index += 8;
        value
    }

    fn parse_size(&mut self, bit_mode: BitMode) -> Size {
        let address = match bit_mode {
            BitMode::_32 => Size::u32(self.as_u32()),
            BitMode::_64 => Size::u64(self.as_u64()),
        };
        address
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn read_remaining(&mut self) -> &'a[u8] {
        &self.data[self.index..]
    }
}

unsafe fn str_from_null_terminated_utf8(s: &[u8]) -> &str {
    CStr::from_ptr(s.as_ptr() as *const _).to_str().unwrap()
}
