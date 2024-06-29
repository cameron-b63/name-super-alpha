// This file contains the implmented instruction set as a series of dope vectors.
use crate::instruction_information::InstructionInformation;

pub const R_TYPE: u8 = 1;
pub const I_TYPE: u8 = 2;
pub const J_TYPE: u8 = 3;

const NUM_OF_IMPL_SECTIONS: usize = 3;

pub const INSTRUCTIONS: [InstructionInformation; NUM_OF_IMPL_SECTIONS] = [
    InstructionInformation{
        mnemonic: "add",
        instr_type: R_TYPE,
        opcode: 0,
        shamt: 0,
        funct: 0b100000,
        args: 3,
    },

    InstructionInformation{
        mnemonic: "lui",
        instr_type: I_TYPE,
        opcode: 0x0F,
        shamt: 0,
        funct: 0,
        args: 2,
    },

    InstructionInformation{
        mnemonic: "j",
        instr_type: J_TYPE,
        opcode: 0x02,
        shamt: 0,
        funct: 0,
        args: 1,
    }
];