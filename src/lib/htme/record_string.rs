use basic_types::formats::Format;

//TODO: decide if I need to move these?


/*
 * returns string containing all records from program if all code and format pair make sense.
 * returns error message and its index for an invalid code and format.
 */
pub fn text_record_from_program(program: &Vec<(u32, u32, Format)>)->String{

    //counter to know where error happened
    let mut i = 0;
    let record_type = String::from("T");
    //empty record we keep appending to
    let mut record = String::from("");

    //iterate on program (code-format tuple)
    for &(address, code, format) in program.iter(){
        //push string onto record
        let s = string_from_object_code(code,format as u8);
        record.push_str(&s);

        //next
        i = i+1;
    }

    //if we're here, everything went well and record is returned
    return record_type + &record;
}



/*
 * returns string from a format-valid object-code.
 * returns error-string if the object-code and format don't add up.
 */
pub fn string_from_object_code(code: u32, string_width_in_bytes: u8)-> String{

    let hex_digits = (string_width_in_bytes*2) as u32;

    //return string with right amount of zeros to the left.
    return format!("{:01$X}", code, hex_digits as usize);
}


//TODO: are we gonna use this somewhere?
/*
 * returns u32 minimum number of hexa digits required to store the u32 code.
 */
pub fn min_hexa_digits_required(code:u32)-> u32 {
    return (code as f64).log(16.0) as u32 + 1;
}
