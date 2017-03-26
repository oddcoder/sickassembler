
#[cfg(test)]
mod raw_program_tests{
    use htme::raw_program::*;
    use basic_types::formats::Format;

    #[test]
    fn test_records_from_raw_program(){
        let valid_program = vec![(0x1000,0xC4, Format::One), (0x1002, 0xF3, Format::Two), (0x1006, 0x3F4D3, Format::Four)];

        let raw_program: RawProgram = RawProgram{
            program_name: String::from("COPY"),
            starting_address: 0x1000,
            program_length: 0x102A,
            program: valid_program,
            first_instruction_address: 0x1000
        };

        let end_record = match raw_program.end_record(){
            Ok(s) => s,
            Err(e) => panic!("Error: {}", e)
        };
        assert_eq!(end_record,String::from("E001000"));

        let header_record = match raw_program.header_record(){
            Ok(s) => s,
            Err(e) => panic!("Error: {}", e)
        };
        assert_eq!(header_record,String::from("H00100000102A"));
    }
}
