use basic_types::flags::*;
use basic_types::instruction::{Instruction, AsmOperand};
use basic_types::operands::{OperandType, Value};
use basic_types::formats::*;
use basic_types::instruction_set;
use basic_types::unit_or_pair::UnitOrPair;
use std::num;

pub fn translate(instruction: &Instruction) -> Result<u32, &str> {
    //let f_vals = instruction.check_invalid_flags();   // TODO Report to RLS
    //resolve_instruction_code(instruction, 0).and_then(resolve_operands)

    // Check the flags for options
    // Check for memory out of range error
    // Assemble the instruciton
    let raw_operans = resolve_incomplete_operands(instruction);
    let raw_opcode = resolve_opcode(instruction);
    let raw_flags = instruction.get_flags_value(); // TODO propagate the error from getting flag values

    debug!("Tranlating instruction {:?}", instruction);
    debug!("Raw instruction operands {:?}", raw_operans);
    debug!("Raw flag value {:?}", raw_flags);
    debug!("Instruction opcode {:?}", raw_opcode);

    unimplemented!();
}

fn resolve_incomplete_operands(instruction: &Instruction) -> Result<Vec<u32>, String> {
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

fn resolve_opcode(instr: &Instruction) -> Result<u32, &str> {
    // Get the opcode value from the instruction set table
    // Check format correctness
    match instruction_set::fetch_instruction(instr) {
        Ok(inst) => Ok(inst.op_code),
        Err(err) => Err(err),
    }
}

fn resolve_label(label: &str) -> Result<u32, &str> {
    // Check the symtab
    // Check the range of addresses with the instruction format
    unimplemented!();
}
