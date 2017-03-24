
#[cfg(test)]
mod instuction_tests {

    use basic_types::instruction::Instruction;
    use basic_types::flags::Flags;
    use basic_types::formats::Format;
    use basic_types::operands::*;
    use basic_types::unit_or_pair::UnitOrPair;

    #[test]
    #[should_panic]
    fn double_flags() {

        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        // Repeated flags, should panic
        instr.set_flag(Flags::BaseRelative).expect("flags");
        instr.set_flag(Flags::BaseRelative).expect("flags");
    }

    #[test]
    fn format_3_base_relative() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());


        instr.set_format(Format::Three);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 12-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 12 + 2);
    }


    #[test]
    #[should_panic]
    fn format_3_base_pc_relative() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        // Setting Base and Pc Relative flags is an error
        instr.set_format(Format::Three);
        instr.set_flag(Flags::BaseRelative);
        instr.set_flag(Flags::PcRelative);

        // n i x b p e | 12-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(),
                   (1 << 12 + 2) + (1 << 12 + 3));
    }

    #[test]
    #[should_panic]
    fn format_4_no_e_flag4() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        // Not setting the E flag on a format four instruction is an error
        instr.set_format(Format::Four);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 20-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 2);
    }

    #[test]
    #[should_panic]
    fn format_4_base_relative() {
        let mut instr: Instruction = Instruction::new_simple("format 4 rel - B".to_owned());

        // Instruction 4 doesn't use any type of relative addressing
        // that's why this should panic
        instr.set_format(Format::Four);
        instr.set_flag(Flags::Extended);
        instr.set_flag(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 20-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(),
                   (1 << 20 + 1) + (1 << 20 + 3));
    }

    #[test]
    #[should_panic]
    fn format_4_pc_relative() {
        let mut instr: Instruction = Instruction::new_simple("format 4 rel - PC".to_owned());

        // Instruction 4 doesn't use any type of relative addressing
        instr.set_format(Format::Four);
        instr.set_flag(Flags::Extended);
        instr.set_flag(Flags::PcRelative);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 20-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(),
                   (1 << 20 + 1) + (1 << 20 + 2));
    }

}
