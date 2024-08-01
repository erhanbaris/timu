use std::ffi::CStr;

pub mod elf;
// pub mod mach_o;

use elf::ElfFormat;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinaryFormatError {
    #[error("Binary format not valid")]
    MalformedBinary(#[from] std::str::Utf8Error),

    #[error("Invalid format")]
    InvalidFormat,

    #[error("Out of range")]
    OutOfRange,

    #[error("No code found")]
    NoCode,

    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),

    #[error("unknown error")]
    Unknown,
}

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

    pub fn fetch_u8(&mut self) -> Result<u8, BinaryFormatError> {
        let value = self.data[self.index];
        self.index += 1;
        Ok(value)
    }

    pub fn fetch_u16(&mut self) -> Result<u16, BinaryFormatError> {
        let value = ((self.data[self.index + 1] as u16) << 8) | self.data[self.index] as u16;
        self.index += 2;
        Ok(value)
    }

    pub fn fetch_u32(&mut self) -> Result<u32, BinaryFormatError> {
        let value = (
               (self.data[self.index + 3] as u32) << 24)
            | ((self.data[self.index + 2] as u32) << 16)
            | ((self.data[self.index + 1] as u32) << 8)
            | self.data[self.index] as u32;
        self.index += 4;
        Ok(value)
    }

    pub fn fetch_u64(&mut self) -> Result<u64, BinaryFormatError> {
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
        Ok(value)
    }

    pub fn fetch_i64(&mut self) -> Result<i64, BinaryFormatError> {
        let value = (
               (self.data[self.index + 7] as i64) << 56)
            | ((self.data[self.index + 6] as i64) << 48)
            | ((self.data[self.index + 5] as i64) << 40)
            | ((self.data[self.index + 4] as i64) << 32)
            | ((self.data[self.index + 3] as i64) << 24)
            | ((self.data[self.index + 2] as i64) << 16)
            | ((self.data[self.index + 1] as i64) << 8)
            | self.data[self.index] as i64;
        self.index += 8;
        Ok(value)
    }

    fn parse_size(&mut self, bit_mode: BitMode) -> Result<Size, BinaryFormatError> {
        let address = match bit_mode {
            BitMode::_32 => Size::u32(self.fetch_u32()?),
            BitMode::_64 => Size::u64(self.fetch_u64()?),
        };
        Ok(address)
    }

    pub fn set_index(&mut self, index: usize) -> Result<(), BinaryFormatError> {

        if index >= self.data.len() {
            Err(BinaryFormatError::OutOfRange)
        } else {
            self.index = index;
            Ok(())
        }
    }

    pub fn read_remaining(&mut self) -> &'a[u8] {
        &self.data[self.index..]
    }
}

unsafe fn str_from_null_terminated_utf8(s: &[u8]) -> Result<&str, std::str::Utf8Error> {
    CStr::from_ptr(s.as_ptr() as *const _).to_str()
}

pub struct BinaryRepresentation<'a> {
    pub codes: &'a [u8]
}

pub trait BinaryFormat<'a> {
    fn parse(reader: &'a mut BufferReader) -> Result<Self, BinaryFormatError> where Self: Sized;
    fn get_codes(&self) -> &'a [u8];
}

pub fn parse(filename: &str) -> Vec<u8>  {
    let contents = std::fs::read(filename).expect("Should have been able to read the file");
    let mut reader = BufferReader::new(&contents[..]);
    let binary = ElfFormat::parse(&mut reader).unwrap();
    //println!("Elf :{:#?}", &binary);

    binary.get_codes().to_vec()
    //let elf = ElfFormat::parse(&contents).unwrap();
}
