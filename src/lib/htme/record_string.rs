use basic_types::formats::Format;


pub fn string_from_object_code(code: i32, format: Format)-> Result<String, &'static str> {
    let string_width_in_bytes = format as u8;
    if format == Format::None{
        return Err("argument format equals None. Init your sick instruction well!")
    }
    let hex_digits = (string_width_in_bytes*2) as i32;
    if hex_digits < min_hexa_digits_required(code){
        return Err("object code value is too big for format size. What the heck are you doing?")
    }
    return Ok(format!("{:01$X}", code, hex_digits as usize));
}

pub fn min_hexa_digits_required(code:i32)-> i32 {
    return (code as f64).log(16.0) as i32 + 1;
}
