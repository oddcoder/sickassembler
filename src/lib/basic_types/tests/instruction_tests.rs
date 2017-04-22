
#[cfg(test)]
mod instuction_tests {

    use instruction::Instruction;

    use formats::Format;

    #[test]
    fn format_3_base_relative() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        instr.set_format(Format::Three);
        instr.set_base_relative();

        // push all along the addresses and the two flags e,p
        // n i x b p e | 12-addr - 0-indexed
        // 1 1 0 1 0 0
        assert_eq!(instr.get_flags_value().unwrap(),
                   (1 << (12 + 2)) + (1 << (12 + 4)) + (1 << (12 + 5)));
    }

    #[test]
    fn format_4() {
        let mut instr: Instruction = Instruction::new_simple("load".to_owned());

        // Not setting the E flag on a format four instruction is an error
        instr.set_format(Format::Four);

        // push all along the addresses and the two flags e,p
        // n i x b p e | 20-addr - 0-indexed
        assert_eq!(instr.get_flags_value().unwrap(),
                   (1 << 20) + (1 << (20 + 4)) + (1 << (20 + 5)));
    }

    #[test]
    fn format_3_pc_relative() {
        let mut instr: Instruction = Instruction::new_simple("format 4 rel - PC".to_owned());

        instr.set_pc_relative();
        match instr.get_flags_value() {
            Err(_) => (), // Pass
            _ => panic!("Format isn't set and the function returned a correct result"),
        }
    }

}
