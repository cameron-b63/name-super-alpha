// This file contains the implmented instruction set as a series of dope vectors.
use crate::dope_vector::{DopeVector, R_TYPE, I_TYPE, J_TYPE};

const NUM_OF_IMPL_SECTIONS: usize = 3;

pub const INSTRUCTIONS: [DopeVector; NUM_OF_IMPL_SECTIONS] = [
    DopeVector{
        mnemonic: "add",
        instr_type: R_TYPE,
        opcode: 0,
        funct: 0b100000,
    },

    DopeVector{
        mnemonic: "lui",
        instr_type: I_TYPE,
        opcode: 0x0F,
        funct: 0,
    },

    DopeVector{
        mnemonic: "j",
        instr_type: J_TYPE,
        opcode: 0x02,
        funct: 0,
    }
];