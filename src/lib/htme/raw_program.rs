use basic_types::formats::Format;
use htme::record_string::*;

pub struct RawProgram {
    pub program_name: String,
    pub starting_address: u32,
    pub program_length: u32,
    pub program: Vec<(u32, u32, Format)>,
    pub first_instruction_address: u32,
}


impl RawProgram {
    pub fn end_record(& self)->Result<String, &'static str>{
        let record_type = String::from("E");
        let record_width_in_bytes: u8 = 3;
        return match string_from_object_code((*self).first_instruction_address, record_width_in_bytes){
            Ok(record) => Ok(record_type + &record),
            Err(e) => Err(e)
        };
    }


    //TODO:make this a lot less ugly
    pub fn text_records(& self)->Result<String, (&'static str, i32)>{
        //intializing prev_address and prev_format
        let ref program = (*self).program;
        let (first_address, first_code, first_format) = program[0];
        let mut prev_address = first_address;
        let mut prev_format = first_format;
        let mut records = String::from("");

        //number of instructions
        let n = program.capacity();
        //counters
        let mut i = 0;
        let mut begin = 0;
        let mut bytes_left = 30;

        //iterating
        for &(address, code, format) in program.iter(){
            if address - prev_address > (prev_format as u32) || bytes_left < (format as u32){
                let mut vec = vec![];
                vec.extend_from_slice(&program[begin .. i]);
                match text_record_from_program(&vec) {
                    Ok(record) => {
                        if begin != 0{
                            records.push_str("\n");
                        }
                        records.push_str(&record);
                    },
                    Err(t) => return Err(t)
                }
                begin = i;
                bytes_left = 30;
            }
            bytes_left = bytes_left - format as u32;
            i = i+1;
            prev_address = address;
            prev_format = format;

        }

        let mut vec = vec![];
        vec.extend_from_slice(&program[begin .. i]);
        match text_record_from_program(&vec) {
            Ok(record) => {
                records.push_str("\n");
                records.push_str(&record);
            },
            Err(t) => return Err(t)
        }
        return Ok(records);
    }


    pub fn header_record(&self)->Result<String, &'static str>{
        let record_type = String::from("H");
        let mut header = record_type;

        let record_width_in_bytes: u8 = 3;
        match string_from_object_code((*self).starting_address, record_width_in_bytes){
            Ok(record) => header = header+ &record,
            Err(e) => return Err(e)
        }

        match string_from_object_code((*self).program_length, record_width_in_bytes){
            Ok(record) => header = header+ &record,
            Err(e) => return Err(e)
        }
        return Ok(header);
    }
}
