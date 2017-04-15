use basic_types::instruction::Instruction;
use basic_types::operands::Value;
use basic_types::instruction_set::{self, AssemblyDef};
use std::fmt::UpperHex;
use std::marker::Sized;
use regex::Regex;

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

fn resolve_incomplete_operands(instruction: &Instruction) -> Result<String, &str> {
    // Convert immediate and indirect operands to a basic forms -> Raw
    let mut raws: String = String::new();
    let op_vec = instruction.unwrap_operands();

    for operand in &op_vec {
        let raw: Result<String, &str> = match operand.val {
            Value::SignedInt(x) => {
                if x > 0x7FFFFF {
                    return Err("Value out of 23-bit range");
                }
                Ok(to_hex(x))
            }
            Value::Register(ref x) => {
                let reg_num = *x as u8;
                Ok((reg_num as u32).to_string())
            }
            // Get from symtab
            Value::Label(ref x) => resolve_label(x.as_str()),
            Value::Raw(x) => Ok(x.to_string()),
            // Used by WORD / BYTE -> Generate hex codes for operand
            Value::Bytes(ref text) => resolve_directive_operand(text),
        };

        if raw.is_err() {
            return Err("Couldn't resolve label"); // Nothing else can fail
        }
        let mut operand: String = raw.unwrap();
        raws.push_str(&mut operand);
    }
    Ok(raws)
}

/// Get the opcode value from the instruction set table
fn resolve_opcode(instr: &Instruction) -> Result<String, &str> {

    let instruction_set_def: AssemblyDef;

    match instruction_set::fetch_instruction(&instr.mnemonic) {
        Ok(inst) => instruction_set_def = inst,
        Err(err) => return Err(err),
    };

    Ok(to_hex(instruction_set_def.op_code))
}

fn resolve_label(label: &str) -> Result<String, &str> {
    // TODO: Check the symtab
    // TODO: Check the range of addresses with the instruction format
    unimplemented!();
}

/// Converts the operand of the WORD/BYTE directive to object code
fn resolve_directive_operand(operand: &String) -> Result<String, &str> {
    // TODO: lazy_static the regex
    let hex_regex: Regex = Regex::new(r"^(x|X)'[0-9a-fA-F]+'").unwrap();
    let str_regex: Regex = Regex::new(r"^(c|C)'.+'").unwrap();

    if hex_regex.is_match(operand) == false && str_regex.is_match(operand) == false {
        return Err("Operand isn't on the correct format");
    }

    if operand.starts_with('X') || operand.starts_with('x') {
        // ex. INPUT BYTE X’F1’ -> F1
        let captures = hex_regex.captures(operand.as_str()).unwrap();
        let mut operand_match: String = captures.get(0).unwrap().as_str().to_owned();
        remove_container(&mut operand_match);
        return Ok(operand_match);
    } else {
        let captures = str_regex.captures(operand.as_str()).unwrap();
        let mut operand_match: String = captures.get(0).unwrap().as_str().to_owned();
        remove_container(&mut operand_match);

        return Ok(parse_str_operand(operand_match));
    }
}

fn parse_str_operand(operand_match: String) -> String {
    // EOF BYTE C’EOF’ -> 454F46
    operand_match.chars()
        .map(|c| to_hex(c as u32))
        .collect::<Vec<String>>()
        .join("")

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

/// Removes the container of a WORD/BYTE oeprand, the prefix, the '
/// X'asdas' -> asdas ,and so on
fn remove_container(byte_operand: &mut String) {
    byte_operand.remove(0);
    byte_operand.remove(0);
    byte_operand.pop();
}

fn to_hex<T>(num: T) -> String
    where T: UpperHex + Sized
{
    format!("{:X}", num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use basic_types::formats;
    use basic_types::operands::{Value, OperandType};
    use basic_types::register::Register;

    #[test]
    fn test_resolve_op_code() {
        let mut inst = Instruction::new_simple("add".to_owned());
        inst.format = formats::Format::Four;
        assert_eq!(resolve_opcode(&inst).unwrap(), "18");

        inst.format = formats::Format::Three;
        assert!(resolve_opcode(&inst).unwrap() == "18");
    }

    #[test]
    fn test_resolve_regs() {
        let mut inst = Instruction::new_simple("add".to_owned());
        inst.add_operand(OperandType::Register, Value::Register(Register::A));
        let opr: String = resolve_incomplete_operands(&inst).unwrap();
        assert_eq!(opr, "0");
    }

    #[test]
    fn test_byte_operand_failing() {
        let test_str: String = "abc'".to_owned(); // Malformed
        let test_str_1: String = "'abc".to_owned(); // Malformed
        let test_str_2: String = "X'abc".to_owned(); // Malformed
        let test_str_3: String = "C'abc".to_owned(); // Malformed

        let result = resolve_directive_operand(&test_str);
        let result_1 = resolve_directive_operand(&test_str_1);
        let result_2 = resolve_directive_operand(&test_str_1);
        let result_3 = resolve_directive_operand(&test_str_1);

        assert!(result.is_err());
        assert!(result_1.is_err());
        assert!(result_2.is_err());
        assert!(result_3.is_err());
    }

    #[test]
    fn test_byte_operand_parsing() {

        check_str_operand("x'0A'", "0A");
        check_str_operand("x'FF'", "FF");

        check_str_operand("C'cab'", "636162");
        check_str_operand("C'EOF'", "454F46");
    }

    fn check_str_operand(x: &str, v: &str) {
        let result = resolve_directive_operand(&x.to_owned())
            .expect(format!("Failed to parse {}", x).as_str());
        assert_eq!(result.to_uppercase(), v.to_uppercase());
    }

    #[test]
    fn check_string_convert_to_vec() {
        let test_str: String = "abc".to_owned();
        let result = parse_str_operand(test_str);
        assert_eq!(result, "616263");
    }
}
