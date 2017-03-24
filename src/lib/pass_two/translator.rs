use basic_types::flags::*;
use basic_types::instruction::{Instruction, AsmOperand};
use basic_types::operands::{OperandType, Value};
use basic_types::formats::Format;
use basic_types::instruction_set;
use basic_types::unit_or_pair::UnitOrPair;
use std::num;

pub fn translate(instruction: &Instruction) -> Result<u32, &str> {
    //let f_vals = instruction.check_invalid_flags();   // TODO Report to RLS
    //resolve_instruction_code(instruction, 0).and_then(resolve_operands)

    // Check the flags for options
    // Check for memory out of range error
    // Assemble the instruciton

    unimplemented!();
}

fn resolve_incomplete_operands(instruction: &mut Instruction) -> Result<Vec<u32>, String> {
    // Convert immediate and indirect operands to a basic forms -> Raw
    let mut raws: Vec<u32> = Vec::new();
    let opVec = instruction.unwrap_operands();

    for operand in &opVec {
        let raw: Result<u32, &str> = match operand.val {
            Value::SignedInt(x) => Ok(x.abs() as u32),
            Value::Register(ref x) => Ok((*x as u8) as u32),
            // Get from symtab
            Value::Label(ref x) => resolve_label(x.as_str()),
            Value::Raw(x) => Ok(x),
        };

        match raw {
            Ok(x) => raws.push(x),
            _ => return Err(format!("Couldn't resolve label {:?}", operand.val).clone()),  // Nothing else can fail
        }
    }
    Ok(raws)
}

fn resolve_opcode(instr: &Instruction, code_val: u32) -> Result<u32, &str> {
    // Get the opcode value from the instruction set table
    // Check format correctness
    unimplemented!();
}

fn resolve_label(label: &str) -> Result<u32, &str> {
    // Check the symtab
    // Check the range of addresses with the instruction format
    unimplemented!();
}