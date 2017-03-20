use basic_types::flags::*;
use basic_types::instruction::Instruction;
use basic_types::operands::Operand;
use basic_types::formats::Formats;

pub fn generate_object_code(instruction: &Instruction) -> Result<u32, &str> {
    //let f_vals = instruction.check_invalid_flags();   // TODO Report to RLS

    resolve_instruction_code(instruction, 0).and_then(resolve_operands)

}

fn resolve_instruction_code(instr: &Instruction, code_val: u32) -> Result<u32, &str> {
    // Get the opcode value from the instruction set table
    // Check format correctness
}

fn resolve_operands(instruction: &Instruction, code_val: u32) -> Result<u32, &str> {
    // Based on the given label, match a function
}

fn resolve_label(label: &str) -> Result<u32, &str> {
    // Check the symtab
    // Check the range of addresses with the instruction format
}