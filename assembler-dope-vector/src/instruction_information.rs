// This file contains the struct definition as well as any implmentations for the [InstructionInformation] struct.
#[derive(Debug)]
#[repr(C)]
pub struct InstructionInformation {
    pub(crate) mnemonic: &'static str,
    pub(crate) instr_type: u8,
    pub(crate) opcode: u8,
    pub(crate) shamt: u8,
    pub(crate) funct: u8,
    pub(crate) args: u8,
}
