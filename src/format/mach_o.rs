use core::str;
use std::{ffi::CStr, fs};

use super::{BitMode, BufferReader, Endianness};

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

#[derive(Debug, Default)]
pub struct LoadCommand {
    pub command_type: u32,
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

    pub fn parse(&mut self, reader: &mut BufferReader) {
        self.magic_number = reader.fetch_u32();
        self.bit_mode = match self.magic_number {
            0xfeedface => BitMode::_32,
            0xfeedfacf => BitMode::_64,
            _ => panic!("Unknown format")
        };
        self.cpu_type = unsafe { core::mem::transmute::<u32, CpuType>((reader.fetch_u32() << 8) >> 8) };
        self.cpu_subtype = reader.fetch_u32();
        self.file_type = unsafe { core::mem::transmute::<u32, FileType>(reader.fetch_u32()) };
        self.number_of_load_commands = reader.fetch_u32();
        self.size_of_load_commands = reader.fetch_u32();
        self.flags = reader.fetch_u32();

        if self.bit_mode == BitMode::_64 {
            reader.fetch_u32();
        }

        for _ in 0..self.number_of_load_commands {
            self.load_commands.push(LoadCommand {
                command_type: reader.fetch_u32(),
                command_size: reader.fetch_u32()
            })
        }
    }
}

#[derive(Debug, Default)]
pub struct MachOFormat {
    pub header: MachOHeader
}

impl MachOFormat {
    pub fn parse(contents: &[u8]) -> Self {
        let mut header = MachOHeader::default();
        let mut reader = BufferReader::new(contents);

        header.parse(&mut reader);

        Self { header }
    }
}

pub fn parse(filename: &str) {
    let contents = fs::read(filename).expect("Should have been able to read the file");

    let elf = MachOFormat::parse(&contents);
    println!("Elf :{:#?}", elf);
}
