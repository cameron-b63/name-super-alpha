// This is the preprocessor for NAME.
// It records line numbers for section .line, and creates the symbol table.
// It also records and .eqv symbols in the symbol table.
const MIPS_TEXT_START_ADDR: u32 = 0x400000;

use crate::name_structs::{LineSectionEntry, SymbolTableEntry, SymbolType};
use crate::parser::{parse, LineComponent, ComponentType};

// Perform a pass to grab line numbers. When symbols are encountered, create a new symbol table entry
// And append them to the symbol table.
pub fn preprocess(file_contents: &String) -> (Vec<LineSectionEntry>, Vec<SymbolTableEntry>) {
    let lines = file_contents.split('\n');

    let mut line_number: u32 = 1;
    let mut current_address: u32 = MIPS_TEXT_START_ADDR;
    let mut must_increase_address: bool;

    let mut section_dot_line: Vec<LineSectionEntry> = vec!();
    let mut symbol_table: Vec<SymbolTableEntry> = vec!();

    for line in lines {
        must_increase_address = false;
        // Add the line's information to section .line (it's a surprise tool that will help us later!)
        // This is done now because any pseudoinstructions or macros will affect the line numbers. This count must be done as early as possible.
        let line_entry: LineSectionEntry = LineSectionEntry {
            line_number: line_number,
            content: line.to_string(),
        };

        section_dot_line.push(line_entry);

        // The preprocessor will search for any declared .eqv values, and replace them on that line.
        // If the line contains a .eqv directive, the preprocessor will add it to the list to search for.
        // TODO

        // Here, the line is parsed into its components. Any found labels are added to the symbol table
        // everything else is only used to calculate the current address.
        let components: Vec<LineComponent> = parse(&line).expect("Failed to parse line in preprocessing.");
        for component in components {
            match component.component_type {
                ComponentType::Label => {
                    let symbol: SymbolTableEntry = SymbolTableEntry{
                        symbol_type: SymbolType::Label,
                        identifier: component.content.to_string(),
                        address: Some(current_address),
                        value: None,
                    };

                    symbol_table.push(symbol);
                },
                ComponentType::Instruction => {
                    must_increase_address = true;
                },
                _ => {}
            }
        }

        line_number += 1;

        if must_increase_address{
            current_address += 4;
        }
    }

    return (section_dot_line, symbol_table);
}