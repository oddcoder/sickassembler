use std::collections::hash_map::{Entry, HashMap};
use basic_types::instruction::*;
use filehandler::*;
fn get_instruction_size (inst:&Instruction) -> i32 {
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
