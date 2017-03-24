use basic_types::formats::Format;


pub fn string_from_object_code(code: i32, format: Format)-> String {
    let string_width_in_bytes = format as u8;
    return format!("{:01$X}", code, (string_width_in_bytes*2) as usize);
}

pub fn min_hexa_digits_required(code:i32)-> i32 {
    return (code as f64).log(16.0) as i32 + 1;
}
