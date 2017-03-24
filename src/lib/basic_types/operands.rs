#![allow(dead_code)]

use basic_types::register::Register;
/**
*  Instruction operand
*/
<<<<<<< Updated upstream
#[derive(Debug,PartialEq)]
pub enum Operand {
    Register(Option<Register>), // Register number
    Immediate(Option<i32>),
    Indirect(Option<u32>), // memory location/label -> memory location
    Label(Option<String>), // Load the memory address for the lable
=======
#[derive(Debug,PartialEq,Clone)]
pub enum OperandType {
    Register, // Register number
    Immediate,
    Indirect, // memory location/label -> memory location
    Label, // Load the memory address for the lable
>>>>>>> Stashed changes
    None,

    // Pass2 will convert any of the provided operands above to a raw nueric value
    // This is also supported if the source code contains a direct memory address
    // This can also be the n operand from the instruction set, ex. shift r1,n TODO check range value
    // Raw is used for raw hex output, i.e Unsigned values, unlike immediate
    Raw,
}

#[derive(Debug,PartialEq,Clone)]
pub enum Value {
    Register(Register),
    SignedInt(i32),
    Raw(u32),
    Label(String),
}