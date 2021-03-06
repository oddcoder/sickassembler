#![allow(dead_code)]

use register::Register;
use std::fmt;
/**
*  Instruction operand
*/
#[derive(Debug,PartialEq,Clone)]
pub enum OperandType {
    Register, // Register number
    Immediate,
    Indirect, // memory location/label -> memory location
    Label, // Load the memory address for the lable
    Bytes,
    VarArgs, // Variable length operands, for EXTREF,EXTDEF
    Expression,
    None,

    // Pass2 will convert any of the provided operands above to a raw nueric value
    // This is also supported if the source code contains a direct memory address
    // This can also be the n operand from the instruction set, ex. shift r1,n TODO check range value
    // Raw is used for raw hex output, i.e Unsigned values, unlike immediate
    Raw,
}

pub fn match_value(first: &OperandType, second: &Value) -> bool {
    match (first.clone(), second.clone()) {
        (OperandType::Immediate, Value::SignedInt(_)) |
        (OperandType::Indirect, Value::Label(_)) |
        (OperandType::Label, Value::Label(_)) |
        (OperandType::None, Value::None) |
        (OperandType::Raw, Value::Raw(_)) |
        (OperandType::Register, Value::Register(_)) => true,
        _ => false,
    }
}

/// Use the SignedInt with immediate operand types
#[derive(PartialEq,Clone)]
pub enum Value {
    Register(Register),
    SignedInt(i32),
    Raw(u32), // Use this with indexed ?
    Label(String),
    Bytes(String),
    VarArgs(Vec<String>),
    Expression(Vec<String>),
    None,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_print;

        to_print = match self.clone() {
            Value::Register(x) => format!("Register {:#4X}", (x as u8)),
            Value::Raw(x) => format!("Raw {:#4X}", x),
            Value::SignedInt(x) => format!("Immediate {}", x),
            Value::Label(x) => format!("Label {}", x),
            Value::Bytes(x) => format!("Literal/Bytes {}", x),
            _ => String::new(),
        };

        write!(f, "\"{}\"", to_print)
    }
}
