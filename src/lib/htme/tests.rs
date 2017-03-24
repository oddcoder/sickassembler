
#[cfg(test)]
mod formatting_tests{
    use htme::record_string::*;
    use basic_types::formats::Format;

    #[test]
    fn test_string_from_code() {
        let code: i32 = 196;
        let string_code = match string_from_object_code(code, Format::One){
            Ok(s) =>  s,
            Err(e) => {
                println!("Error: {:}", e);
                "".to_string()
            }
        };
        assert_eq!("C4".to_string(), string_code);

        // let code: i32 = 243;
        // let string_code = string_from_object_code(code, Format::Two);
        // assert_eq!("00F3".to_string(), string_code);
        //
        // let code: i32 = 259283;
        // let string_code = string_from_object_code(code, Format::Four);
        // assert_eq!("0003F4D3".to_string(), string_code);

    }

    #[test]
    fn test_hex_digits_required(){
        let digits = min_hexa_digits_required(196);
        assert_eq!(digits, 2);

        let digits = min_hexa_digits_required(259283);
        assert_eq!(digits, 5);
    }
}
