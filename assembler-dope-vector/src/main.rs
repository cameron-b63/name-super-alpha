mod instruction_information;
mod instructions;
mod assembly_utils;
mod parser;
mod nma;

use std::io;
use std::fs;

use crate::nma::assemble;

fn main(){
    // Get filename
    let mut input = String::new();
    let filename: String = io::stdin().read_line(&mut input).expect("Failed to read entered filename.").to_string();

    // Get file contents as string
    let file_contents = fs::read_to_string(filename.trim()).expect("Failed to read file.");

    // Invoke the assembler on those file contents
    match assemble(&file_contents){
        Ok(_assembled_vectors) => {
            // Write et_rel to file with sections
        },
        Err(_) => {
            panic!("A fatal error occurred during assembly.");
        }
    };
}