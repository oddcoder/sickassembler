
#[cfg(test)]
mod raw_program_tests {
    use htme::raw_program::*;
    use htme::record_string::*;
    use formats::Format;
    use instruction::*;
    use operands::*;
    use unit_or_pair::UnitOrPair;


    #[test]
    fn test_records_from_raw_program() {

        let mut instr1 = Instruction::new_simple("some mnemonic".to_string());
        let mut instr2 = instr1.clone();
        let operand = AsmOperand::new(OperandType::Label, Value::Label("el_label".to_string()));

        let mut instr3 = Instruction::new(String::new(),
                                          "some mnemonic".to_string(),
                                          UnitOrPair::Unit(operand));

        let mut instr4 = instr1.clone();
        let mut instr5 = instr1.clone();
        instr1.set_format(Format::One);
        instr1.locctr = 0x1000;

        instr2.set_format(Format::Two);
        instr2.locctr = 0x1001;

        instr3.set_format(Format::Four);
        instr3.locctr = 0x1007;

        instr4.set_format(Format::Two);
        instr4.locctr = 0x100C;

        instr5.set_format(Format::Two);
        instr5.locctr = 0x100E;

        let valid_program = vec![(string_from_object_code(0xC4, 1), instr1),
                                 (string_from_object_code(0xF3, 2), instr2),
                                 (string_from_object_code(0x3F4D3, 4), instr3),
                                 (string_from_object_code(0x43, 2), instr4),
                                 (string_from_object_code(0x43, 2), instr5)];

        let raw_program: RawProgram = RawProgram {
            program_name: String::from("COPY.htme"),
            starting_address: 0x1000,
            program_length: 0x102A,
            program: valid_program,
            first_instruction_address: 0x1000,
        };

        let end_record = raw_program.end_record();
        assert_eq!(end_record, String::from("E001000"));

        let header_record = raw_program.header_record();
        assert_eq!(header_record, String::from("H00100000102A"));

        let text_records = raw_program.text_records();
        assert_eq!(text_records, "TC400F3\nT0003F4D3\nT00430043");

        let modification_records = raw_program.modification_records();
        assert_eq!(modification_records, "M00100805\n");

        let all_records = raw_program.all_records();
        assert_eq!(all_records,
                   "H00100000102A\nTC400F3\nT0003F4D3\nT00430043\nM00100805\nE001000");

        raw_program.output_to_file();
    }
}
