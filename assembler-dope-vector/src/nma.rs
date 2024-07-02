// This is the NAME MIPS Assembler that main.rs invokes.

use crate::name_structs::InstructionInformation;
use crate::name_constants::INSTRUCTIONS;
use crate::assembly_utils::{assemble_instruction, print_assembled_instruction};
use crate::parser::{parse, /*base_parse,*/ ComponentType, LineComponent};
use crate::preprocessor::preprocess;

use std::collections::HashMap;

pub fn assemble(file_contents: &String) -> Result<(), String> {
    // Invoke the preprocessor
    let (_section_dot_line, symbol_table) = preprocess(&file_contents);

    dbg!(&symbol_table);

    // Setup instruction table for use later
    let mut instruction_table: HashMap<&'static str, &InstructionInformation> = HashMap::new();
    for instruction in &INSTRUCTIONS {
        instruction_table.insert(instruction.mnemonic, instruction);
    }

    let lines = file_contents.split('\n');

    let mut _line_number = 1;

    let mut section_dot_text_bytes: Vec<u8> = vec!();

    for line in lines {
        let line_components: Vec<LineComponent> = parse(&line).expect("Failed to parse.");
        let mut associated_instruction_information: Option::<&InstructionInformation> = None;
        let mut arguments: Vec<LineComponent> = vec!();

        for component in line_components {
            dbg!(&component);
            match component.component_type {
                ComponentType::Instruction => {
                    associated_instruction_information = Some(instruction_table.get(&component.content).expect("Invalid instruction mnemonic."));
                },
                ComponentType::Register => {
                    arguments.push(component);
                },
                ComponentType::Directive => {
                    todo!();
                },
                ComponentType::Identifier => {
                    // The identifier will be translated out of the symbol table in the instruction assembly phase.
                    // All that must be done right now is to push the component into the arguments.
                    arguments.push(component);
                },
                ComponentType::Immediate => {
                    // Parsing the given immediate into a u32 won't help yet. that should be handled by the assembly of the actual instruction.
                    // All there is to do right now is push the component into the arguments.
                    arguments.push(component);
                },
                ComponentType::Comment => {
                    // Do nothing; eventually, add information to section .line
                },
                _ => {
                    // Any underscore matching implies that functionality was handled by the preprocessor, and thus there is nothing left to do now.
                }
            }
        }

        if Option::is_some(&associated_instruction_information) {
            if let Ok(assembled_instruction) = assemble_instruction(&associated_instruction_information.unwrap(), &arguments){
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