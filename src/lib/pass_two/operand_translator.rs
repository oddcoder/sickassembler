use instruction::Instruction;
use basic_types::formats::Format;
use basic_types::operands::Value;
use basic_types::register::Register;
use pass_one::pass_one::get_symbol;
use base_table::get_base_at;
use regex::Regex;
use literal_table::get_literal;
use super::super::{to_hex_string, remove_literal_container};

pub fn parse_operand(instruction: &mut Instruction, val: &Value) -> Result<String, String> {
    match *val {
        Value::None => Ok(String::new()),
        Value::Raw(x) => Ok(to_hex_string(x)),
        Value::SignedInt(x) => parse_signed_int(x),
        Value::Register(ref x) => parse_register(*x),
        Value::Label(ref lbl) => parse_label(instruction, lbl),
        Value::Bytes(ref text) => parse_bytes(instruction, text),
    }
}

fn parse_register(operand: Register) -> Result<String, String> {
    let reg_num = operand as u8;
    Ok(to_hex_string(reg_num as u32))
}

fn parse_signed_int(x: i32) -> Result<String, String> {
    if x > 0x7FFFFF {
        return Err("Value out of 23-bit range".to_string());
    }
    Ok(to_hex_string(x))
}

fn parse_label(instruction: &mut Instruction, lbl: &str) -> Result<String, String> {
    let sym_addr;
    match get_symbol(&lbl.to_owned()) {
        Ok(addr) => sym_addr = addr,
        Err(e) => return Err(e),
    }

    match get_disp(instruction, sym_addr) {
        Ok(addr) => return Ok(addr),
        Err(e) => {
            return Err(e);
        }
    };
}

fn parse_bytes(instruction: &mut Instruction, text: &str) -> Result<String, String> {
    if text.starts_with("=") {
        // Return the address of the literal, not its value
        let sym_addr = get_literal(text).unwrap().address as i32;
        match get_disp(instruction, sym_addr) {
            Ok(addr) => Ok(addr),
            Err(e) => {
                return Err(format!("{}", e.to_string()));
            }
        }
    } else {
        Ok(translate_literal(text))
    }
}


/// Converts the literal of the WORD/BYTE directive to object code
pub fn translate_literal(literal: &str) -> String {

    if literal.starts_with('X') || literal.starts_with('x') {
        // ex. INPUT BYTE X’F1’ -> F1
        let captures = HEX_REGEX.captures(literal).unwrap();
        let mut operand_match: String = captures.get(0).unwrap().as_str().to_owned();
        remove_literal_container(&mut operand_match);
        return operand_match;
    } else if literal.starts_with('C') || literal.starts_with('c') {
        let captures = STR_REGEX.captures(literal).unwrap();
        let mut operand_match: String = captures.get(0).unwrap().as_str().to_owned();
        remove_literal_container(&mut operand_match);

        return parse_str_operand(operand_match);
    } else {
        panic!("Invalid literal to translate {}, expected C|X'...' ",
               literal);
    }

}

fn parse_str_operand(operand_match: String) -> String {
    // EOF BYTE C’EOF’ -> 454F46
    operand_match.chars()
        .map(|c| to_hex_string(c as u32))
        .collect::<Vec<String>>()
        .join("")
}

fn get_disp(instruction: &mut Instruction, sym_addr: i32) -> Result<String, String> {

    if instruction.get_format() == Format::Four {
        if sym_addr > 0xFFFFF {
            return Err("Address is out of 20-bit range".to_owned());
        }
        return Ok(to_hex_string(sym_addr & 0xFFFF));
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

        if 0 <= disp && disp < 4096 {
            instruction.set_base_relative();
            final_disp = disp & 0xFFF;

        } else {
            return Err(format!("Address is out of base relative range disp:{:#X} base:{:#X} \
                                symbol addr:{:#X} , Instruction:{:?}",
                               disp,
                               base,
                               sym_addr,
                               instruction));
        }

    } else {
        return Err(format!("Address is out of PC relative range and no base is specified, \
                            Displacement:{:#X} Target Address:{:#X} {} Loc:{:#X}",
                           disp,
                           sym_addr,
                           instruction.mnemonic,
                           instruction.locctr));
    }

    panic_on_memory_limit(final_disp, instruction.locctr);
    return Ok(to_hex_string(final_disp));
}



fn panic_on_memory_limit(disp: i32, locctr: i32) {
    if disp + locctr >= (1 << 20) {
        panic!("Out of range address {}", disp + locctr)
    }
}

lazy_static!{
    // lazy_static regex to avoid recompilation on each function call -> read the docs
    static ref HEX_REGEX:Regex = Regex::new(r"^(x|X)'[0-9a-fA-F]+'").unwrap();
    static ref STR_REGEX: Regex = Regex::new(r"^(c|C)'.+'").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
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
