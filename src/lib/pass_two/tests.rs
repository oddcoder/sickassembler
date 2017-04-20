#[cfg(test)]
mod tests {
    use pass_two::translator::*;
    use basic_types::instruction::{Instruction, AsmOperand};
    use basic_types::operands::{OperandType, Value};
    use basic_types::formats::Format;
    use basic_types::flags::Flags;
    use basic_types::unit_or_pair::UnitOrPair;
    use basic_types::register::Register;
    #[test]
    fn flag_resolution() {
        let mut instr: Instruction =
            Instruction::new(String::new(),
                             "load".to_owned(),
                             UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                              Value::SignedInt(5))));

        // Format should be set first before adding any operands
        instr.set_format(Format::Four);
        println!("{:?}", instr);
        // n i x b p e | 20-addr - 0 - indexed
        assert_eq!(instr.get_flags_value().unwrap(), (1 << 20 + 4) + (1 << 20));
    }

    #[test]
    fn translate_correct() {
        let mut instrs = vec![
            //create_instruction("comp",
                                    // UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                    //                                  Value::SignedInt(0))),
                                    //                                  Format::Three),
                                    
                                                                     create_instruction("TIXR",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Register,
                                                                     Value::Register(Register::T))),
                                                                     Format::Two)
                                    ];

        // assert_eq!(translate(&mut instrs[0]).unwrap(), "290000");
        assert_eq!(translate(&mut instrs[0]).unwrap(), "B850");
    }

    fn create_instruction(mnemonic: &str,
                          operands: UnitOrPair<AsmOperand>,
                          format: Format)
                          -> Instruction {

        let mut instr = Instruction::new(String::new(), mnemonic.to_owned(), operands);
        instr.set_format(format);
        instr
    }
}