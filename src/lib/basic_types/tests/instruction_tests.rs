
#[cfg(test)]
mod instuction_tests {

    use basic_types::instruction::Instruction;
    use basic_types::flags::Flags;
    use basic_types::formats::Format;

    #[test]
    fn format_3_base_relative() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        instr.set_format(Format::Three);
        instr.set_addressing_mode(Flags::BaseRelative);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 12-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 12 + 2);
    }

    #[test]
    fn format_4() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        // Not setting the E flag on a format four instruction is an error
        instr.set_format(Format::Four);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 20-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(), 1 << 20 + 0);
    }

    #[test]
    fn format_3_pc_relative() {
        let mut instr: Instruction = Instruction::new_simple("format 4 rel - PC".to_owned());

        instr.set_addressing_mode(Flags::PcRelative);
        match instr.get_flags_value() {
            Err(_) => (), // Pass
            _ => panic!("Format isn't set and the function returned a correct result"),
        }
    }

}
