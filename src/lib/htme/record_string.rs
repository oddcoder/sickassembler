use basic_types::formats::Format;


/*
 * returns string containing all records from program if all code and format pair make sense.
 * returns error message and its index for an invalid code and format.
 * TODO: format record well
 */

pub fn text_record_from_program(program: &Vec<(i32, Format)>)->Result<String, (&'static str, i32)>{
    let mut i = 0;
    let mut record = String::from("");
    for &(code, format) in program.iter(){
        match string_from_object_code(code,format) {
            Ok(s) => record.push_str(&s),
            Err(s) => return Err((s,i))
        }
        i = i+1;
    }
    return Ok(record);
}


/*
 * returns string from a format-valid object-code.
 * returns error-string if the object-code and format don't add up.
 */
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


/*
 * returns i32 minimum number of hexa digits required to store the i32 code.
 */
pub fn min_hexa_digits_required(code:i32)-> i32 {
    return (code as f64).log(16.0) as i32 + 1;
}
