#[cfg(test)]
mod tests {
    use pass_two::translator::*;
    use instruction::{Instruction, AsmOperand};
    use operands::{OperandType, Value};
    use formats::Format;
    use flags::Flags;
    use unit_or_pair::UnitOrPair;
    use register::Register;
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
            create_instruction("comp",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                                     Value::SignedInt(0))),
                                                                     Format::Three),
                                    
                                                                     create_instruction("TIXR",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Register,
                                                                     Value::Register(Register::T))),
                                                                     Format::Two),
                                                                     create_instruction("LDA",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                                     Value::SignedInt(3))),
                                                                     Format::Three),
                                                                     create_instruction("LDT",
                                    UnitOrPair::Unit(AsmOperand::new(OperandType::Immediate,
                                                                     Value::SignedInt(4096))),
                                                                     Format::Four)
                                    ];

        assert_eq!(translate(&mut instrs[0]).unwrap(), "290000");
        assert_eq!(translate(&mut instrs[1]).unwrap(), "B850");
        assert_eq!(translate(&mut instrs[2]).unwrap(), "010003");
        assert_eq!(translate(&mut instrs[3]).unwrap(), "75101000");
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
