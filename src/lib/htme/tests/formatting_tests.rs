
#[cfg(test)]
mod formatting_tests {
    use htme::record_string::*;
    use formats::Format;

    #[test]
    fn test_string_from_code() {
        let code: u32 = 0xC4;
        let string_code = string_from_object_code(code, Format::One as u8);
        assert_eq!("C4".to_string(), string_code);


        let code: u32 = 0x3F4D3;
        let string_code = string_from_object_code(code, Format::Four as u8);
        assert_eq!("0003F4D3".to_string(), string_code);
    }


    #[test]
    fn test_hex_digits_required() {
        let digits = min_hexa_digits_required(0x34);
        assert_eq!(digits, 2 as u32);

        let digits = min_hexa_digits_required(0x532FF);
        assert_eq!(digits, 5 as u32);
    }

}
