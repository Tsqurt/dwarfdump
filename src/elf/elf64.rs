// content for mod elf::elf64

#[derive(Debug)]
pub struct ElfHeader {
    // 0x7f, 'E', 'L', 'F'; a.k.a. (little-endian) 0x46454c7f
    magic_number: [u8; 4],

    // "32-bit" or "64-bit"
    architecture: String,

    // "little-endian" or "big-endian"
    data_encoding: String, 

    // "none" or "current"
    elf_version: String,

    // "System V" or "HP-UX" or "NetBSD" or "Linux" 
    // or "Solaris" or "AIX" or "IRIX" or "FreeBSD" or 
    // "Tru64" or "Modesto" or "OpenBSD" or "OpenVMS" or
    //  "NonStop Kernel" or "AROS" or "Fenix OS" or
    // "CloudABI" or "Stratus Technologies OpenVOS"
    os_abi: String,

    // 0, compatiable to elf; 
    // else ...
    abi_version: u8,

    // "unknown" or "relocatable" or "executable" or "shared object" or "core dump"
    object_file_type: String,

    // "unknown" or "AT&T WE 32100" or "Advanced Micro Devices X86-64" or ...
    machine: String,

    // "none" or "current"
    version: String,

    // entry point address
    entry_point_address: u64,

    // program header table file offset
    program_header_table_file_offset: u64,

    // section header table file offset
    section_header_table_file_offset: u64,

    // processor-specific flags
    flags: u32,

    // size of this header
    elf_header_size: u16,

    // size of a program header table entry
    program_header_table_entry_size: u16,

    // number of entries in the program header table
    program_header_table_entry_number: u16,

    // size of a section header table entry
    section_header_table_entry_size: u16,

    // number of entries in the section header table
    section_header_table_entry_number: u16,

    // section header table index of the entry associated with the section name string table
    section_header_table_string_table_index: u16,
}

// decode a buffer to an Elfheader
pub fn decode_buffer_to_elfheader(buffer: &[u8]) -> Option<ElfHeader> {
    if buffer.len() < 64 {
        return None;
    }
    let magic_number = [buffer[0], buffer[1], buffer[2], buffer[3]];
    let architecture = match buffer[4] {
        1 => "32-bit".to_string(),
        2 => "64-bit".to_string(),
        _ => "unknown".to_string(),
    };
    let data_encoding = match buffer[5] {
        1 => "little-endian".to_string(),
        2 => "big-endian".to_string(),
        _ => "unknown".to_string(),
    };
    let elf_version = match buffer[6] {
        1 => "current".to_string(),
        _ => "unknown".to_string(),
    };
    let os_abi = match buffer[7] {
        0 => "System V".to_string(),
        1 => "HP-UX".to_string(),
        2 => "NetBSD".to_string(),
        3 => "Linux".to_string(),
        6 => "Solaris".to_string(),
        7 => "AIX".to_string(),
        8 => "IRIX".to_string(),
        9 => "FreeBSD".to_string(),
        10 => "Tru64".to_string(),
        11 => "Modesto".to_string(),
        12 => "OpenBSD".to_string(),
        13 => "OpenVMS".to_string(),
        14 => "NonStop Kernel".to_string(),
        15 => "AROS".to_string(),
        16 => "Fenix OS".to_string(),
        17 => "CloudABI".to_string(),
        18 => "Stratus Technologies OpenVOS".to_string(),
        _ => "unknown".to_string(),
    };
    let abi_version = buffer[8];
    let object_file_type = match u16::from_le_bytes([buffer[16], buffer[17]]) {
        0 => "unknown".to_string(),
        1 => "relocatable".to_string(),
        2 => "executable".to_string(),
        3 => "shared object".to_string(),
        4 => "core dump".to_string(),
        _ => "unknown".to_string(),
    };
    let machine = match u16::from_le_bytes([buffer[18], buffer[19]]) {
        0 => "unknown".to_string(),
        62 => "Advanced Micro Devices X86-64".to_string(),
        _ => "unknown".to_string(),
    };
    let version = match u32::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]) {
        0 => "none".to_string(),
        1 => "current".to_string(),
        _ => "unknown".to_string(),
    };
    let entry_point_address = u64::from_le_bytes([
        buffer[24], buffer[25], buffer[26], buffer[27],
        buffer[28], buffer[29], buffer[30], buffer[31],
    ]);
    let program_header_table_file_offset = u64::from_le_bytes([
        buffer[32], buffer[33], buffer[34], buffer[35],
        buffer[36], buffer[37], buffer[38], buffer[39],
    ]);
    let section_header_table_file_offset = u64::from_le_bytes([
        buffer[40], buffer[41], buffer[42], buffer[43],
        buffer[44], buffer[45], buffer[46], buffer[47],
    ]);
    let flags = u32::from_le_bytes([buffer[48], buffer[49], buffer[50], buffer[51]]);
    let elf_header_size = u16::from_le_bytes([buffer[52], buffer[53]]);
    let program_header_table_entry_size = u16::from_le_bytes([buffer[54], buffer[55]]);
    let program_header_table_entry_number = u16::from_le_bytes([buffer[56], buffer[57]]);
    let section_header_table_entry_size = u16::from_le_bytes([buffer[58], buffer[59]]);
    let section_header_table_entry_number = u16::from_le_bytes([buffer[60], buffer[61]]);
    let section_header_table_string_table_index = u16::from_le_bytes([buffer[62], buffer[63]]);
    Some(ElfHeader {
        magic_number,
        architecture,
        data_encoding,
        elf_version,
        os_abi,
        abi_version,
        object_file_type,
        machine,
        version,
        entry_point_address,
        program_header_table_file_offset,
        section_header_table_file_offset,
        flags,
        elf_header_size,
        program_header_table_entry_size,
        program_header_table_entry_number,
        section_header_table_entry_size,
        section_header_table_entry_number,
        section_header_table_string_table_index,
    })
}


#[derive(Debug)]
pub struct ProgramHeaderTableEntry {
    // "unknown", "loadable", "dynamic", "interpreter", "note", "shlib", "phdr", "tls", "gnu_eh_frame"
    entry_type: String, // 4bytes

    // "read", "write", "execute", 4bytes
    flag_readable: bool,
    flag_wirtable: bool,
    flag_executable: bool,
    flag_value : u32,

    offset: u64,
    virtual_address: u64,
    physical_address: u64,
    file_size: u64,
    memory_size: u64,
    alignment: u64,
}

pub fn decode_buffer_to_program_header_table_entry(buffer: &[u8]) -> Option<ProgramHeaderTableEntry> {
    if buffer.len() < 56 {
        return None;
    }
    let entry_type = match u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) {
        0 => "unknown".to_string(),
        1 => "loadable".to_string(),
        2 => "dynamic".to_string(),
        3 => "interpreter".to_string(),
        4 => "note".to_string(),
        5 => "shlib".to_string(),
        6 => "phdr".to_string(),
        7 => "tls".to_string(),
        0x6474e550 => "gnu_eh_frame".to_string(),
        _ => "unknown type".to_string(),
    };
    let flag_readable = (u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) & 0x1) != 0;
    let flag_wirtable = (u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) & 0x2) != 0;
    let flag_executable = (u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) & 0x4) != 0;
    let flag_value = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
    let offset = u64::from_le_bytes([
        buffer[8], buffer[9], buffer[10], buffer[11],
        buffer[12], buffer[13], buffer[14], buffer[15],
    ]);
    let virtual_address = u64::from_le_bytes([
        buffer[16], buffer[17], buffer[18], buffer[19],
        buffer[20], buffer[21], buffer[22], buffer[23],
    ]);
    let physical_address = u64::from_le_bytes([
        buffer[24], buffer[25], buffer[26], buffer[27],
        buffer[28], buffer[29], buffer[30], buffer[31],
    ]);
    let file_size = u64::from_le_bytes([
        buffer[32], buffer[33], buffer[34], buffer[35],
        buffer[36], buffer[37], buffer[38], buffer[39],
    ]);
    let memory_size = u64::from_le_bytes([
        buffer[40], buffer[41], buffer[42], buffer[43],
        buffer[44], buffer[45], buffer[46], buffer[47],
    ]);
    let alignment = u64::from_le_bytes([
        buffer[48], buffer[49], buffer[50], buffer[51],
        buffer[52], buffer[53], buffer[54], buffer[55],
    ]);
    Some(ProgramHeaderTableEntry {
        entry_type,
        flag_readable,
        flag_wirtable,
        flag_executable,
        flag_value,
        offset,
        virtual_address,
        physical_address,
        file_size,
        memory_size,
        alignment,
    })
}

#[derive(Debug)]
pub struct ProgramHeaderTable{
    // section header table entries
    entries: Vec<ProgramHeaderTableEntry>,
}

pub fn decode_buffer_to_program_header_table(buffer: &[u8], entry_size: u16, entry_number: u16) -> Option<ProgramHeaderTable> {
    if buffer.len() < (entry_size as usize) * (entry_number as usize) {
        return None;
    }
    let mut entries = Vec::new();
    for i in 0..entry_number {
        let entry = decode_buffer_to_program_header_table_entry(&buffer[(i as usize) * (entry_size as usize)..((i + 1) as usize) * (entry_size as usize)]);
        match entry {
            Some(entry) => entries.push(entry),
            None => (),
        }
    }
    Some(ProgramHeaderTable {
        entries,
    })
}

#[derive(Debug)]
pub struct SectionHeaderTableEntry {

    section_name_index: u32,
    
    // "unknown", "program bits", "symbol table", "string table", 
    // "relocation entries", "symbol hash table", "dynamic", 
    // "notes", "no bits", "relocation", "reserved", "dynamic symbol table"
    // "lobits user", "hibits user"
    section_type: String,

    flag_write: bool,
    flag_allocation: bool,
    flag_executable_instructions: bool,
    flag_processor_specific: u64,

    address: u64,
    offset: u64,
    size: u64,
    index_link: u32,
    extra_information: u32,
    address_alignment: u64,
    entry_size: u64,
}

pub fn decode_buffer_to_section_header_table_entry(buffer: &[u8]) -> Option<SectionHeaderTableEntry> {
    if buffer.len() < 64 {
        return None;
    }
    let section_name_index = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    let section_type = match u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]) {
        0 => "unknown".to_string(),
        1 => "program bits".to_string(),
        2 => "symbol table".to_string(),
        3 => "string table".to_string(),
        4 => "relocation entries".to_string(),
        5 => "symbol hash table".to_string(),
        6 => "dynamic".to_string(),
        7 => "notes".to_string(),
        8 => "no bits".to_string(),
        9 => "relocation".to_string(),
        10 => "reserved".to_string(),
        11 => "dynamic symbol table".to_string(),
        0x6fffff00 => "lobits user".to_string(),
        0x6fffffff => "hibits user".to_string(),
        _ => "unknown".to_string(),
    };
    let flag_write = (u64::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]) & 0x1) != 0;
    let flag_allocation = (u64::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]) & 0x2) != 0;
    let flag_executable_instructions = (u64::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]) & 0x4) != 0;
    let flag_processor_specific = u64::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]);
    let address = u64::from_le_bytes([
        buffer[16], buffer[17], buffer[18], buffer[19],
        buffer[20], buffer[21], buffer[22], buffer[23],
    ]);
    let offset = u64::from_le_bytes([
        buffer[24], buffer[25], buffer[26], buffer[27],
        buffer[28], buffer[29], buffer[30], buffer[31],
    ]);
    let size = u64::from_le_bytes([
        buffer[32], buffer[33], buffer[34], buffer[35],
        buffer[36], buffer[37], buffer[38], buffer[39],
    ]);
    let index_link = u32::from_le_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]);
    let extra_information = u32::from_le_bytes([buffer[44], buffer[45], buffer[46], buffer[47]]);
    let address_alignment = u64::from_le_bytes([
        buffer[48], buffer[49], buffer[50], buffer[51],
        buffer[52], buffer[53], buffer[54], buffer[55],
    ]);
    let entry_size = u64::from_le_bytes([
        buffer[56], buffer[57], buffer[58], buffer[59],
        buffer[60], buffer[61], buffer[62], buffer[63],
    ]);
    Some(SectionHeaderTableEntry {
        section_name_index,
        section_type,
        flag_write,
        flag_allocation,
        flag_executable_instructions,
        flag_processor_specific,
        address,
        offset,
        size,
        index_link,
        extra_information,
        address_alignment,
        entry_size,
    })
}

#[derive(Debug)]
pub struct SectionHeaderTable {
    // section header table entries
    entries: Vec<SectionHeaderTableEntry>,
}

pub fn decode_buffer_to_section_header_table(buffer: &[u8], entry_size: u16, entry_number: u16) -> Option<SectionHeaderTable> {
    if buffer.len() < (entry_size as usize) * (entry_number as usize) {
        return None;
    }
    let mut entries = Vec::new();
    for i in 0..entry_number {
        let entry = decode_buffer_to_section_header_table_entry(&buffer[(i as usize) * (entry_size as usize)..((i + 1) as usize) * (entry_size as usize)]);
        match entry {
            Some(entry) => entries.push(entry),
            None => (),
        }
    }
    Some(SectionHeaderTable {
        entries,
    })
}

#[derive(Debug)]
pub struct StringTable {
    strings: Vec<String>,
}

pub fn decode_buffer_to_string_table(buffer: &[u8]) -> Option<StringTable> {
    // the first & last strings are "\0"
    // all strings are null-ended
    let mut strings = Vec::new();

    let mut i = 0;
    if buffer.len() == 0 {
        return None;
    }
    if buffer[i] != 0u8 {
        return None;
    }
    i += 1;
    strings.push(String::from(""));
    
    while i < buffer.len() {
        let mut string = String::from("");

        while i < buffer.len() && buffer[i] != 0u8 {
            string.push(buffer[i] as char);
            i += 1;
        }
        strings.push(string.clone());
        if string == "" {
            break;
        } else {
            i += 1;
        }
    }

    if i == buffer.len() {
        return None;
    }

    Some(StringTable {
        strings,
    })
}

#[derive(Debug)]
pub struct Elf64 {
    // elf header
    elf_header: ElfHeader,

    // program header table
    program_header_table: ProgramHeaderTable,

    // section header table
    section_header_table: SectionHeaderTable,

    // section name string table
    string_table: StringTable,
}

pub fn decode_buffer_to_elf64(buffer: &[u8]) -> Option<Elf64> {
    let elf_header = decode_buffer_to_elfheader(&buffer)?;
    let program_header_table = decode_buffer_to_program_header_table(&buffer[(elf_header.program_header_table_file_offset as usize)..], elf_header.program_header_table_entry_size, elf_header.program_header_table_entry_number)?;
    let section_header_table = decode_buffer_to_section_header_table(&buffer[(elf_header.section_header_table_file_offset as usize)..], elf_header.section_header_table_entry_size, elf_header.section_header_table_entry_number)?;
    
    // TODO: index may be SHN_UNDEF or SHN_XINDEX 
    let index = elf_header.section_header_table_string_table_index as usize;
    let offset = section_header_table.entries[index].offset as usize;

    let string_table = decode_buffer_to_string_table(&buffer[(offset as usize)..])?;
    Some(Elf64 {
        elf_header,
        program_header_table,
        section_header_table,
        string_table,
    })
}

