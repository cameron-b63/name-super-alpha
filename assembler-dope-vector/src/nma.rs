// This is the NAME MIPS Assembler that main.rs invokes.

use crate::instruction_information::InstructionInformation;
use crate::name_constants::INSTRUCTIONS;
use crate::assembly_utils::{assemble_instruction, print_assembled_instruction, parse_register_to_u8};
use crate::parser::{parse, /*base_parse,*/ ComponentType, LineComponent};

use std::collections::HashMap;

pub fn assemble(file_contents: &String) -> Result<(), String> {
    // Setup instruction table for use later
    let mut instruction_table: HashMap<&'static str, &InstructionInformation> = HashMap::new();
    for instruction in &INSTRUCTIONS {
        instruction_table.insert(instruction.mnemonic, instruction);
    }

    // Processing logic
    let lines = file_contents.split('\n');
    let mut _line_number = 1;
    let mut section_dot_text_bytes: Vec<u8> = vec!();

    for line in lines {
        let line_components: Vec<LineComponent> = parse(&line).expect("Failed to parse.");
        let mut associated_instruction_information: Option::<&InstructionInformation> = None;
        let mut associated_registers: Vec<u8> = vec!();

        for component in line_components {
            dbg!(&component);
            match component.component_type {
                ComponentType::Instruction => {
                    associated_instruction_information = Some(instruction_table.get(&component.content).expect("Invalid instruction mnemonic."));
                },
                ComponentType::Register => {
                    if let Ok(register) = parse_register_to_u8(&component.content){
                        associated_registers.push(register);
                    }
                },
                ComponentType::Directive => {
                    todo!();
                },
                ComponentType::Identifier => {
                    todo!();
                },
                ComponentType::Label => {
                    todo!();
                },
                ComponentType::Immediate => {
                    
                },
                ComponentType::Comment => {
                    // Do nothing; eventually, add information to section .line
                },
            }
        }

        if Option::is_some(&associated_instruction_information) {
            if let Ok(assembled_instruction) = assemble_instruction(&associated_instruction_information.unwrap()){
                print_assembled_instruction(&assembled_instruction);
                section_dot_text_bytes.extend_from_slice(&assembled_instruction.to_be_bytes());
            } else {
                return Err(format!("Failed to assemble instruction"));
            }
        }

        dbg!(&section_dot_text_bytes);

        _line_number += 1;
    }

    Ok(())
}