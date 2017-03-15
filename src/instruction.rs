#![allow(dead_code)]
// Make the linter silent

enum Formats {
    One,
    Two,
    Three,
    Four,
    None,
}

enum Flags {
    Indirect = 0,
    Immediate,
    Indexed,
    Extended,
    BaseRelative,
    PcRelative,
    None,
}

/**
 * Resembles a SIC/XE instruction, this object is immutable,
 * Each method that mutates the state should return a new object
 */
struct Instruction {
    format: Formats,
    instruction: String,
    flags: Vec<Flags>,
    op1: Operand,
    op2: Operand,
}

impl Instruction {
    fn new(instruction: String, op1: Operand, op2: Operand) -> Instruction {
        Instruction {
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
    fn set_flag(mut self, flag: Flags) {
        self.flags.push(flag);
    }
}

/**
*  Instruction operand
*/
pub enum Operand {
    Register(u8),
    Immediate(i32),
    Label(String), // Load the memory address for the lable
    None,
}
