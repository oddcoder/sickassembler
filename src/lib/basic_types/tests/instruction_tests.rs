
#[cfg(test)]
mod instuction_tests {

    use basic_types::instruction::Instruction;
    use basic_types::flags::Flags;
    use basic_types::formats::Format;
    use basic_types::operands::Operand;
    use basic_types::unit_or_pair::UnitOrPair;

    #[test]
    #[should_panic]
    fn double_flags() {

        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
                                                      UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));
                                                      
=======
                                                      Operand::Immediate(Some(5)),
                                                      Operand::Immediate(Some(1)));
>>>>>>> Pass 2 preparation
        instr.set_flag(Flags::BaseRelative);
        instr.set_flag(Flags::BaseRelative);
    }

    #[test]
    fn format_3_base_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
                                                      UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));
=======
                                                      Operand::Immediate(Some(5)),
                                                      Operand::Immediate(Some(1)));
>>>>>>> Pass 2 preparation


        instr.set_format(Format::Three);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 12 + 2);
    }


    #[test]
    #[should_panic]
    fn format_3_base_pc_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
                                                     UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));
=======
                                                      Operand::Immediate(Some(5)),
                                                      Operand::Immediate(Some(1)));
>>>>>>> Pass 2 preparation


        // Setting Base and Pc Relative flags is an error
        instr.set_format(Format::Three);
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
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
                                                     UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));
=======
                                                      Operand::Immediate(Some(5)),
                                                      Operand::Immediate(Some(1)));
>>>>>>> Pass 2 preparation

        // Not setting the E flag on a format four instruction is an error
        instr.set_format(Format::Four);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 2);
    }

    #[test]
    #[should_panic]
    fn format_4_base_relative() {
        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
                                               UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));
=======
                                                      Operand::Immediate(Some(5)),
                                                      Operand::Immediate(Some(1)));

>>>>>>> Pass 2 preparation
        // Instruction 4 doesn't use any type of relative addressing
        instr.set_format(Format::Four);
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
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
                                                 UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));
=======
                                                      Operand::Immediate(Some(5)),
                                                      Operand::Immediate(Some(1)));
>>>>>>> Pass 2 preparation


        // Instruction 4 doesn't use any type of relative addressing
        instr.set_format(Format::Four);
        instr.set_flag(Flags::Extended);
        instr.set_flag(Flags::PcRelative);

        // push all along the addresses and the two flags e,p
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 2);
    }

}
