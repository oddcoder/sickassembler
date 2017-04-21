use std::collections::hash_map::{Entry, HashMap};
use basic_types::instruction::*;
use basic_types::formats::Format;
use filehandler::*;
use basic_types::unit_or_pair::*;
use parking_lot::RwLock;
use basic_types::operands::*;
use super::super::RawProgram;

fn get_instruction_size(inst: &Instruction) -> i32 {
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
                        return (x.len() as i32 - 3) / 2;
                    } else if &x[0..1] == "C" {
                        return x.len() as i32 - 3;
                    }
                }
                _ => panic!("Unexpected Error"),
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
                _ => panic!("Unexpected Error"),
            }
        }
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
        }
        _ => (),
    }
    return 0;
}

pub fn pass_one(mut file: FileHandler) -> (HashMap<String, i32>, RawProgram) {

    let mut symbol_table: HashMap<String, i32> = HashMap::new();

    let prog_info = file.parse_file().unwrap();
    let mut prog = prog_info.0;
    let mut loc = prog_info.1 as i32;

    for &mut (_, ref mut instruction) in prog.program.iter_mut() {
        instruction.locctr = loc;
        if !instruction.label.is_empty() {

            if symbol_table.contains_key(&instruction.label) {
                panic!("Label {} is defined at more than one location",
                       instruction.label);
            };

            insert_symbol(&instruction.label, loc);
        }

        loc += get_instruction_size(&instruction);
    }

    return (get_all_symbols(), prog);
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
