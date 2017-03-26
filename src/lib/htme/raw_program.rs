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

    pub fn text_record(& self)->Result<String, &'static str>{
        unimplemented!()
    }


    pub fn header_record() {
        unimplemented!()
    }
}
