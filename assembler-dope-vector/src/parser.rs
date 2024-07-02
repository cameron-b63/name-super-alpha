// This file contains the line parser for the assembler.

use crate::name_constants::INSTRUCTIONS;

#[derive(Debug)]
pub enum ComponentType {
    Instruction,
    Register,
    Immediate,
    Directive,
    Comment,
    Label,
    Identifier,
}

#[derive(Debug)]
pub struct LineComponent<'a> {
    pub(crate) component_type: ComponentType,
    pub(crate) content: &'a str,
}

// The intention with this parsing is to split each line into its individual components. 
pub fn parse<'a>(line: &'a str) -> Result<Vec<LineComponent<'a>>, &'static str> {
    let mut found_components: Vec<LineComponent> = vec!();

    let mut mnemonics: Vec<&str> = vec!();

    // Retrieve instruction mnemonics from data file
    for instruction in &INSTRUCTIONS {
        mnemonics.push(instruction.mnemonic);
    }

    // Removing leading whitespace allows first char processing
    let trimmed_line = line.trim();

    // If line starts with '#' we have an early exit since the whole line must now be a comment
    if trimmed_line.starts_with('#') {
        let comment: LineComponent = LineComponent{
            component_type: ComponentType::Comment,
            content: line,
        };
        found_components.push(comment);
        return Ok(found_components);
    }

    // Tokenize the string and categorize using special tokens such as '#', '.', and ':'
    let mut iterable_tokens = trimmed_line.split_whitespace();
    while let Some(word) = iterable_tokens.next() {
        if word.ends_with(':') {
            let len = word.len();
            if len == 1 {
                return Err("Cannot have an empty label (i.e. just ':')");
            }

            let identifier = &word[..len - 1]; // Chop off last character of slice

            let label: LineComponent = LineComponent{
                component_type: ComponentType::Label,
                content: identifier,
            };
            found_components.push(label);
            continue;
        
        } else if word.starts_with('$') {
            let register: LineComponent = LineComponent{
                component_type: ComponentType::Register,
                content: word
            };
            found_components.push(register);
            continue;
        } else if word.starts_with('.') {
            let directive: LineComponent = LineComponent{
                component_type: ComponentType::Directive,
                content: word
            };
            found_components.push(directive);
            continue;
        } if word.starts_with('#'){
            break;
        } else if let Ok(_immediate) = base_parse(word) {
            let immediate: LineComponent = LineComponent{
                component_type: ComponentType::Immediate,
                content: word,
            };
            found_components.push(immediate);
            continue;
        } else if word.chars().all( |c| c.is_alphanumeric()) {
            // Test to see if found alphanumeric string is an instruction mnemonic
            if mnemonics.contains(&word) {
                let instruction: LineComponent = LineComponent{
                    component_type: ComponentType::Instruction,
                    content: word,
                };
                found_components.push(instruction);
                continue;
            } else {
                let identifier: LineComponent = LineComponent{
                    component_type: ComponentType::Identifier,
                    content: word,
                };
                found_components.push(identifier);
                continue;
            }
        }
    }

    // Collect any trailing inline comment
    if let Some(index) = trimmed_line.find('#') {
        // Collect the entire inline comment for debugging purposes
        let collected_comment: &'a str = trimmed_line[index..].trim();
        let inline_comment: LineComponent = LineComponent{
            component_type: ComponentType::Comment,
            content: collected_comment,
        };
        found_components.push(inline_comment);
    } 
    
    return Ok(found_components);
}

// Parses literals in hex, bin, oct, and decimal.
pub fn base_parse(input: &str) -> Result<u32, &'static str> {
    if input.starts_with("0x") {
        // Hexadecimal
        u32::from_str_radix(&input[2..], 16).map_err(|_| "Failed to parse as hexadecimal")
    } else if input.starts_with("0b") {
        // Binary
        u32::from_str_radix(&input[2..], 2).map_err(|_| "Failed to parse as binary")
    } else if input.starts_with('0') && input.len() > 1 {
        // Octal
        u32::from_str_radix(&input[1..], 8).map_err(|_| "Failed to parse as octal")
    } else {
        // Decimal
        input
            .parse::<u32>()
            .map_err(|_| "Failed to parse as decimal")
    }
}