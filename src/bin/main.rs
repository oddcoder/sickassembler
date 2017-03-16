extern crate sick_lib;

use sick_lib::basic_types::*;
use instruction::Instruction;
use operands::Operand;

fn main() {
    // `to_owned()` creates an owned `String` from a string slice.
    Instruction::new("instruction: String".to_owned(),
                     Operand::Immediate(23),
                     Operand::Immediate(21));
    println!("Hello, world!");
}
