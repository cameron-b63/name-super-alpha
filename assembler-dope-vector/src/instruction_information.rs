// This file contains the struct definition as well as any implmentations for the [InstructionInformation] struct.
#[derive(Debug)]
#[repr(C)]
pub struct InstructionInformation {
    pub(crate) mnemonic: &'static str,
    pub(crate) instr_type: InstructionType,
    pub(crate) opcode: u8,
    pub(crate) shamt: u8,
    pub(crate) funct: u8,
    pub(crate) args: u8,
}

// This is the instruction type 
#[derive(Debug)]
pub enum InstructionType {
    RType,
    IType,
    JType,
}
