use basic_types::flags::*;
use basic_types::instruction::Instruction;
use basic_types::operands::Operand;
use basic_types::formats::Format;

pub fn translate(instruction: &Instruction) -> Result<u32, &str> {
    //let f_vals = instruction.check_invalid_flags();   // TODO Report to RLS
    //resolve_instruction_code(instruction, 0).and_then(resolve_operands)
    unimplemented!();
}

fn resolve_incomplete_operands(instruction: &Instruction) {
    // Convert immediate and indirect operands to a basic forms -> Raw

    // OpCode retrival is done after all the operands related to pass1 have been
    // resolved, ( indirect, immediate, label ) -> Raw as this will have
    // direct mapping to the instruction set
    unimplemented!();
}

fn resolve_opcode(instr: &Instruction, code_val: u32) -> Result<u32, &str> {
    // Get the opcode value from the instruction set table
    // Check format correctness
    unimplemented!();
}

fn resolve_operands(instruction: &Instruction, code_val: u32) -> Result<u32, &str> {
    // Based on the given label, match a function
    unimplemented!();
}

fn resolve_label(label: &str) -> Result<u32, &str> {
    // Check the symtab
    // Check the range of addresses with the instruction format
    unimplemented!();
}
