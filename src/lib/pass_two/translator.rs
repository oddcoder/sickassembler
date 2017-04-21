use super::super::{to_hex, string_from_object_code};
use instruction::Instruction;
use operands::Value;
use instruction_set::{self, AssemblyDef, is_action_directive};
use formats::*;
use semantics_validator;
use base_table::{set_base, end_base};
use std::u32;
use regex::Regex;

pub fn translate(instruction: &mut Instruction) -> Result<String, String> {

    let mut errs: Vec<String> = Vec::new();
    // TODO: check for action directives in the caller of this function
    // TODO: Check the flags for options
    // FIXME: handle base-relative addressing
    {
        if let Err(e) = semantics_validator::validate_semantics(instruction) {
            panic!("Symantic Error(s): {}", e);
        }
    }

    // Resolve operands first, in case of a directive, this function will return early
    let raw_operands: Result<String, String> = resolve_incomplete_operands(instruction);

    if is_directive(instruction) {
        return raw_operands;
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
        return Err(errs.join(", "));
    }

    // The operands are numeric if it's a normal instruction, not a directive
    let operands: u32 = u32::from_str_radix(&operands.unwrap(), 16)
        .expect("Failed to parse operand");

    let numeric_val = op_code.unwrap() + flags.unwrap();
    Ok(string_from_object_code(numeric_val + operands,
                               (get_bit_count(instruction.format) / 8) as u8))
}

/// Returns the hex value of operands
fn resolve_incomplete_operands(instruction: &Instruction) -> Result<String, String> {
    // Convert immediate and indirect operands to a basic forms -> Raw
    let mut raws: String = String::new();
    let op_vec = instruction.unwrap_operands();

    // TODO: indeirect and indexed operands
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
                println!("reg:{}", reg_num);
                to_hex(reg_num as u32)
            }
            // Get from symtab
            Value::Label(ref x) => {

                let sym_addr = resolve_label(x.as_str());

                if let Err(e) = sym_addr {
                    return Err(e.to_string());
                }

                match get_disp(instruction, sym_addr.unwrap()) {
                    Ok(addr) => addr,
                    Err(e) => return Err(e.to_string()),
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

    let op_code = instruction_set_def.get_opcode_value(instr.format);
    Ok(op_code)
}

/// Returns the location of the symbol from the
/// symtab, the result is returned as i32 (it'll be envolved in subtraction)
///  as it'll be subtracted from the locctr
fn resolve_label(label: &str) -> Result<i32, &str> {
    // TODO: Check the literal table
    // TODO: Check the symtab
    // TODO: Check the range of addresses with the instruction format
    if label.starts_with("=") {
        panic!("Literal!!");
    }
    unimplemented!();
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

fn resolve_base_directive(instr: &Instruction) {
    let mnemonic = instr.mnemonic.to_uppercase();
    let locctr = instr.locctr as u32;
    if mnemonic == "BASE" {
        let val = instr.get_first_operand().val;
        //set_base(locctr /**/);
    } else if mnemonic == "NOBASE" {
        end_base(locctr as u32);
    } else {
        panic!("Unknown instruction {:?}", instr);
    }
}

fn parse_str_operand(operand_match: String) -> String {
    // EOF BYTE C’EOF’ -> 454F46
    operand_match.chars()
        .map(|c| to_hex(c as u32))
        .collect::<Vec<String>>()
        .join("")
}

fn validate_instruction(instr: &mut Instruction) -> Result<(), String> {
    // TODO: aggregate errors
    // TODO: indexed addressing with PC/Base relative instructions and for format 4
    // TODO: handling base-relative adderssing
    // TODO: Check operands for the adressing mode

    if is_directive(instr) {
        return Ok(());
    }

    Ok(())
}

fn get_disp(instruction: &Instruction, sym_addr: i32) -> Result<String, &str> {
    // TODO: move to the instruction itself
    // If the instruction is format 4, return the address
    if instruction.format == Format::Four {
        if sym_addr > 0xFFFFF {
            return Err("Address is out of 20-bit range");
        }
        return Ok(to_hex(sym_addr));
    }

    // TODO: check the calculation and range
    let disp = (instruction.locctr + instruction.format as i32) - sym_addr;

    // TODO: Check for memory out of range error, using the locctr of instruction
    // if {

    // }
    // else
    if !(disp >= -2048 && disp <= 2047) {
        // TODO: check for base value
        // If failed, error
        return Err("Address is out of range");
    }


    // Take the last 20 bits of the number
    return Ok(to_hex(disp));
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
    use basic_types::formats;
    use basic_types::unit_or_pair::UnitOrPair;
    use basic_types::operands::{Value, OperandType};
    use basic_types::register::Register;
    use basic_types::instruction::AsmOperand;
    #[test]
    fn test_resolve_op_code() {
        let mut inst = Instruction::new_simple("ldx".to_owned());

        inst.format = formats::Format::One;
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x04);

        inst.format = formats::Format::Three;
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x040000);

        inst.format = formats::Format::Four;
        assert!(resolve_opcode(&inst).unwrap() == 0x04000000);
    }

    #[test]
    fn test_resolve_regs() {
        let inst =
            Instruction::new(String::new(),
                             "add".to_owned(),
                             UnitOrPair::Unit(AsmOperand::new(OperandType::Register,
                                                              Value::Register(Register::B))));

        let opr: String = resolve_incomplete_operands(&inst).unwrap();
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
}
