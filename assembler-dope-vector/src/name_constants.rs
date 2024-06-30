// This file contains the implmented instruction set as a series of dope vectors, as well as the register set.
use crate::instruction_information::{InstructionInformation, InstructionType};

const NUM_OF_IMPL_SECTIONS: usize = 3;
const NUM_OF_REGISTERS: usize = 32;

pub const REGISTERS: [&'static str; NUM_OF_REGISTERS] = [
    "$zero", 
    "$at", 
    "$v0", "$v1", 
    "$a0", "$a1", "$a2", "$a3",
    "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7",
    "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7",
    "$t8", "$t9", 
    "$k0", "$k1", 
    "$gp", "$sp", "$fp", 
    "$ra"
];


pub const INSTRUCTIONS: [InstructionInformation; NUM_OF_IMPL_SECTIONS] = [
    InstructionInformation{
        mnemonic: "add",
        instr_type: InstructionType::RType,
        opcode: 0,
        shamt: 0,
        funct: 0b100000,
        args: 3,
    },

    InstructionInformation{
        mnemonic: "lui",
        instr_type: InstructionType::IType,
        opcode: 0x0F,
        shamt: 0,
        funct: 0,
        args: 2,
    },

    InstructionInformation{
        mnemonic: "j",
        instr_type: InstructionType::JType,
        opcode: 0x02,
        shamt: 0,
        funct: 0,
        args: 1,
    }
];