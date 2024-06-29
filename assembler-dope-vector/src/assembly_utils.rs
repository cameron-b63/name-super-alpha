// This file contains helper functions for assembly.

use crate::instruction_information::{InstructionInformation};

pub fn assemble_instruction(_content: &str) -> Result<u32, &'static str> {
    return Ok(0xBEEF as u32);
}

pub fn print_assembled_instruction(packed: &u32){
    println!("0x{:08x}", packed);
    println!("0b{:032b}", packed);
}

fn assemble_r_type(rs: u8, rt: u8, rd: u8, shamt: u8, funct: u8) -> u32 {
    let opcode: u32 = 0;         // Constant for all R-type instructions
    // Return packed instruction
    return
        ((opcode as u32) << 26) |
        ((rs as u32) << 21) | 
        ((rt as u32) << 16) | 
        ((rd as u32) << 11) | 
        ((shamt as u32) << 6) |
        ((funct as u32))
}