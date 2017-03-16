#![allow(dead_code)]

use basic_types::register::Register;

/**
*  Instruction operand
*/
pub enum Operand {
    Register(Register), // Register number
    Immediate(i32),
    Label(String), // Load the memory address for the lable
    None,
}
