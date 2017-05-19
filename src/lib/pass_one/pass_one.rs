use std::collections::HashSet;
use instruction::*;
use formats::Format;
use unit_or_pair::*;
use operands::*;
use literal::Literal;
use literal_table::{insert_literal, get_unresolved, get_literal};
use std::u32;
use symbol::{Symbol, SymbolType};
use super::super::*;
use basic_types::symbol_tables::define_local_symbol;

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

pub fn pass_one(prog_info: RawProgram) -> Result<(HashSet<Symbol>, RawProgram), String> {

    // TODO: replace the literal in an instruction operand with the literal label
    // if let Value::Bytes(ref x) = instruction.get_first_operand().val {}

    let prog = prog_info;
    let mut prog: RawProgram = prog;
    let temp_instructions: Vec<Instruction>;

    // Move the instructions into a temp storage
    // to allow for adding literals
    temp_instructions = prog.program
        .into_iter()
        .map(move |t: (_, Instruction)| t.1)
        .collect::<Vec<Instruction>>();

    prog.program = Vec::new(); // To cancel the move effect

    // TODO: return here
    let (errs, instructions) = process_instructions(temp_instructions, &mut prog);
    // Move the instructions back
    prog.program = instructions.into_iter()
        .map(|i| (String::new(), i))
        .collect::<Vec<(_, Instruction)>>();

    if errs.len() != 0 {
        return Err(errs.join("\n "));
    }

    Ok((get_all_symbols(), prog))
}

fn flush_literals(instructions: &mut Vec<Instruction>, start_loc: u32, current_csect: &str) -> i32 {

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
        define_local_symbol(&lit, lit_addr, current_csect).unwrap();
    }
    loc as i32
}

fn process_instructions(temp_instructions: Vec<Instruction>,
                        mut prog: &mut RawProgram)
                        -> (Vec<String>, Vec<Instruction>) {
    let mut loc = 0;
    let mut errs: Vec<String> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut current_csect: String = String::new();


    match parse_start(&temp_instructions[0], &mut prog) {
        Err(e) => errs.push(e),
        Ok(start) => loc = start as i32,
    }

    // Skip the first instruction
    let temp_instructions = temp_instructions.into_iter().skip(1);
    for instruction in temp_instructions {
        let mut instruction: Instruction = instruction;
        let instruction_size: i32 = get_instruction_size(&instruction);

        instruction.locctr = loc;

        match instruction.mnemonic.to_uppercase().as_str() {
            "START" => errs.push("Duplicate START instruction".to_owned()),
            "LTORG" => {
                loc = flush_literals(&mut instructions, loc as u32, &current_csect);
            }
            "END" => {
                match parse_end(&instruction, &mut prog, loc + instruction_size) {
                    Ok(_) => instructions.push(instruction.clone()),
                    Err(e) => errs.push(e),
                }
            }
            _ => {
                loc = consume_instruction(&instruction,
                                          loc,
                                          &mut current_csect,
                                          instruction_size,
                                          &mut errs,
                                          &mut instructions)
            }
        };
    }

    // Flush remaining literals
    flush_literals(&mut instructions, loc as u32, &current_csect);

    if prog.program_length == u32::MAX {
        errs.push(format!("Couldn't find the END instruction"));
    }
    (errs, instructions)
}

fn consume_instruction(instruction: &Instruction,
                       mut loc: i32,
                       mut csect: &mut String,
                       instruction_size: i32,
                       errs: &mut Vec<String>,
                       instructions: &mut Vec<Instruction>)
                       -> i32 {
    // This function exists just to improve testability

    if !instruction.label.is_empty() && instruction.mnemonic != "CSECT" {
        if let Err(e) = define_local_symbol(&instruction.label, loc, &csect) {
            errs.push(format!("{}", e));
        }
    }

    match instruction.mnemonic.to_uppercase().as_str() {
        "EXTREF" => {
            // TODO: do
        }
        "EXTDEF" => {
            // TODO: do
        }
        "CSECT" => {
            // TODO: add csect to master table
            *csect = instruction.label.clone();
            loc = 0;
        }
        _ => {
            loc += instruction_size;
            instructions.push(instruction.clone());
        }
    }

    loc
}

/// Gets the address of the first executable isntruction
fn parse_end(instruction: &Instruction,
             prog: &mut RawProgram,
             end_instr_addr: i32)
             -> Result<(), String> {
    // TODO: change read_start to read boundary START/END
    // or replace with is action directive
    // FIXME: check for instructions after END
    let operands = unwrap_to_vec(&instruction.operands);
    let end_loc: i32;
    if operands.len() != 0 {
        if let Value::Raw(op_end) = operands[0].val {
            // Will panic on negative value
            end_loc = op_end as i32;
        } else if let Value::Label(ref lbl) = operands[0].val {
            match get_symbol_for_end(&lbl) {
                Ok(addr) => end_loc = addr,
                Err(e) => return Err(e),
            }
        } else {
            return Err(format!("Invalid END operands, found {:?}", operands));
        }
    } else {
        // End operand isn't specified, default: program start address
        end_loc = prog.first_instruction_address as i32;
    }

    prog.starting_address = end_loc as u32;
    prog.program_length = (end_instr_addr - end_loc) as u32;
    Ok(())
}


fn parse_start(instruction: &Instruction, prog: &mut RawProgram) -> Result<i32, String> {

    // Duplicate start instruction
    if instruction.mnemonic.to_uppercase() != "START" {
        return Err(format!("Program must have its first instrution as START"));
    } else if !prog.program_name.is_empty() {
        return Err(format!("Invalid START instruction {:?} , old prog name: {}",
                           instruction,
                           prog.program_name));
    } else if instruction.label.is_empty() {
        return Err(format!("Program doesn't have a name specified in START"));
    }

    let start_addr: u32;
    if let Value::Raw(adr) = instruction.get_first_operand().val {
        start_addr = adr as u32;
    } else {
        start_addr = 0;
    }

    prog.program_name = instruction.label.clone();
    prog.first_instruction_address = start_addr;

    // Program name goes to sym_tab
    if let Err(e) = define_local_symbol(&instruction.label, start_addr as i32, &String::new()) {
        // Add prog name to symtab
        return Err(format!("{}", e));
    }

    Ok(start_addr as i32)
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

pub fn get_symbol_for_end(symbol: &str) -> Result<i32, String> {
    // Used with the END instruction only
    match symbol_tables::get_symbol(symbol, &String::new()) {
        Ok(sym) => {
            if sym.symbol_type == SymbolType::Imported {
                return Err(format!("Can't use an Imported symbol with END"));
            }
            Ok(sym.symbol.get_address())
        }
        Err(_) => Err(format!("Couldn't find symbol {{ {} }} for END instruction", symbol)),
    }

}

pub fn get_all_symbols() -> HashSet<Symbol> {
    let mut result: HashSet<Symbol> = HashSet::new();
    for val in symbol_tables::get_all_symbols() {
        result.insert(val.clone());
    }

    result
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
