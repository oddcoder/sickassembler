use super::super::string_from_object_code;
use instruction::Instruction;
use operands::Value;
use instruction_set::{self, AssemblyDef, is_base_mode_directive, is_decodable_directive};
use semantics_validator;
use base_table::{set_base, end_base};
use symbol_tables::get_symbol;
use symbol::SymbolType;
use pass_two::operand_translator::parse_operand;
use std::u32;


use super::super::RawProgram;

/// Returns the errors
pub fn pass_two(prog: &mut RawProgram) -> Vec<String> {

    let mut errs: Vec<String> = Vec::new();

    for &mut (ref mut obj_code, ref mut instr) in prog.program.iter_mut() {
        // TODO: add obj code
        match translate(instr) {
            Ok(obj) => *obj_code = obj,
            Err(e) => errs.push(e),
        }
    }

    errs
}

fn translate(instruction: &mut Instruction) -> Result<String, String> {

    let mut errs: Vec<String> = Vec::new();

    if let Err(e) = semantics_validator::validate_semantics(instruction) {
        errs.push(format!("Semantic Error(s): {} \n {:?} \n\n", e, instruction));
    }

    // Resolve operands first, in case of a directive, this function will return early
    if is_base_mode_directive(&instruction.mnemonic).is_some() {
        // Add the base entry
        match resolve_base_directive(instruction) {
            Ok(_) => return Ok(String::new()),
            Err(e) => return Err(e),
        }
    }

    let raw_operands: Result<String, String>;
    raw_operands = resolve_incomplete_operands(instruction);

    if is_decodable_directive(&instruction.mnemonic) {
        return raw_operands;
    } else if is_directive(instruction) && !is_decodable_directive(&instruction.mnemonic) {
        return Ok(String::new());
    }
    // Assemble the instruciton
    // Operand field in the hex code
    let raw_opcode: Result<u32, &str> = resolve_opcode(instruction);
    let raw_flags: Result<u32, String> = instruction.get_flags_value();

    debug!("Tranlating instruction {:?}", instruction);
    debug!("Raw instruction operands {:?}", raw_operands);
    debug!("Raw flag value {:?}", raw_flags);
    debug!("Instruction opcode {:?}", raw_opcode);

    let op_code = raw_opcode.map_err(|e| errs.push(e.to_owned()));
    let operands = raw_operands.map_err(|e| errs.push(e));
    let flags = raw_flags.map_err(|e| errs.push(e));

    if errs.len() > 0 {
        return Err(errs.join("\n "));
    }

    // The operands are numeric if it's a normal instruction, not a directive
    let operands: u32 = u32::from_str_radix(&operands.unwrap(), 16)
        .map_err(|e| errs.push("Failed to parse operand ".to_owned() + &e.to_string()))
        .unwrap_or(0);

    let numeric_val = op_code.unwrap() + flags.unwrap();
    Ok(string_from_object_code(numeric_val + operands, (instruction.get_format()) as u8))
}

/// Returns the hex value of operands
fn resolve_incomplete_operands(instruction: &mut Instruction) -> Result<String, String> {
    // Convert immediate and indirect operands to a basic forms -> Raw
    let mut raws: String = String::new();
    let mut errs: Vec<String> = Vec::new();
    let op_vec = instruction.unwrap_operands();

    for operand in &op_vec {
        // TODO: do the same with this as the operand_parser
        match parse_operand(instruction, &operand.val) {
            Ok(mut raw) => raws.push_str(&mut raw),
            Err(e) => errs.push(e),
        }
    }

    if errs.len() > 0 {
        return Err(format!("Found error while parsing operands {}", errs.join("\n ")));
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

    let op_code = instruction_set_def.get_opcode_value(instr.get_format());
    Ok(op_code as u32)
}

fn resolve_base_directive(instr: &Instruction) -> Result<(), String> {
    let mnemonic = instr.mnemonic.to_uppercase();
    let locctr = instr.locctr;

    if mnemonic == "BASE" {
        if let Value::Label(val) = instr.get_first_operand().val {

            /// Returns the location of the symbol from the
            /// symtab, the result is returned as i32 (it'll be envolved in subtraction)
            ///  as it'll be subtracted from the locctr
            match get_symbol(&val, &instr.csect) {
                Ok(sym) => {
                    if sym.symbol_type == SymbolType::Imported {
                        return Err(format!("Base can't be an imported symbol {{ {:?} }}", instr));
                    }
                    set_base(locctr, sym.get_address())
                }
                Err(e) => return Err(format!("Invalid base {} {}", val, e)),
            }
        }
    } else if mnemonic == "NOBASE" {
        end_base(locctr);
    } else {
        return Err(format!("Unknown instruction {:?}", instr));
    }
    Ok(())
}

fn is_directive(instr: &Instruction) -> bool {
    if let Ok(_) = instruction_set::fetch_directive(&instr.mnemonic) {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use formats::{self, Format};
    use unit_or_pair::UnitOrPair;
    use operands::{Value, OperandType};
    use register::Register;
    use instruction::AsmOperand;

    #[test]
    fn test_resolve_op_code() {
        let mut inst = Instruction::new_simple("ldx".to_owned());

        inst.set_format(formats::Format::One);
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x04);

        inst.set_format(formats::Format::Three);
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x040000);

        inst.set_format(formats::Format::Four);
        assert!(resolve_opcode(&inst).unwrap() == 0x04000000);
    }

    #[test]
    fn test_resolve_regs() {
        let mut inst =
            Instruction::new(String::new(),
                             "add".to_owned(),
                             UnitOrPair::Unit(AsmOperand::new(OperandType::Register,
                                                              Value::Register(Register::B))));

        let opr: String = resolve_incomplete_operands(&mut inst).unwrap();
        assert_eq!(opr, "3");
    }

    #[test]
    fn translate_correct() {
        let mut instrs = vec![
                                                                     create_instruction("comp",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                                     Value::SignedInt(0))),
                                                                     Format::Three),
                                    
                                                                     create_instruction("TIXR",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Register,
                                                                     Value::Register(Register::T))),
                                                                     Format::Two),
                                                                     create_instruction("LDA",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                                     Value::SignedInt(3))),
                                                                     Format::Three),
                                                                     create_instruction("LDT",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                                     Value::SignedInt(4096))),
                                                                     Format::Four)
                                    ];

        assert_eq!(translate(&mut instrs[0]).unwrap(), "290000");
        assert_eq!(translate(&mut instrs[1]).unwrap(), "B850");
        assert_eq!(translate(&mut instrs[2]).unwrap(), "010003");
        assert_eq!(translate(&mut instrs[3]).unwrap(), "75101000");
    }

    fn create_instruction(mnemonic: &str,
                          operands: UnitOrPair<AsmOperand>,
                          format: Format)
                          -> Instruction {

        let mut instr = Instruction::new(String::new(), mnemonic.to_owned(), operands);
        instr.set_format(format);
        instr
    }
}
