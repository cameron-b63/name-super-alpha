// This file contains the struct definition as well as key implmentations / constants for a dope vector struct.

pub const R_TYPE: u8 = 1;
pub const I_TYPE: u8 = 2;
pub const J_TYPE: u8 = 3;

#[derive(Debug)]
#[repr(C)]
pub struct DopeVector {
    pub(crate) mnemonic: &'static str,
    pub(crate) instr_type: u8,
    pub(crate) opcode: u8,
    pub(crate) funct: u8,
}
