use std::collections::HashMap;
use instruction::*;
use formats::Format;
use unit_or_pair::*;
use parking_lot::RwLock;
use operands::*;
use literal::Literal;
use literal_table::{insert_literal, get_unresolved, get_literal};
use std::u32;
use super::super::RawProgram;

fn get_instruction_size(inst: &Instruction) -> i32 {
    match inst.get_format() {
        Format::One => return 1,
        Format::Two => return 2,
        Format::Three => return 3,
        Format::Four => return 4,
        Format::None => (),
    }
    match &*inst.mnemonic.to_uppercase() {

        "BYTE" => {
            let operands = unwrap_to_vec(&inst.operands);
            if operands.len() != 1 {
                panic!("BYTE expects only 1 operand");
            }

            let instr_len = match operands[0].val {
                Value::Raw(_) => 1,
                Value::SignedInt(_) => 1,
                Value::Bytes(ref x) => {
                    if x.starts_with("X") {
                        let len: f32 = ((x.len() - 3) as f32 / 2.0) as f32;
                        len.ceil() as i32
                    } else if x.starts_with("C") {
                        x.len() as i32 - 3
                    } else if x.starts_with("=") {
                        get_literal(x).unwrap().length_in_bytes()
                    } else {
                        panic!("Unexpected Error {:?}", *inst)
                    }
                }
                _ => panic!("Unexpected Error {:?}", *inst),
            };
            return instr_len;
        }
        "WORD" => return 3,
        "RESB" => {
            let operands = unwrap_to_vec(&inst.operands);
            if operands.len() != 1 {
                panic!("RESB expects only 1 operand");
            }
            match operands[0].val {
                Value::SignedInt(x) => return x,
                _ => panic!("Unexpected Error {:?}", *inst),
            }
        }
        "RESW" => {
            let operands = unwrap_to_vec(&inst.operands);
            if operands.len() != 1 {
                panic!("RESW expects only 1 operand");
            }
            match operands[0].val {
                Value::SignedInt(x) => return x * 3,
                _ => panic!("Unexpected Error - find_instr_length {:?}", operands),
            }
        }
        _ => (),
    }
    return 0;
}

pub fn pass_one(prog_info: Result<(RawProgram, usize), String>)
                -> Result<(HashMap<String, i32>, RawProgram), String> {

    // TODO: replace the literal in an instruction operand with the literal label
    // if let Value::Bytes(ref x) = instruction.get_first_operand().val {}

    if let Err(e) = prog_info {
        return Err(e);
    }
    let mut errs: Vec<String> = Vec::new();
    let (prog, loc) = prog_info.unwrap();
    let mut loc = loc as i32;
    let mut prog: RawProgram = prog;
    let temp_instructions: Vec<Instruction>;

    {
        // Move the instructions into a temp storage
        // to allow for adding literals
        temp_instructions = prog.program
            .into_iter()
            .map(move |t: (_, Instruction)| t.1)
            .collect::<Vec<Instruction>>();
    }

    let mut instructions: Vec<Instruction> = Vec::new();

    for mut instruction in temp_instructions {
        instruction.locctr = loc;

        if !instruction.label.is_empty() {
            if let Err(e) = insert_symbol(&instruction.label, loc) {
                errs.push(format!("{}", e));
            }
        }

        if let Some(op) = has_literal(&instruction) {
            insert_unresolved(&(op.to_owned()));
        }

        if instruction.mnemonic.to_uppercase() == "LTORG" {
            loc = flush_literals(&mut instructions, loc as u32);
        } else {
            loc += get_instruction_size(&instruction);
            instructions.push(instruction.clone());
        }

        if instruction.mnemonic.to_uppercase() == "END" {
            match parse_end(&instruction) {
                Ok(end) => {
                    prog.starting_address = end;
                    prog.program_length = (loc as u32) - prog.first_instruction_address;
                }
                Err(e) => errs.push(e),
            }

        }
    }

    // Flush remaining literals
    flush_literals(&mut instructions, loc as u32);

    if prog.program_length == u32::MAX {
        errs.push(format!("Couldn't find the END instruction"));
    }

    // Move the instructions back
    prog.program = instructions.into_iter()
        .map(|i| (String::new(), i))
        .collect::<Vec<(_, Instruction)>>();

    if errs.len() != 0 {
        return Err(errs.join("\n "));
    }

    Ok((get_all_symbols(), prog))
}

fn flush_literals(instructions: &mut Vec<Instruction>, start_loc: u32) -> i32 {
    // TODO: fix literals
    // TODO: insert in instruction vector
    let mut loc = start_loc;
    for lit in get_unresolved() {
        insert_literal(&lit, loc);
        // literal declaration to be inserted in code
        let lit_decl: Instruction = *create_from_literal(&lit, loc as i32);
        let lit_sz = get_instruction_size(&lit_decl) as u32;
        loc += lit_sz;

        let lit_addr = lit_decl.locctr;
        instructions.push(lit_decl);

        // Add literals to symbol table
        insert_symbol(&lit, lit_addr);

    }
    loc as i32
}

/// Gets the address of the first executable isntruction
fn parse_end(instruction: &Instruction) -> Result<u32, String> {
    // TODO: change read_start to read boundary START/END
    // or replace with is action directive
    // check for instructions after END ( not applicable )
    let operands = unwrap_to_vec(&instruction.operands);

    if operands.len() != 0 {
        if let Value::Raw(op_end) = operands[0].val {
            // Will panic on negative value
            Ok(op_end as u32)
        } else {
            Err(format!("Invalid END operands, found {:?}", operands))
        }
    } else {
        // End operand isn't specified, default: program start address
        Ok((instruction.locctr) as u32)
    }

}

fn create_from_literal(lit: &String, locctr: i32) -> Box<Instruction> {
    //Instruction::new(lit.name,lit.external_name)
    insert_literal(lit, locctr as u32);
    let literal: Literal = get_literal(lit).unwrap();

    let mut lit_instr =
        Instruction::new(literal.label,
                         "BYTE".to_owned(),
                         UnitOrPair::Unit(AsmOperand::new(OperandType::Bytes,
                                                          Value::Bytes(literal.external_name))));
    lit_instr.locctr = literal.address as i32;
    Box::new(lit_instr)
}

fn has_literal(instr: &Instruction) -> Option<String> {

    for opr in instr.unwrap_operands() {
        let opr: AsmOperand = opr;
        if let Value::Label(lbl) = opr.val {
            if is_literal(&lbl) {
                return Some(lbl);
            }
        }
    }
    None
}

lazy_static!{
    static ref SYMBOL_TABLE: RwLock<HashMap<String,i32>> = RwLock::new(HashMap::new());
}

fn insert_symbol(symbol: &String, address: i32) -> Result<(), String> {

    if exists(symbol) {
        return Err(format!("Label {} is defined at more than one location", symbol));
    }

    SYMBOL_TABLE.write().insert(symbol.clone(), address);
    Ok(())
}

pub fn get_symbol(symbol: &String) -> Option<i32> {
    if exists(symbol) == false {
        None
    } else {
        Some(SYMBOL_TABLE.read().get(symbol).unwrap().clone())
    }
}

pub fn get_all_symbols() -> HashMap<String, i32> {
    SYMBOL_TABLE.read().clone()
}

fn exists(symbol: &String) -> bool {
    return SYMBOL_TABLE.read().contains_key(symbol);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_literal_def() {
        let instr: Instruction = *create_from_literal(&("C'BOX'".to_owned()), 1025);
        println!("{:?}", instr);
        assert_eq!(instr.mnemonic, "BYTE");
        assert_eq!(instr.locctr, 1025);
        assert_eq!(instr.get_first_operand().val,
                   Value::Bytes(("C'BOX'".to_owned())));
    }
}
