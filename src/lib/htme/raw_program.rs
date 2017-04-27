use std::io::prelude::*;
use std::fs::File;
use formats::Format;
use instruction::Instruction;
use operands::*;
use htme::record_string::*;
use std::fmt;

pub struct RawProgram {
    pub program_name: String,
    pub starting_address: u32,
    pub program_length: u32,
    pub program: Vec<(String, Instruction)>,
    pub first_instruction_address: u32,
}


impl RawProgram {
    // pub fn new(program_name: String, starting_address: u32, program_length:u32, program: Vec<(u32, String, Instruction)>, first_instruction_address: u32)->RawProgram{
    //
    //     let mut vec = vec![];
    //     for &(address, ref code, ref instruction) in program.iter(){
    //         vec.push((address, code.to_owned(), (*instruction).get_format()));
    //     }
    //
    //     RawProgram{
    //         program_name: program_name,
    //         starting_address: starting_address,
    //         program_length: program_length,
    //         program: vec,
    //         first_instruction_address: first_instruction_address
    //     }
    //
    // }

    pub fn end_record(&self) -> String {

        //initing record
        let record = String::from("E");

        //getting hexacode s from 1st instruction address with right amount of zeros.
        let record_width_in_bytes: u8 = 3;
        let s = string_from_object_code(self.first_instruction_address, record_width_in_bytes);

        //returning "E"+Some Hexa
        return record + s.as_str();
    }


    //TODO:make this a lot less ugly
    pub fn text_records(&self) -> String {
        //intializing prev_address and prev_format
        let ref program = self.program;
        let (_, ref first_instruction) = program[0];
        let first_address = first_instruction.locctr;
        let mut prev_address = first_address;
        let mut prev_format = first_instruction.get_format();
        let mut records = String::from("");

        //counters
        let mut i = 0;
        let mut begin = 0;
        let mut bytes_left = 30;

        //iterating on program: address, code, instruction tuple.
        for &(_, ref instruction) in program.iter() {
            //I only care about instruction's format
            let format = (*instruction).get_format();
            let address = (*instruction).locctr;

            //terminating conditions for a single T record
            if address - prev_address > prev_format as i32 || bytes_left < format as u32 {
                let mut vec = vec![];
                vec.extend_from_slice(&program[begin..i]);
                let record = text_record_from_program(&vec);
                if begin != 0 {
                    records.push_str("\n");
                }
                records.push_str(&record);
                begin = i;
                bytes_left = 30;
            }
            bytes_left = bytes_left - format as u32;
            i = i + 1;
            prev_address = address;
            prev_format = format;
        }

        let mut vec = vec![];
        vec.extend_from_slice(&program[begin..i]);
        let record = text_record_from_program(&vec);
        if begin != 0 {
            records.push_str("\n");
        }
        records.push_str(&record);

        return records;
    }


    pub fn header_record(&self) -> String {
        let record_type = String::from("H");
        let mut header = record_type;

        //6 columns
        let record_width_in_bytes: u8 = 3;

        //appending starting address
        let record = string_from_object_code(self.starting_address, record_width_in_bytes);
        header = header + &record;

        //appending program length
        let record = string_from_object_code(self.program_length, record_width_in_bytes);
        header = header + &record;

        return header;
    }

    pub fn modification_records(&self) -> String {
        let mut records = String::from("");

        for &(_, ref instruction) in self.program.iter() {
            let format = instruction.get_format();
            let address: u32 = instruction.locctr as u32;

            if format == Format::Four {
                let operands_vector = instruction.unwrap_operands();
                if operands_vector[0].opr_type == OperandType::Label {
                    let record =
                        String::from("M") + string_from_object_code(address + 1, 3).as_str() +
                        string_from_object_code(5, 1).as_str() + "\n";
                    records = records + record.as_str();
                }
            }
        }
        return records;
    }

    pub fn all_records(&self) -> String {
        return format!("{}\n{}\n{}{}",
                       self.header_record(),
                       self.text_records(),
                       self.modification_records(),
                       self.end_record());
    }


    pub fn output_to_file(&self) {
        let mut file = match File::create(self.program_name.clone()) {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };
        write!(file, "{}", self.all_records());
    }
}

impl fmt::Debug for RawProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "START Operand: {:?} Name: {:?}, Length: {:?}, First execution Address: {:?}",
               self.first_instruction_address,
               self.program_name,
               self.program_length,
               self.starting_address)
    }
}
