use core::str;
use std::{ffi::CStr, fs};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{BinaryFormat, BinaryFormatError, BitMode, BufferReader, Endianness};

#[repr(u32)]
#[derive(Debug, Default)]
pub enum CpuType {
    #[default]
    VAX = 0x00000001,
    ROMP = 0x00000002,
    NS32032 = 0x00000004,
    NS32332 = 0x00000005,
    MC680x0 = 0x00000006,
    x86 = 0x00000007,
    MIPS = 0x00000008,
    NS32352 = 0x00000009,
    MC98000 = 0x0000000A,
    HP_PA = 0x0000000B,
    ARM = 0x0000000C,
    MC88000 = 0x0000000D,
    SPARC = 0x0000000E,
    i860_big_endian = 0x0000000F,
    i860_little_endian = 0x00000010,
    RS_6000 = 0x00000011,
    PowerPC = 0x00000012,
}

#[repr(u32)]
#[derive(Debug, Default)]
pub enum FileType {
    #[default]
    object_file = 0x00000001,
    executable_file = 0x00000002,
    shared_library_file = 0x00000003,
    core_file = 0x00000004,
    preloaded_executable_file = 0x00000005,
    dynamically_shared_library_file = 0x00000006,
    dynamic_link_editor = 0x00000007,
    dynamically_bound_bundle_file = 0x00000008,
    shared_library_stub = 0x00000009,
    with_only_debug_sections = 0x0000000A,
    x86_64_kexts = 0x0000000B,
    composed_file = 0x0000000C,

}

#[repr(u32)]
#[derive(TryFromPrimitive)]
#[derive(Debug, Default)]
pub enum LoadCommandType {
    #[default]
    Segment = 0x00000019
}

#[derive(Debug, Default)]
pub struct LoadCommand {
    pub command_type: LoadCommandType,
    pub command_size: u32,
}

#[derive(Debug, Default)]
pub struct MachOHeader {
    pub bit_mode: BitMode,
    pub magic_number: u32,
    pub cpu_type: CpuType,
    pub cpu_subtype: u32,
    pub file_type: FileType,
    pub number_of_load_commands: u32,
    pub size_of_load_commands: u32,
    pub flags: u32,

    pub load_commands: Vec<LoadCommand>
}

impl MachOHeader {
    pub fn build(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn parse(&mut self, reader: &mut BufferReader) -> Result<(), BinaryFormatError> {
        self.magic_number = reader.fetch_u32()?;
        self.bit_mode = match self.magic_number {
            0xfeedface => BitMode::_32,
            0xfeedfacf => BitMode::_64,
            _ => return Err(BinaryFormatError::InvalidFormat)
        };
        self.cpu_type = unsafe { core::mem::transmute::<u32, CpuType>((reader.fetch_u32()? << 8) >> 8) };
        self.cpu_subtype = reader.fetch_u32()?;
        self.file_type = unsafe { core::mem::transmute::<u32, FileType>(reader.fetch_u32()?) };
        self.number_of_load_commands = reader.fetch_u32()?;
        self.size_of_load_commands = reader.fetch_u32()?;
        self.flags = reader.fetch_u32()?;

        if self.bit_mode == BitMode::_64 {
            reader.fetch_u32();
        }

        for _ in 0..self.number_of_load_commands {
            let command_type = LoadCommandType::try_from(reader.fetch_u32()?).map_err(|_| BinaryFormatError::InvalidType)?;

            if let command_type = LoadCommandType::Segment {
                let command_size = reader.fetch_u32()?;
                let segment_name = reader.fetch_slice(8, 8 + 16);
                let address = reader.parse_size(BitMode::_64);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct MachOFormat<'a> {
    pub header: MachOHeader,
    pub buffer: &'a [u8]
}

impl<'a> BinaryFormat<'a> for MachOFormat<'a> {
    fn parse(reader: &'a mut BufferReader) -> Result<Self, super::BinaryFormatError> where Self: Sized {
        let mut header = MachOHeader::default();
        
        header.parse(reader)?;

        Ok(Self { header, buffer: reader.read_remaining() })        
    }

    fn get_codes(&self) -> &'a [u8] {
        self.buffer
    }
}
