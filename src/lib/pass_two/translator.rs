use super::super::{to_hex, string_from_object_code};
use instruction::Instruction;
use operands::Value;
use instruction_set::{self, AssemblyDef, is_base_mode_directive, is_decodable_directive};
use formats::*;
use semantics_validator;
use base_table::{set_base, end_base, get_base_at};
use pass_one::pass_one::get_symbol;
use std::u32;
use regex::Regex;
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
    // TODO: check for action directives in the caller of this function
    // TODO: Check the flags for options
    // FIXME: handle base-relative addressing
    {
        if let Err(e) = semantics_validator::validate_semantics(instruction) {
            errs.push(format!("Symantic Error(s): {} \n {:?} \n\n", e, instruction));
        }
    }

    // Resolve operands first, in case of a directive, this function will return early

    if is_base_mode_directive(&instruction.mnemonic).is_some() {
        // Add the base entry
        resolve_base_directive(instruction);
        return Ok(String::new());
    }

    let raw_operands: Result<String, String>;
    {
        raw_operands = resolve_incomplete_operands(instruction);
    }


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
    let op_vec = instruction.unwrap_operands();

    for operand in &op_vec {
        let mut raw: String = match operand.val {
            Value::None => String::new(),
            Value::SignedInt(x) => {
                if x > 0x7FFFFF {
                    return Err("Value out of 23-bit range".to_string());
                }
                to_hex(x)
            }
            Value::Register(ref x) => {
                let reg_num = *x as u8;
                to_hex(reg_num as u32)
            }
            // Get from symtab
            Value::Label(ref lbl) => {

                let sym_addr;
                match get_symbol(&lbl.to_owned()) {
                    Some(addr) => sym_addr = addr,
                    None => return Err(format!("Symbol not found {{ {} }}", lbl)),
                }

                match get_disp(instruction, sym_addr) {
                    Ok(addr) => addr,
                    Err(e) => {
                        return Err(format!("{}", e.to_string()));
                    }
                }

            }
            Value::Raw(x) => to_hex(x),
            // Used by WORD / BYTE -> Generate hex codes for operand
            Value::Bytes(ref text) => translate_literal(text),
        };
        raws.push_str(&mut raw);
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

fn resolve_base_directive(instr: &Instruction) {
    let mnemonic = instr.mnemonic.to_uppercase();
    let locctr = instr.locctr;

    if mnemonic == "BASE" {
        if let Value::Label(val) = instr.get_first_operand().val {
            match resolve_label(&val) {
                Ok(addr) => set_base(locctr, addr),
                Err(e) => panic!("Invalid base {}", e),
            }
        }
    } else if mnemonic == "NOBASE" {
        end_base(locctr);
    } else {
        panic!("Unknown instruction {:?}", instr);
    }
}

/// Returns the location of the symbol from the
/// symtab, the result is returned as i32 (it'll be envolved in subtraction)
///  as it'll be subtracted from the locctr
fn resolve_label(label: &str) -> Result<i32, &str> {
    match get_symbol(&label.to_owned()) {
        Some(addr) => Ok(addr),
        None => Err("Symbol not found"),
    }
}

/// Converts the literal of the WORD/BYTE directive to object code
pub fn translate_literal(lit: &String) -> String {
    let mut literal = lit.clone();
    // TODO: cleanase the design of the string parser
    if literal.starts_with("=") {
        literal.drain(0..1);
    }

    if literal.starts_with('X') || literal.starts_with('x') {
        // ex. INPUT BYTE X’F1’ -> F1
        let captures = HEX_REGEX.captures(literal.as_str()).unwrap();
        let mut operand_match: String = captures.get(0).unwrap().as_str().to_owned();
        remove_container(&mut operand_match);
        return operand_match;
    } else if literal.starts_with('C') || literal.starts_with('c') {
        let captures = STR_REGEX.captures(literal.as_str()).unwrap();
        let mut operand_match: String = captures.get(0).unwrap().as_str().to_owned();
        remove_container(&mut operand_match);

        return parse_str_operand(operand_match);
    } else {
        panic!("Invalid literal to translate {}, expected C|X'...' ",
               literal);
    }

}

fn is_directive(instr: &Instruction) -> bool {
    if let Ok(_) = instruction_set::fetch_directive(&instr.mnemonic) {
        return true;
    }
    return false;
}

fn parse_str_operand(operand_match: String) -> String {
    // EOF BYTE C’EOF’ -> 454F46
    operand_match.chars()
        .map(|c| to_hex(c as u32))
        .collect::<Vec<String>>()
        .join("")
}

fn get_disp(instruction: &mut Instruction, sym_addr: i32) -> Result<String, String> {

    // TODO: move to the instruction itself
    // If the instruction is format 4, return the address
    if instruction.get_format() == Format::Four {
        if sym_addr > 0xFFFFF {
            return Err("Address is out of 20-bit range".to_owned());
        }
        return Ok(to_hex(sym_addr & 0xFFFF));
    }

    let final_disp: i32;
    let disp: i32 = sym_addr - (instruction.locctr + instruction.get_format() as i32);
    let base = get_base_at(instruction.locctr as u32);

    // PC relative is invalid
    if -2048 <= disp && disp < 2048 {

        instruction.set_pc_relative();
        final_disp = disp & 0xFFF;

    } else if base.is_some() {

        let base = base.unwrap() as i32;
        let disp = sym_addr - base;

        if 0 <= base && base < 4096 {
            instruction.set_base_relative();
            final_disp = disp & 0xFFF;

        } else {
            return Err(format!("Address is out of base relative range disp:{:X} base:{:X} \
                                symbol addr:{:X} , Instruction:{:?}",
                               disp,
                               base,
                               sym_addr,
                               instruction));
        }

    } else {
        return Err(format!("Address is out of PC relative range and no base is specified, \
                            Displacement:{:X} Target Address:{:X} {} Loc:{:X}",
                           disp,
                           sym_addr,
                           instruction.mnemonic,
                           instruction.locctr));
    }

    panic_on_memory_limit(final_disp, instruction.locctr);
    return Ok(to_hex(final_disp));
}



fn panic_on_memory_limit(disp: i32, locctr: i32) {
    if disp + locctr >= (1 << 20) {
        panic!("Out of range address {}", disp + locctr)
    }
}

/// Removes the container of a WORD/BYTE oeprand, the prefix, the '
/// X'asdas' -> asdas ,and so on
fn remove_container(byte_operand: &mut String) {
    byte_operand.remove(0);
    byte_operand.remove(0);
    byte_operand.pop();
}

lazy_static!{
    // lazy_static regex to avoid recompilation on each function call -> read the docs
    static ref HEX_REGEX:Regex = Regex::new(r"^(x|X)'[0-9a-fA-F]+'").unwrap();
    static ref STR_REGEX: Regex = Regex::new(r"^(c|C)'.+'").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use formats;
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
    fn test_byte_operand_parsing() {

        check_str_operand("x'0A'", "0A");
        check_str_operand("x'FF'", "FF");

        check_str_operand("C'cab'", "636162");
        check_str_operand("C'EOF'", "454F46");
    }

    fn check_str_operand(x: &str, v: &str) {
        let result = translate_literal(&x.to_owned());
        assert_eq!(result.to_uppercase(), v.to_uppercase());
    }

    #[test]
    fn check_string_convert_to_vec() {
        let test_str: String = "abc".to_owned();
        let result = parse_str_operand(test_str);
        assert_eq!(result, "616263");
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
