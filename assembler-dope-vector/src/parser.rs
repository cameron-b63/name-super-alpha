// This file contains the line parser for the assembler.
pub enum ComponentType {
    Instruction,
    Register,
    Immediate,
    Directive,
    Comment,
    Label,
    Identifier,
}

pub struct LineComponent<'a> {
    pub(crate) component_type: ComponentType,
    pub(crate) content: &'a str,
}

// The intention with this parsing is to split each line into its individual components. 
pub fn parse(line: &str) -> Result<Vec<LineComponent>, &'static str> {
    let mut found_components: Vec<LineComponent> = vec!();

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
            let label: LineComponent = LineComponent{
                component_type: ComponentType::Label,
                content: word,
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
        } else if is_valid_immediate(word) {
            let immediate: LineComponent = LineComponent{
                component_type: ComponentType::Immediate,
                content: word
            };
            found_components.push(immediate);
            continue;
        } else if word.chars().all( |c| c.is_alphanumeric()) {
            let identifier: LineComponent = LineComponent{
                component_type: ComponentType::Identifier,
                content: word,
            };
            found_components.push(identifier);
            continue;
        } else if word.chars().all( |c| c.is_ascii_lowercase()) {
            todo!();
            // Determine difference between used label and instruction. Likely need to match against set of impl instructions.
        }
    }



    return Ok(found_components);
}

fn is_valid_immediate(_token: &str) -> bool {
    todo!();
}