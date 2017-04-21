
#[cfg(test)]
mod record_from_program_tests {
    use htme::record_string::*;
    use basic_types::formats::Format;
    use basic_types::instruction::Instruction;

    #[test]
    fn test_text_record_from_program() {

        let mut instr1 = Instruction::new_simple("some mnemonic".to_string());
        let mut instr2 = instr1.clone();
        let mut instr3 = instr1.clone();
        instr1.set_format(Format::One);
        instr1.locctr = 0x1000;
        instr2.set_format(Format::Two);
        instr2.locctr = 0x1002;
        instr3.set_format(Format::Four);
        instr3.locctr = 0x1006;
        let valid_program = vec![(string_from_object_code(0xC4, 1), instr1),
                                 (string_from_object_code(0xF3, 2), instr2),
                                 (string_from_object_code(0x3F4D3, 4), instr3)];

        let record = text_record_from_program(&valid_program);
        assert_eq!(record, String::from("TC400F30003F4D3"));
    }
}
