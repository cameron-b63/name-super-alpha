// This file contains helper functions for assembly.

use crate::instruction_information::InstructionInformation;
use crate::name_constants::REGISTERS;

pub fn assemble_instruction(_instruction_info: &InstructionInformation) -> Result<u32, &'static str> {
    return Ok(0xBEEF as u32);
}

pub fn print_assembled_instruction(packed: &u32){
    println!("0x{:08x}", packed);
    println!("0b{:032b}", packed);
}

pub fn parse_register_to_u8(register: &str) -> Result<u8, &'static str> {
    // First, try a simple lookup on the REGISTERS constant.
    if let Some(index) = REGISTERS.iter().position(|&x| x == register){
        return Ok(index as u8);
    } else if let Ok(attempted_straight_parse) = register.chars().skip(1).collect::<String>().parse::<u8>(){
        // This line looks like wizard stuff but really I'm just removing the first char from the string by
        // using an iterator, skipping an item, and collecting everything else back together
        return Ok(attempted_straight_parse);
    } else {
        return Err("Register parse failed");
    }
}

/*
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
*/