
#[cfg(test)]
mod record_from_program_tests{
    use htme::record_string::*;
    use basic_types::formats::Format;

    #[test]
    fn test_text_record_from_program(){

        let valid_program = vec![
            (0x1000, string_from_object_code(0xC4, 1), Format::One),
            (0x1002, string_from_object_code(0xF3, 2), Format::Two),
            (0x1006, string_from_object_code(0x3F4D3, 4), Format::Four)
        ];

        let record = text_record_from_program(&valid_program);
        assert_eq!(record, String::from("TC400F30003F4D3"));
    }
}
