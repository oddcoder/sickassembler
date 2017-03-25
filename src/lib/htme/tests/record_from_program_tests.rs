
#[cfg(test)]
mod record_from_program_tests{
    use htme::record_string::*;
    use basic_types::formats::Format;

    #[test]
    fn test_text_record_from_program(){
        let valid_program = vec![(0x1000,0xC4, Format::One), (0x1002, 0xF3, Format::Two), (0x1006, 0x3F4D3, Format::Four)];
        let record = match text_record_from_program(&valid_program){
            Ok(s) => {
                s
            },
            Err((err, i)) => {
                panic!("Error: {:} at code with index {}", err,i);
            }
        };
        assert_eq!(record, String::from("TC400F30003F4D3"));
    }


    #[test]
    #[should_panic]
    fn failed_test_record_from_program() {
        let valid_program = vec![(0x1000, 0xC4, Format::One), (0x1002, 0xF3, Format::None), (0x1006, 0x3F4D3, Format::Four)];
        let record = match text_record_from_program(&valid_program){
            Ok(s) => {
                println!("{:}", s);
                s
            },
            Err((err, i)) => {
                panic!("Error: {:} at code with index {}", err,i);
            }
        };
    }
}
