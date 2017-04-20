
#[cfg(test)]
mod raw_program_tests{
    use htme::raw_program::*;
    use htme::record_string::*;
    use basic_types::formats::Format;
    use basic_types::instruction::Instruction;
    use basic_types::operands::Operand;
    use basic_types::unit_or_pair::UnitOrPair;


    #[test]
    fn test_records_from_raw_program(){


        let mut instr: Instruction = Instruction::new(String::new(),
                                                      "load".to_owned(),
                                                      UnitOrPair::Pair(
                                                          Operand::Immediate(Some(5)),
                                                          Operand::Immediate(Some(1))));

        instr.set_format(Format::One);


        let valid_program = vec![
                (0x1000, string_from_object_code(0xC4, 1), instr),
                // (0x1001, string_from_object_code(0xF3, 2), instructions[1]),
                // (0x1007, string_from_object_code(0x3F4D3, 4), instructions[2]),
                // (0x100C, string_from_object_code(0x43, 2), instructions[3]),
                // (0x100E, string_from_object_code(0x43, 2), instructions[4])
            ];

        let raw_program: RawProgram = RawProgram::new(
            /*program_name:*/ String::from("COPY"),
            /*starting_address:*/ 0x1000,
            /*program_length:*/ 0x102A,
            /*program:*/ valid_program,
            /*first_instruction_address:*/ 0x1000
        );

        let end_record = raw_program.end_record();
        assert_eq!(end_record,String::from("E001000"));

        let header_record = raw_program.header_record();
        assert_eq!(header_record,String::from("H00100000102A"));

        let text_records = raw_program.text_records();
        assert_eq!(text_records, "TC4");

        // assert_eq!(text_records, "TC400F3\nT0003F4D3\nT00430043");
    }
}
