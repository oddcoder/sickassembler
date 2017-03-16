#![allow(dead_code)]
#![allow(unused_imports)]
// Make the linter silent

use basic_types::formats::Formats;
use basic_types::flags::Flags;
use basic_types::operands::Operand;
use basic_types::register::Register;

/**
 * Resembles a SIC/XE instruction, this object is immutable,
 * Each method that mutates the state should return a new object
 */
pub struct Instruction {
    label: String,
    format: Formats,
    instruction: String,
    flags: Vec<Flags>,
    op1: Operand,
    op2: Operand,
}

impl Instruction {
    /**
     * new A plain new instruction
     */
    pub fn new(label: String, instruction: String, op1: Operand, op2: Operand) -> Instruction {
        Instruction {
            label: label,
            format: Formats::None,
            instruction: instruction,
            flags: Vec::new(),
            op1: op1,
            op2: op2,
        }
    }


    /**
    * to_pc_relative returns a new instructions object with PC
    * relative flag set to 1
    */
    pub fn set_flag(mut self, flag: Flags) {
        self.flags.push(flag);
    }

    /**
     *  get_flags_value returns the numeric value of the flags
     *  declared on this instruction
     */
    fn get_flags_value() -> u32 {
        // Create an extra check to see if conflicting flags exist
        // Set the flags if the instuction is not any of the special cases
        // i.e set the Indirect and Immediate flags to 1
        panic!("Not implemented")
    }
}
