
#[cfg(test)]
mod instuction_tests {

    use basic_types::instruction::Instruction;
    use basic_types::flags::Flags;
    use basic_types::formats::Formats;
    use basic_types::operands::Operand;

    #[test]
    #[should_panic]
    fn print_flags() {

        let mut instr: Instruction = Instruction::new("load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));
        instr.set_flag(Flags::BaseRelative);
        instr.set_flag(Flags::BaseRelative);
    }

    #[test]
    fn base_relative_f3() {
        let mut instr: Instruction = Instruction::new("load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        instr.set_format(Formats::Three);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value(), 1 << 12 + 2);
    }

    #[test]

    fn base_pc_relative_f3() {
        let mut instr: Instruction = Instruction::new("load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        instr.set_format(Formats::Three);
        instr.set_flag(Flags::BaseRelative);
        instr.set_flag(Flags::PcRelative);

        assert_eq!(instr.get_flags_value(), (1 << 12 + 2) + (1 << 12 + 1));
    }

    #[test]
    fn base_relative_f4() {
        let mut instr: Instruction = Instruction::new("load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        instr.set_format(Formats::Four);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value(), 1 << 20 + 2);
    }


}
