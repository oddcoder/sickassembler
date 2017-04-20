use std::collections::hash_map::{Entry, HashMap};
use basic_types::instruction::*;
use basic_types::formats::Format;
use filehandler::*;
use basic_types::unit_or_pair::*;
use basic_types::operands::*;

fn get_instruction_size (inst:&Instruction) -> i32 {
    match inst.format {
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
                panic!("RESB expects only 1 operand");
            }
            match operands[0].val {
                Value::Raw(x) => return 1,
                Value::SignedInt(x) => return 1,
                Value::Label(ref x) => {
                    if &x[0..1] == "X" {
                        return (x.len() as i32 - 3)/2;
                    } else if &x[0..1] == "C"{
                        return x.len() as i32 - 3;
                    }
                }
                _ => panic!("Unexpected Error"),
            }
        },
        "WORD" => return 3,
        "RESB" => {
            let operands = unwrap_to_vec(&inst.operands);
            if operands.len() != 1 {
                panic!("RESB expects only 1 operand");
            }
            match operands[0].val {
                Value::Raw(x) => return x as i32,
                Value::SignedInt(x) => return x,
                _ => panic!("Unexpected Error"),
            }
        },
        "RESW" => {
            let operands = unwrap_to_vec(&inst.operands);
            if operands.len() != 1 {
                panic!("RESW expects only 1 operand");
            }
             match operands[0].val {
                 Value::Raw(x) => return x as i32,
                 Value::SignedInt(x) => return x,
                 _ => panic!("Unexpected Error"),
             }
        },
        _ => (),
    }
    return 0;
}
pub fn pass_one (start: &i32, mut file:FileHandler) ->(HashMap<String, i32>, Vec<Instruction>) {
    let mut loc  = *start;
    let mut symbol_table:HashMap<String, i32> = HashMap::new();
    let mut listing:Vec<Instruction> = Vec::new();
    let mut instruction;
    loop {
        match file.read_instruction() {
            None => break,
            Some(inst) => instruction = inst,
        }
        instruction.locctr = loc;
        if !instruction.label.is_empty() {
            if symbol_table.contains_key(&instruction.label) {
                panic!("Label {} is defined at more than one location", instruction.label);
            };
            symbol_table.insert(instruction.label.clone(), loc);
        }
        loc += get_instruction_size(&instruction);
        listing.push(instruction);
    }
    return (symbol_table, listing);
}
