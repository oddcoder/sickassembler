
#[cfg(test)]
mod formatting_tests{
    use htme::record_string::*;
    use basic_types::formats::Format;

    #[test]
    fn test_string_from_code() {
        let code: u32 = 0xC4;
        let string_code = match string_from_object_code(code, Format::One as u8){
            Ok(s) =>  s,
            Err(e) => {
                panic!("Error: {:}", e);
                "".to_string()
            }
        };
        assert_eq!("C4".to_string(), string_code);


        let code: u32 = 0x3F4D3;
        let string_code = match string_from_object_code(code, Format::Four as u8){
            Ok(s) =>  s,
            Err(e) => {
                println!("Error: {:}", e);
                "".to_string()
            }
        };
        assert_eq!("0003F4D3".to_string(), string_code);
    }

    #[test]
    #[should_panic]
    fn failed_test_string_from_code(){

        // //should panic: format = None
        // let code: u32 = 0xC4;
        // let string_code = match string_from_object_code(code, Format::None as u8){
        //     Ok(s) =>  s,
        //     Err(e) => {
        //         panic!("Error: {:}", e);
        //     }
        // };

        //should panic: size of format < size of code
        let code: u32 = 0x3F4D3;
        let string_code = match string_from_object_code(code, Format::One as u8){
            Ok(s) =>  s,
            Err(e) => {
                panic!("Error: {:}", e);
            }
        };

    }

    #[test]
    fn test_hex_digits_required(){
        let digits = min_hexa_digits_required(0x34);
        assert_eq!(digits, 2 as u32);

        let digits = min_hexa_digits_required(0x532FF);
        assert_eq!(digits, 5 as u32);
    }

}
