use basic_types::formats::Format;
use basic_types::instruction::Instruction;
use htme::record_string::*;

pub struct RawProgram {
    pub program_name: String,
    pub starting_address: u32,
    pub program_length: u32,
    pub program: Vec<(u32, String, Format)>,
    pub first_instruction_address: u32,
}


impl RawProgram {
    pub fn new(program_name: String, starting_address: u32, program_length:u32, program: Vec<(u32, String, Instruction)>, first_instruction_address: u32)->RawProgram{

        let mut vec = vec![];
        for &(address, ref code,  ref instruction) in program.iter(){
            vec.push((address, code.to_owned(), (*instruction).format));
        }

        RawProgram{
            program_name: program_name,
            starting_address: starting_address,
            program_length: program_length,
            program: vec,
            first_instruction_address: first_instruction_address
        }

    }

    pub fn end_record(& self)->String{

        //initing record
        let record = String::from("E");

        //getting hexacode s from 1st instruction address with right amount of zeros.
        let record_width_in_bytes: u8 = 3;
        let s = string_from_object_code((*self).first_instruction_address, record_width_in_bytes);

        //returning "E"+Some Hexa
        return record + &s;
    }


    //TODO:make this a lot less ugly
    pub fn text_records(& self)->String{
        //intializing prev_address and prev_format
        let ref program = (*self).program;
        let (first_address, ref first_code, first_format) = program[0];
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
        for &(address, ref code, format) in program.iter(){
            if address - prev_address > (prev_format as u32) || bytes_left < (format as u32){
                let mut vec = vec![];
                vec.extend_from_slice(&program[begin .. i]);
                let record = text_record_from_program(&vec);
                if begin != 0{
                    records.push_str("\n");
                }
                records.push_str(&record);
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
        let record = text_record_from_program(&vec);
        if begin != 0{
            records.push_str("\n");
        }
        records.push_str(&record);

        return records;
    }


    pub fn header_record(&self)->String{
        let record_type = String::from("H");
        let mut header = record_type;

        //6 columns
        let record_width_in_bytes: u8 = 3;

        //appending starting address
        let record = string_from_object_code((*self).starting_address, record_width_in_bytes);
        header = header+ &record;

        //appending program length
        let record = string_from_object_code((*self).program_length, record_width_in_bytes);
        header = header+ &record;

        return header;
    }
}
