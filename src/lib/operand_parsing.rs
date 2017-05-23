/// This code is composed accroding to the parse decision tree
/// of the SIC/XE
/// The code coould've been better i.e -thing(x).or_else(|_| another_thing(x))-
/// but for the sake of error aggregation

use operands::{OperandType, Value};
use register::Register;
use instruction::AsmOperand;
use literal_table::insert_unresolved;
use super::*;
use std::i32;


pub fn parse_directive_operand(op: &str, instruction: &str) -> Result<AsmOperand, String> {
    let mut errs: String = String::new();
    let inst = instruction.to_uppercase();

    let result = parse_bytes(op)
        .or_else(|e| {
            errs = format!("{}", e);
            // RESW/B
            if inst == "RESB" || inst == "RESW" || inst == "WORD" {
                parse_signed_int(op)
            } else {
                Err("Not RESB/W or WORD".to_owned())
            }

        })
        .or_else(|e| {
            // START / END
            errs = format!("{}\n{}", errs, e);
            if inst == "START" || inst == "END" {
                parse_hex(op)
            } else {
                Err("Not START/END".to_owned())
            }
        })
        .or_else(|e| {
            // BASE / NOBASE
            errs = format!("{}\n{}", errs, e);
            parse_label(op, OperandType::None)
        })
        .or_else(|e| {
            errs = format!("{}\n{}", errs, e);
            if inst == "EQU"{
                parse_signed_int(op)
                .or_else(|e| {
                    errs = format!("{}\n{}", errs, e);
                    parse_instruction_operand(op)
                })
                .or_else(|e| {
                    errs = format!("{}\n{}", errs, e);
                    parse_expression(op)
                })
            }
            else {
                Err("not EQU".to_owned())
            }

        });

    match result {
        Ok(r) => return Ok(r),
        Err(_) => return Err(errs),
    };
}

pub fn parse_instruction_operand(op: &str) -> Result<AsmOperand, String> {
    let mut errs: String = String::new();
    let result = parse_register(op)
        .or_else(|e| {
            errs = format!("{}", e);
            parse_hex(op)
        })
        .or_else(|e| {
            errs = format!("{}\n{}", errs, e);
            parse_memory_operand(op)
        });
    match result {
        Ok(r) => return Ok(r),
        Err(_) => return Err(errs),
    };
}

/// Occurs when: Instruction -> F3 / F4
fn parse_memory_operand(op: &str) -> Result<AsmOperand, String> {
    let prefix = &op[0..1];
    let content = &op[1..];
    match prefix {
        "#" => parse_label(content, OperandType::Immediate).or_else(|_| parse_signed_int(content)),
        "@" => parse_label(content, OperandType::Indirect),
        "=" => parse_literal(op),
        _ => {
            // Label
            parse_label(op, OperandType::Label)
        }
    }

}

/// Occurs when: Instruction -> F3 / F2 / F1
fn parse_register(op: &str) -> Result<AsmOperand, String> {
    match op {
        "A" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::A))),
        "X" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::X))),
        "L" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::L))),
        "B" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::B))),
        "S" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::S))),
        "T" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::T))),
        "F" => Ok(AsmOperand::new(OperandType::Register, Value::Register(Register::F))),
        _ => Err(format!("{} is not a register", op)),
    }
}

/// Occurs when: Inst-> F3 Immediate
fn parse_signed_int(op: &str) -> Result<AsmOperand, String> {
    match i32::from_str_radix(&op, 10) {
        Ok(hex) => Ok(AsmOperand::new(OperandType::Immediate, Value::SignedInt(hex))),
        Err(e) => Err(e.to_string()),
    }
}

/// Occurs when: Directive -> C
fn parse_bytes(op: &str) -> Result<AsmOperand, String> {
    if is_ascii_or_word_operand(op) {
        return Ok(create_operand(OperandType::Bytes, Value::Bytes(op.to_owned())));
    }
    Err(format!("isn't on the form of C|X'...'"))
}

/// Occurs when: Instruction -> F3 -> memory -> label , Directive -> label i.e BASE/NOBASE
fn parse_label(op: &str, t: OperandType) -> Result<AsmOperand, String> {

    if op.starts_with("X'") && op.ends_with("'") {

        // Immediate hex -> #X'F1' ( the # is removed by the caller )
        // Convert to immediate decimal as the type Raw isn't supported by F3/F4 instructions
        let op = &mut op.to_owned();
        remove_literal_container(op);

        match u32::from_str_radix(&op, 16) {
            Ok(decimal) => {
                return Ok(create_operand(OperandType::Immediate, Value::SignedInt(decimal as i32)))
            }

            Err(e) => return Err(e.to_string()),
        }

    } else if is_label(op) {
        return Ok(create_operand(t, Value::Label(op.to_owned())));
    }

    Err(format!("{} Isn't a label", op))
}

/// Occurs when: Directive -> Start/End OR F2 with n -> Shl / Shr / SVC
/// Converts a hexadecimal string to a raw operand
fn parse_hex(op: &str) -> Result<AsmOperand, String> {
    match u32::from_str_radix(&op, 16) {
        Ok(hex) => Ok(create_operand(OperandType::Raw, Value::Raw(hex))),
        Err(e) => Err(e.to_string()),
    }
}

/// Occurs when: Inst -> F3 , i.e =(X|C)'...'
fn parse_literal(op: &str) -> Result<AsmOperand, String> {
    if !is_literal(op) {
        return Err(format!("Invalid literal {}", op));
    }
    insert_unresolved(&op.to_owned());
    Ok(create_operand(OperandType::Label, Value::Bytes(op.to_owned())))
}

fn create_operand(t: OperandType, v: Value) -> AsmOperand {
    AsmOperand::new(t, v)
}

pub fn parse_ref_operands(ops: Vec<String>) -> AsmOperand {
    return AsmOperand::new(OperandType::VarArgs, Value::VarArgs(ops));
}

//parses expression operands
fn parse_expression(op:&str)-> Result<AsmOperand, String> {
    if is_expression(op) {
        let labels = capture_expression(op);
        println!("{:?}", labels);
        return Ok(create_operand(OperandType::Expression, Value::Expression(labels)));
    }
    else{
        return Err(format!("{} is not an expression.", op));
    }
}

//returns expression to be computed and labels therein
fn capture_expression(op:&str)->Vec<String>{
    let matches = EXPRESSION.captures(op).unwrap();
    let mut terms_vector = Vec::new();
    for a_match in matches.iter() {
        let term = a_match.map_or("", |m| m.as_str());
        //skipping empty captures from repeated groups
        if term != ""{
            terms_vector.push(String::from(term))
        }
    }
    return terms_vector
}
