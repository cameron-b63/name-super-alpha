mod dope_vector;
mod instructions;
use crate::dope_vector::{DopeVector, R_TYPE, I_TYPE, J_TYPE};
use crate::instructions::INSTRUCTIONS;

use std::collections::HashMap;

fn main() {
    let mut table: HashMap<&'static str, &DopeVector> = HashMap::new();

    for instruction in &INSTRUCTIONS {
        table.insert(instruction.mnemonic, instruction);
    }

    dbg!(table.get("add"));
}
