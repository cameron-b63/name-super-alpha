// This is the NAME MIPS Assembler that main.rs invokes.

use crate::instruction_information::InstructionInformation;
use crate::instructions::{INSTRUCTIONS, R_TYPE, I_TYPE, J_TYPE};
use crate::assembly_utils::{assemble_instruction, print_assembled_instruction};
use crate::parser::{parse, ComponentType, LineComponent};

use std::collections::HashMap;

pub fn assemble(file_contents: &String) -> Result<(), String> {
    // Setup table
    let mut table: HashMap<&'static str, &InstructionInformation> = HashMap::new();

    for instruction in &INSTRUCTIONS {
        table.insert(instruction.mnemonic, instruction);
    }

    let lines = file_contents.split('\n');

    let mut _line_number = 1;

    let mut section_dot_text_bytes: Vec<u8> = vec!();

    for line in lines {
        // Parse line
        let line_components: Vec<LineComponent> = parse(&line).expect("Failed to parse.");

        // Match handling for each different component
        for component in line_components {
            match component.component_type {
                ComponentType::Instruction => {
                    // Assemble instruction
                    if let Ok(packed) = assemble_instruction(&component.content){
                        print_assembled_instruction(&packed);
                        section_dot_text_bytes.extend_from_slice(&packed.to_be_bytes());
                    } else {
                        return Err(format!("Malformed instruction."));
                    }

                    // Update line information
                },
                ComponentType::Directive => {
                    todo!();
                },
                ComponentType::Label => {
                    todo!();
                },
                ComponentType::Comment => {
                    // Do nothing; eventually, add information to .line
                },
            }
        }

        _line_number += 1;
    }

    Ok(())
}