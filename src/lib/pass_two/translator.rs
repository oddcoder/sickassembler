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

#[cfg(test)]
mod tests {
    use super::*;
    use basic_types::formats;
    use basic_types::operands::Value;
    use basic_types::register::Register;

    #[test]
    fn test_resolve_op_code() {
        let mut inst = Instruction::new_simple("add".to_owned());
        inst.format = formats::Format::Four;
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x18);

        inst.format = formats::Format::Three;
        assert!(resolve_opcode(&inst).unwrap() == 0x18);
    }

    #[test]
    fn add_format_one() {
        let mut inst = Instruction::new_simple("add".to_owned());
        assert_eq!(resolve_opcode(&inst).unwrap(), 0x18);
    }

    #[test]
    fn test_resolve_regs() {
        let mut inst = Instruction::new_simple("add".to_owned());
        inst.add_operand(OperandType::Register, Value::Register(Register::A));
        let oprs: Vec<u32> = resolve_incomplete_operands(&inst).unwrap();

        assert_eq!(oprs.len(), 1);
        assert_eq!(oprs[0], 0);
    }

fn resolve_label(label: &str) -> Result<u32, &str> {
    // Check the symtab
    // Check the range of addresses with the instruction format
    unimplemented!();
}
