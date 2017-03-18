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

    // Pass2 will convert any of the provided operands above to a raw nueric value
    // This is also supported if the source code contains a direct memory address
    Raw(u32),
}
