use pass_two::translator::*;
use basic_types::instruction::{Instruction, AsmOperand};
use basic_types::operands::{OperandType, Value};
use basic_types::formats::Format;
use basic_types::flags::Flags;
use basic_types::unit_or_pair::UnitOrPair;

#[test]
fn flag_resolution() {
    //pass_two::object_code_generator::object_code_gen::generate_object_code

    let mut instr: Instruction = Instruction::new_simple("load".to_owned());

    instr.add_operand(OperandType::Immediate, Value::SignedInt(5));

    instr.set_format(Format::Four);
    instr.set_flag(Flags::Immediate);
    instr.set_flag(Flags::Extended);

    // n i x b p e | 20-addr - 0 - indexed
    assert_eq!(instr.get_flags_value().unwrap(), (1 << 20 + 4) + (1 << 20));
}
