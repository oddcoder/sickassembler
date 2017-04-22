use std::collections::HashMap;
use instruction::*;
use formats::Format;
use filehandler::*;
use unit_or_pair::*;
use parking_lot::RwLock;
use operands::*;
use literal::Literal;
use literal_table::{insert_literal, get_unresolved, get_literal};
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
            match operands[0].val {
                Value::Raw(_) => return 1,
                Value::SignedInt(_) => return 1,
                Value::Bytes(ref x) => {
                    if &x[0..1] == "X" {
                        let len: f32 = ((x.len() as i32 - 3) / 2) as f32;
                        return len.ceil() as i32;
                    } else if &x[0..1] == "C" {
                        return x.len() as i32 - 3;
                    } else if &x[0..1] == "=" {
                        return get_literal(x).unwrap().length_in_bytes();
                    }
                }
                _ => panic!("Unexpected Error {:?}", *inst),
            }
        }
        "WORD" => return 3,
        "RESB" => {
            let operands = unwrap_to_vec(&inst.operands);
            if operands.len() != 1 {
                panic!("RESB expects only 1 operand");
            }
            match operands[0].val {
                Value::Raw(x) => return x as i32,
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
                // FIXME: parser mistake, this should take a SingedInt i.e. immediate value
                Value::Raw(x) => return (x * 3) as i32,
                Value::SignedInt(x) => return x * 3,
                _ => panic!("Unexpected Error"),
            }
        }
        _ => (),
    }
    return 0;
}

pub fn pass_one(mut file: FileHandler) -> (HashMap<String, i32>, RawProgram) {

    // TODO: replace the literal in an instruction operand with the literal label
    // if let Value::Bytes(ref x) = instruction.get_first_operand().val {}

    let (mut prog, loc) = file.parse_file().unwrap();
    let mut loc = loc as i32;

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
            if let Err(_) = insert_symbol(&instruction.label, loc) {
                panic!("Label {} is defined at more than one location",
                       instruction.label);
            }
        }

        if instruction.mnemonic.to_uppercase() == "LTORG" {
            loc = flush_literals(&mut instructions, loc as u32);

            // Base resolution can only be in pass2 -> symbol table incomplete & fwd referencing
        } else {
            loc += get_instruction_size(&instruction);
            instructions.push(instruction);
        }
    }

    // Flush remaining literals
    flush_literals(&mut instructions, loc as u32);


    // Move the instructions back
    prog.program = instructions.into_iter()
        .map(|i| (String::new(), i))
        .collect::<Vec<(_, Instruction)>>();

    return (get_all_symbols(), prog);
}

fn flush_literals(instructions: &mut Vec<Instruction>, start_loc: u32) -> i32 {
    // TODO: fix literals
    // TODO: insert in instruction vector
    let mut loc = start_loc;
    for lit in get_unresolved() {
        insert_literal(&lit, loc);
        // literal declaration to be inserted in code
        let lit_decl = *create_from_literal(&lit, loc as i32);
        let lit_sz = get_instruction_size(&lit_decl) as u32;
        loc += lit_sz;
        instructions.push(lit_decl);
    }
    loc as i32
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
