// This file contains the important structs for the NAME assembler and emulator to do their jobs. It will live in name-const.

#[repr(C)]
#[derive(Debug)]
pub struct InstructionInformation {
    pub(crate) mnemonic: &'static str,
    pub(crate) instr_type: InstructionType,
    pub(crate) opcode: u8,
    pub(crate) shamt: u8,
    pub(crate) funct: u8,
    pub(crate) args: u8,
}
 
#[derive(Debug)]
pub enum InstructionType {
    RType,
    IType,
    JType,
}

// The address field is an option since .eqv will not have values.
// Similarly, the value field is an option since labels will not have values.
#[repr(C)]
#[derive(Debug)]
pub struct SymbolTableEntry {
    pub(crate) symbol_type: SymbolType,
    pub(crate) identifier: String,
    pub(crate) address: Option<u32>,
    pub(crate) value: Option<u32>,
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
    _Equivalence,
}

// Note that the null byte to terminate the content string is NOT included in the data until the struct is serialized to bytes.
#[repr(C)]
#[derive(Debug)]
pub struct LineSectionEntry {
    pub(crate) line_number: u32,
    pub(crate) content: String,
}