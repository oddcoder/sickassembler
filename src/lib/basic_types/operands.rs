#![allow(dead_code)]

use basic_types::register::Register;

/**
*  Instruction operand
*/
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
#[derive(Debug,PartialEq,Clone)]
=======
#[derive(Debug,PartialEq)]
>>>>>>> Pass 2 preparation
pub enum Operand {
    Register(Option<Register>), // Register number
    Immediate(Option<i32>),
    Indirect(Option<u32>), // memory location/label -> memory location
    Label(Option<String>), // Load the memory address for the lable
    None,

    // Pass2 will convert any of the provided operands above to a raw nueric value
    // This is also supported if the source code contains a direct memory address
    // This can also be the n operand from the instruction set, ex. shift r1,n TODO check range value
    // Raw is used for raw hex output, i.e Unsigned values, unlike immediate
    Raw(Option<u32>),
}
