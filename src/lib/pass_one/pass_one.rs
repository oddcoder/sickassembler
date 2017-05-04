use std::collections::HashMap;
use instruction::*;
use formats::Format;
use unit_or_pair::*;
use parking_lot::RwLock;
use operands::*;
use literal::Literal;
use literal_table::{insert_literal, get_unresolved, get_literal};
use std::u32;
use super::super::*;

// FIXME: get instruction size shouldn't check for errors
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

pub fn pass_one(prog_info: RawProgram) -> Result<(HashMap<String, i32>, RawProgram), String> {

    // TODO: replace the literal in an instruction operand with the literal label
    // if let Value::Bytes(ref x) = instruction.get_first_operand().val {}

    // FIXME: read the start instruction to set the locctr
    let mut errs: Vec<String> = Vec::new();
    let prog = prog_info;
    let mut loc = 0;
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

    for instruction in temp_instructions {
        let mut instruction: Instruction = instruction;
        instruction.locctr = loc;

        // Comes after label to add the program name to the labels
        if instruction.mnemonic.to_uppercase() == "START" {
            // Duplicate start instruction
            if !prog.program_name.is_empty() {
                errs.push(format!("Invalid START instruction {:?} , old prog name: {}",
                                  instruction,
                                  prog.program_name));
            }

            match parse_start(&instruction) {
                Err(e) => errs.push(e),
                Ok((name, start)) => {
                    prog.program_name = name;
                    prog.first_instruction_address = start;
                    loc = start as i32
                }
            };
        }

        if !instruction.label.is_empty() {
            if let Err(e) = insert_symbol(&instruction.label, loc) {
                errs.push(format!("{}", e));
            }
        }

        if instruction.mnemonic.to_uppercase() == "LTORG" {
            loc = flush_literals(&mut instructions, loc as u32);
        } else {
            loc += get_instruction_size(&instruction);
            instructions.push(instruction.clone());
        }

        // This must come after the location increment to calculate the correct
        // length of the program and not skip the last instruction
        if instruction.mnemonic.to_uppercase() == "END" {
            match parse_end(&instruction) {
                Ok(end) => {
                    prog.first_instruction_address = end;
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
        insert_symbol(&lit, lit_addr).unwrap();

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
            Ok(op_end)
        } else if let Value::Label(ref lbl) = operands[0].val {
            match get_symbol(&lbl) {
                Ok(addr) => Ok(addr as u32),
                Err(e) => Err(e),
            }
        } else {
            Err(format!("Invalid END operands, found {:?}", operands))
        }
    } else {
        // End operand isn't specified, default: program start address
        Ok((instruction.locctr) as u32)
    }

}


fn parse_start(instruction: &Instruction) -> Result<(String, u32), String> {

    let start_addr: u32;
    let prog_name: String;
    if let Value::Raw(adr) = instruction.get_first_operand().val {
        start_addr = adr as u32;
    } else {
        start_addr = 0;
    }
    prog_name = instruction.label.clone();

    Ok((prog_name, start_addr))
}

fn create_from_literal(lit: &String, locctr: i32) -> Box<Instruction> {

    insert_literal(lit, locctr as u32);
    let literal: Literal = get_literal(lit).unwrap();

    // Ad the literal definition, as normal byte/word
    let operand = AsmOperand::new(OperandType::Bytes,
                                  Value::Bytes(literal.external_name[1..].to_owned()));

    let mut lit_instr =
        Instruction::new(literal.label, "BYTE".to_owned(), UnitOrPair::Unit(operand));

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

pub fn get_symbol(symbol: &str) -> Result<i32, String> {
    if exists(symbol) {
        Ok(SYMBOL_TABLE.read().get(symbol).unwrap().to_owned())
    } else {
        Err(format!("Couldn't find symbol {{ {} }}", symbol))
    }
}

pub fn get_all_symbols() -> HashMap<String, i32> {
    SYMBOL_TABLE.read().clone()
}

fn exists(symbol: &str) -> bool {
    return SYMBOL_TABLE.read().contains_key(symbol);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_literal_def() {
        let instr: Instruction = *create_from_literal(&("=C'BOX'".to_owned()), 1025);
        println!("{:?}", instr);
        assert_eq!(instr.mnemonic, "BYTE");
        assert_eq!(instr.locctr, 1025);
        assert_eq!(instr.get_first_operand().val,
                   Value::Bytes(("C'BOX'".to_owned())));
    }
}
