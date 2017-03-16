
#[cfg(test)]
mod instuction_tests {

    use basic_types::instruction::Instruction;
    use basic_types::flags::Flags;
    use basic_types::formats::Formats;
    use basic_types::operands::Operand;

    #[test]
    #[should_panic]
    fn double_flags() {

        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));
        instr.set_flag(Flags::BaseRelative);
        instr.set_flag(Flags::BaseRelative);
    }

    #[test]
    fn format_3_base_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        instr.set_format(Formats::Three);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 12 + 2);
    }


    #[test]
    #[should_panic]
    fn format_3_base_pc_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        // Setting Base and Pc Relative flags is an error
        instr.set_format(Formats::Three);
        instr.set_flag(Flags::BaseRelative);
        instr.set_flag(Flags::PcRelative);

        assert_eq!(instr.get_flags_value().unwrap(),
                   (1 << 12 + 2) + (1 << 12 + 1));
    }

    #[test]
    #[should_panic]
    fn format_4_no_e_flag4() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));

        // Not setting the E flag on a format four instruction is an error
        instr.set_format(Formats::Four);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 2);
    }

    #[test]
    #[should_panic]
    fn format_4_base_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        // Instruction 4 doesn't use any type of relative addressing
        instr.set_format(Formats::Four);
        instr.set_flag(Flags::Extended);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 2);
    }

    #[test]
    #[should_panic]
    fn format_4_pc_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      Operand::Immediate(5),
                                                      Operand::Immediate(1));


        // Instruction 4 doesn't use any type of relative addressing
        instr.set_format(Formats::Four);
        instr.set_flag(Flags::Extended);
        instr.set_flag(Flags::PcRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 2);
    }

}
