use core::str;

use crate::format::str_from_null_terminated_utf8;

use super::{BinaryFormat, BinaryFormatError, BitMode, BufferReader, Endianness, Size};


/* Enums */
#[repr(u32)]
#[derive(Debug, Default)]
pub enum Segment {
    #[default]
    None = 0x00000000,
    Load = 0x00000001,
    Dynamic = 0x00000002,
    Interp = 0x00000003,
    Note = 0x00000004,
    Shlib = 0x00000005,
    ShlibPhdr = 0x00000006,
    Tls = 0x00000007,
}

#[repr(u16)]
#[derive(Debug, Default)]
pub enum ElfType {
    #[default]
    None = 0x00,
    Relocatable = 0x01,
    Executable = 0x02,
    Shared = 0x03,
    Core = 0x04,
}

#[repr(u16)]
#[derive(Debug, Default)]
pub enum ISA {
    #[default]
    None = 0x00,
    Sparc = 0x02,
    x86 = 0x03,
    MIPS = 0x08,
    PowerPC = 0x14,
    ARM = 0x28,
    SuperH = 0x2A,
    IA_64 = 0x32,
    x86_64 = 0x3E,
    AArch64 = 0xB7,
    RISCV = 0xF3,
}

#[repr(u8)]
#[derive(Debug, Default)]
pub enum OsAbi {
    #[default]
    SystemV = 0x00,
    HP_UX = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GNU_Hurd = 0x04,
    Solaris = 0x06,
    AIX = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D,
    NonStopKernel = 0x0E,
    AROS = 0x0F,
    FenixOS = 0x10,
    NuxiCloudABI = 0x11,
    StratusTechnologiesOpenVOS = 0x12,
}

#[repr(u32)]
#[derive(Debug, Default)]
pub enum SectionHeaderType {
    #[default]
    SHT_NULL = 0x0, // Section header table entry unused 
    SHT_PROGBITS = 0x1, // Program data 
    SHT_SYMTAB = 0x2, // Symbol table 
    SHT_STRTAB = 0x3, // String table 
    SHT_RELA = 0x4, // Relocation entries with addends 
    SHT_HASH = 0x5, // Symbol hash table 
    SHT_DYNAMIC = 0x6, // Dynamic linking information 
    SHT_NOTE = 0x7, // Notes 
    SHT_NOBITS = 0x8, // Program space with no data (bss) 
    SHT_REL = 0x9, // Relocation entries, no addends 
    SHT_SHLIB = 0x0A, // Reserved 
    SHT_DYNSYM = 0x0B, // Dynamic linker symbol table 
    SHT_INIT_ARRAY = 0x0E, // Array of constructors 
    SHT_FINI_ARRAY = 0x0F, // Array of destructors 
    SHT_PREINIT_ARRAY = 0x10, // Array of pre-constructors 
    SHT_GROUP = 0x11, // Section group 
    SHT_SYMTAB_SHNDX = 0x12, // Extended section indices 
    SHT_NUM = 0x13, // Number of defined types. 
    SHT_LOOS = 0x60000000, // Start OS-specific. 
}

#[repr(u64)]
#[derive(Debug, Default)]
pub enum SectionHeaderFlag {
    #[default]
    SHF_WRITE = 0x1, // Writable 
    SHF_ALLOC = 0x2, // Occupies memory during execution 
    SHF_EXECINSTR = 0x4, // Executable 
    SHF_MERGE = 0x10, // Might be merged 
    SHF_STRINGS = 0x20, // Contains null-terminated strings 
    SHF_INFO_LINK = 0x40, // 'sh_info' contains SHT index 
    SHF_LINK_ORDER = 0x80, // Preserve order after combining 
    SHF_OS_NONCONFORMING = 0x100, // Non-standard OS specific handling required 
    SHF_GROUP = 0x200, // Section is member of a group 
    SHF_TLS = 0x400, // Section hold thread-local data 
    SHF_MASKOS = 0x0FF00000, // OS-specific 
    SHF_MASKPROC = 0xF0000000, // Processor-specific 
    SHF_ORDERED = 0x4000000, // Special ordering requirement (Solaris) 
    SHF_EXCLUDE = 0x8000000, // Section is excluded unless referenced or allocated (Solaris) 
    
}

/* Enums */

#[derive(Debug, Default)]
pub struct ElfProgramHeader {
    pub segment: Segment,
    pub flags: u32,
    pub offset: Size,
    pub virtual_address: Size,
    pub physical_address: Size,
    pub p_filesz: Size,
    pub p_memsz: Size,
    pub p_align: Size,
}

impl ElfProgramHeader {
    pub fn parse(&mut self, bit_mode: BitMode, reader: & mut BufferReader) -> Result<(), BinaryFormatError> {
        self.segment = unsafe { core::mem::transmute::<u32, Segment>(reader.fetch_u32()?) };

        if BitMode::_64 == bit_mode {
            self.flags = reader.fetch_u32()?;
        }

        self.offset = reader.parse_size(bit_mode)?;
        self.virtual_address = reader.parse_size(bit_mode)?;
        self.physical_address = reader.parse_size(bit_mode)?;
        self.p_filesz = reader.parse_size(bit_mode)?;
        self.p_memsz = reader.parse_size(bit_mode)?;
        
        if BitMode::_32 == bit_mode {
            self.flags = reader.fetch_u32()?;
        }
        self.p_align = reader.parse_size(bit_mode)?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ElfSectionHeader<'a> {
    pub name: &'a str,
    pub sh_name: u32,
    pub sh_type: SectionHeaderType,
    pub sh_flags: SectionHeaderFlag,
    pub sh_offset: Size,
    pub sh_addr: Size,
    pub sh_size: Size,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: Size,
    pub sh_entsize: Size
}

impl<'a> ElfSectionHeader<'a> {
    pub fn parse(&mut self, bit_mode: BitMode, reader: & mut BufferReader) -> Result<(), BinaryFormatError> {
        self.sh_name = reader.fetch_u32()?;
        self.sh_type = unsafe { core::mem::transmute::<u32, SectionHeaderType>(reader.fetch_u32()?) };
        self.sh_flags = unsafe { core::mem::transmute::<u64, SectionHeaderFlag>(match bit_mode {
                BitMode::_32 => reader.fetch_u32()? as u64,
                BitMode::_64 => reader.fetch_u64()?
            })
        };
        self.sh_addr = reader.parse_size(bit_mode)?;
        self.sh_offset = reader.parse_size(bit_mode)?;
        self.sh_size = reader.parse_size(bit_mode)?;
        self.sh_link = reader.fetch_u32()?;
        self.sh_info = reader.fetch_u32()?;
        self.sh_addralign = reader.parse_size(bit_mode)?;
        self.sh_entsize = reader.parse_size(bit_mode)?;
        Ok(())
    }
}


#[derive(Debug, Default)]
pub struct ElfHeader {
    pub bit_mode: BitMode,
    pub endianness: Endianness,
    pub version: u8,
    pub e_type: ElfType,
    pub os_abi: OsAbi,
    pub e_machine: ISA,
    pub e_version: u32,
    pub e_entry: Size,
    pub e_phoff: Size,
    pub e_shoff: Size,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
}

impl ElfHeader {
    pub fn build(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn parse(&mut self, reader: &mut BufferReader) -> Result<(), BinaryFormatError> {

        if reader.fetch_u8()? != 0x7f || reader.fetch_u8()? != 0x45 || reader.fetch_u8()? != 0x4c || reader.fetch_u8()? != 0x46 {
            return Err(BinaryFormatError::InvalidFormat);
        }

        self.bit_mode = match reader.fetch_u8()? {
            1 => BitMode::_32,
            _ => BitMode::_64,
        };

        self.endianness = match reader.fetch_u8()? {
            1 => Endianness::Little,
            _ => Endianness::Big,
        };
        
        self.version = reader.fetch_u8()?;
        self.os_abi = unsafe { core::mem::transmute::<u8, OsAbi>(reader.fetch_u8()?) };

        reader.set_index(16)?;
        self.e_type = unsafe { core::mem::transmute::<u16, ElfType>(reader.fetch_u16()?) };
        self.e_machine = unsafe { core::mem::transmute::<u16, ISA>(reader.fetch_u16()?) };
        self.e_version = reader.fetch_u32()?;

        reader.set_index(24)?;
        self.e_entry = reader.parse_size(self.bit_mode)?;
        self.e_phoff = reader.parse_size(self.bit_mode)?;
        self.e_shoff = reader.parse_size(self.bit_mode)?;

        self.e_flags = reader.fetch_u32()?;
        self.e_ehsize = reader.fetch_u16()?;
        self.e_phentsize = reader.fetch_u16()?;
        self.e_phnum = reader.fetch_u16()?;
        self.e_shentsize = reader.fetch_u16()?;
        self.e_shnum = reader.fetch_u16()?;
        self.e_shstrndx = reader.fetch_u16()?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ElfRela {
    pub offset: u64,
    pub info: u64,
    pub addend: u64
}

#[derive(Debug, Default)]
pub struct ElfFormat<'a> {
    pub elf_header: ElfHeader,
    pub program_header: ElfProgramHeader,
    pub section_headers: Vec<ElfSectionHeader<'a>>,
    pub codes: &'a [u8]
}

impl<'a> BinaryFormat<'a> for ElfFormat<'a> {
    fn parse(reader: &'a mut BufferReader) -> Result<Self, BinaryFormatError> {
        let mut elf_header = ElfHeader::default();
        let mut program_header = ElfProgramHeader::default();
        let mut section_headers = Vec::default();

        elf_header.parse(reader)?;

        reader.set_index(match elf_header.e_phoff {
            Size::None => todo!("elf_header could not parsed"),
            Size::u32(size) => size as usize,
            Size::u64(size) => size as usize,
        })?;

        program_header.parse(elf_header.bit_mode, reader)?;

        /* Lets do calculation about sh_offset */
        reader.set_index(usize::from(elf_header.e_shoff) + (elf_header.e_shentsize * elf_header.e_shstrndx) as usize + match elf_header.bit_mode {
            BitMode::_32 => 0x10,
            BitMode::_64 => 0x18
        })?;

        let string_offset = match elf_header.bit_mode {
            BitMode::_32 => reader.fetch_u32()? as usize,
            BitMode::_64 => reader.fetch_u64()? as usize
        };

        reader.set_index(string_offset)?;
        let string_data = reader.read_remaining();
        
        /* Parse section headers */
        reader.set_index(elf_header.e_shoff.into())?;

        for _ in 0..elf_header.e_shnum {
            let mut section_header = ElfSectionHeader::default();
            section_header.parse(elf_header.bit_mode, reader)?;
            section_header.name = unsafe { str_from_null_terminated_utf8(&string_data[(section_header.sh_name as usize)..])? };
            println!("Section: {:#?}", &section_header);

            section_headers.push(section_header);
        }

        let text_section = section_headers.iter().find(|section| section.name == ".text");
        let codes = match text_section {
            Some(section) => {
                reader.set_index(section.sh_offset.into())?;
                let size: usize = section.sh_size.into();
                let machine_codes = reader.read_remaining();

                println!("Text offset {}", usize::from(section.sh_offset));
                println!("Text section {:#?}", &section);
                let text_relas = Self::find_rela(&section_headers, elf_header.bit_mode, ".rela.text", reader)?;

                &machine_codes[0..size]
            }
            None => return Err(BinaryFormatError::NoCode)
        };

        Ok(Self { elf_header, program_header, section_headers, codes })
    }
    
    fn get_codes(&self) -> &'a [u8] {
        self.codes
    }
}


impl<'a> ElfFormat<'_> {
    fn find_rela(section_headers: &Vec<ElfSectionHeader>, bit_mode: BitMode, name: &str, reader: &mut BufferReader) -> Result<Option<Vec<ElfRela>>, BinaryFormatError> {
        let rela_section = section_headers.iter().find(|section| section.name == name);
        match rela_section {
            Some(section) => {
                reader.set_index(section.sh_offset.into())?;
                let entry_count = usize::from(section.sh_size) / usize::from(section.sh_entsize);
                let mut relas = Vec::new();

                for _ in 0..entry_count {
                    let rela = ElfRela {
                        offset: reader.fetch_u64()?,
                        info: reader.fetch_u64()?,
                        addend: reader.fetch_u64()?
                    };

                    let data = ((rela.info << 32) >> 40);
                    let id = ((rela.info << 56) >> 56);

                    let sym = rela.info >> 32;
                    let type_ = rela.info as u32;

                    relas.push(rela);
                }

                Ok(Some(relas))
            }
            None => Ok(None)
        }
    }
}