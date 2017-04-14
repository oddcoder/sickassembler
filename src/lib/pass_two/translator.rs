use basic_types::flags::*;
use basic_types::instruction::{Instruction, AsmOperand};
use basic_types::operands::{OperandType, Value};
use basic_types::formats::*;
use basic_types::instruction_set::{self, AssemblyDef};
use basic_types::unit_or_pair::UnitOrPair;
use std::num;

pub fn translate(instruction: &Instruction) -> Result<u32, &str> {
    //let f_vals = instruction.check_invalid_flags();   // TODO Report to RLS
    //resolve_instruction_code(instruction, 0).and_then(resolve_operands)

    // Check the flags for options
    // TODO: Check for memory out of range error, using the locctr of instruction
    // TODO: check for base value

    //validate_instruction().unwrap_or();

    // Assemble the instruciton
    let raw_operands = resolve_incomplete_operands(instruction);
    let raw_opcode = resolve_opcode(instruction);
    let raw_flags = instruction.get_flags_value(); // TODO propagate the error from getting flag values



    debug!("Tranlating instruction {:?}", instruction);
    debug!("Raw instruction operands {:?}", raw_operands);
    debug!("Raw flag value {:?}", raw_flags);
    debug!("Instruction opcode {:?}", raw_opcode);

    // TODO: extract error message
    // TODO: combine results

    unimplemented!()
}

fn resolve_incomplete_operands(instruction: &Instruction) -> Result<Vec<u32>, &str> {
    // Convert immediate and indirect operands to a basic forms -> Raw
    let mut raws: Vec<u32> = Vec::new();
    let opVec = instruction.unwrap_operands();

    for operand in &op_vec {
        let raw: Result<Vec<u32>, &str> = match operand.val {
            Value::SignedInt(x) => Ok(vec![x.abs() as u32]),
            Value::Register(ref x) => Ok(vec![(*x as u8) as u32]),
            // Get from symtab
            Value::Label(ref x) => resolve_label(x.as_str()).and_then(|v| Ok(vec![v])),
            Value::Raw(x) => Ok(vec![x]),
            Value::Bytes(ref text) => resolve_directive_operand(text),
        };

        if raw.is_err() {
            return Err("Couldn't resolve label"); // Nothing else can fail
        }
        let mut operand: Vec<u32> = raw.unwrap();
        raws.append(&mut operand);
    }
    Ok(raws)
}

/// Get the opcode value from the instruction set table
fn resolve_opcode(instr: &Instruction) -> Result<u32, &str> {

    let instruction_set_def: AssemblyDef;

    match instruction_set::fetch_instruction(&instr.mnemonic) {
        Ok(inst) => instruction_set_def = inst,
        Err(err) => return Err(err),
    };

    Ok(instruction_set_def.op_code)
}

fn resolve_label(label: &str) -> Result<u32, &str> {
    // TODO: Check the symtab
    // TODO: Check the range of addresses with the instruction format
    unimplemented!();
}

fn resolve_directive_operand(operand: &String) -> Result<Vec<u32>, &str> {
    unimplemented!()
}

fn validate_instruction(instr: &Instruction) -> Result<(), &str> {
    // TODO: aggregate errors
    // TODO: indexed addressing with PC/Base relative instructions and for format 4
    // Check format correctness
    let instruction_set_def: AssemblyDef;

    // Check mnemonic existence
    match instruction_set::fetch_instruction(&instr.mnemonic) {
        Ok(expr) => instruction_set_def = expr,
        Err(e) => return Err(e),
    }

    // Check format correctness
    if instruction_set_def.match_format(&instr.format) == false {
        return Err("Formats mismatched");
    }

    // Check operands
    if instruction_set_def.has_valid_operands(&instr.operands) == false {
        return Err("Operands for this mnemonic are invalid");
    }

    // Check memory range

    if instruction_set_def.match_format(&instr.format) == false {
        return Err("Mismatched instruction formats");
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use basic_types::formats;
    use basic_types::operands::Value;
    use basic_types::register::Register;

    #[test]
    fn test_resolve_op_code() {
        let mut inst = Instruction::new_simple("add".to_owned());
        inst.format = formats::Format::Four;
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x18);

        inst.format = formats::Format::Three;
        assert!(resolve_opcode(&inst).unwrap() == 0x18);
    }

    #[test]
    fn add_format_one() {
        let mut inst = Instruction::new_simple("add".to_owned());
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x18);
    }

    #[test]
    fn test_resolve_regs() {
        let mut inst = Instruction::new_simple("add".to_owned());
        inst.add_operand(OperandType::Register, Value::Register(Register::A));
        let oprs: Vec<u32> = resolve_incomplete_operands(&inst).unwrap();

        assert_eq!(oprs.len(), 1);
        assert_eq!(oprs[0], 0);
    }

}
