// Make the linter silent
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(unused_imports)]

use basic_types::formats::Formats;
use basic_types::flags::Flags;
use basic_types::operands::Operand;
use basic_types::register::Register;

use std::collections;
use std::fmt::Debug;

const BYTE_SIZE_TO_BITS: u8 = 8; // In the SIC machine, a byte is 3 bits

/**
 * Resembles a SIC/XE instruction, this object is immutable,
 * Each method that mutates the state should return a new object
 */

pub struct Instruction {
    format: Formats,
    instruction: String,
    #[derive(Debug)]
    flags: Vec<Flags>,
    op1: Operand,
    op2: Operand,
}

impl Instruction {
    /**
     * new A plain new instruction
     */
    pub fn new(instruction: String, op1: Operand, op2: Operand) -> Instruction {
        Instruction {
            format: Formats::None,
            instruction: instruction,
            flags: Vec::new(),
            op1: op1,
            op2: op2,
        }
    }

    /**
    * to_pc_relative returns a new instructions object with PC
    * relative flag set to 1
    */
    pub fn set_flag(&mut self, flag: Flags) {

        if (*self).format != Formats::Four && (*self).format != Formats::Three {
            panic!("Format 1 or 2 can't have flags set");
        }

        // Check for flag duplication, this is will be an error of a previous function/module
        for flag_iter in &self.flags {
            if flag == *flag_iter {
                panic!("Flag {:?} already exists for this instuction", flag_iter);
            }
        }

        self.flags.push(flag);
    }

    ///
    /// set_format set the formats of the instruction
    ///
    pub fn set_format(&mut self, instruction_format: Formats) {

        if self.format != Formats::None {
            panic!("Format was already set");
        }

        self.format = instruction_format;
    }

    /**
     *  get_flags_value returns the numeric value of the flags
     *  declared on this instruction
     */
    pub fn get_flags_value(&self) -> Result<u32, &str> {
        // TODO Create an extra check to see if conflicting flags exist
        // Set the flags if the instuction is not any of the special cases
        // i.e set the Indirect and Immediate flags to 1

        // Decimal value resulting from decoding the flags
        let mut total_value: u32 = 0;

        // Calculate the instruciton length in bits
        let fmt_num = self.format;
        let instr_len: u8 = fmt_num as u8 * BYTE_SIZE_TO_BITS;

        // check if BaseRelative and PcRelative flags are set, indicate errors
        match self.check_invalid_flags() {
            Err(st) => return Err(st),
            _ => () // Do nothing on success
        };

        for flag_iter in &self.flags {

            // Note that the flag location is counted from left to write
            // to avoid relating it to the instuction length F3 / F4
            // so the total length of the instuction in bits - location
            // from left to right will give us the right value to OR

            total_value += 1 << (instr_len - *flag_iter as u8);

        }

        Ok(total_value)
    }

    fn check_invalid_flags(&self) -> Result<bool, &str> {


        if self.has_flag(Flags::BaseRelative) && self.has_flag(Flags::PcRelative) {
            return Err("PC relative and Base relative flags are set");
        }

        if self.format == Formats::Three && self.has_flag(Flags::Extended) {
            return Err("Extended bit is set with Format 3 instruction");
        }

        // Check if a format 4 instruction has any invalid flags
        if self.format == Formats::Four && !self.has_flag(Flags::Extended) {
            return Err("instuction declared as format four and the E flag isn't set");
        }

        if self.format == Formats::Four && (self.has_flag(Flags::BaseRelative)) {
            return Err("instuction declared as format four and uses base relative addressing");
        }

        if self.format == Formats::Four && self.has_flag(Flags::PcRelative) {
            return Err("instuction declared as format four and uses PC relative addressing");
        }

        if self.format == Formats::Four && self.has_flag(Flags::Indexed) {
            return Err("instuction declared as format four and uses Indexed addressing");
        }

        if self.format == Formats::Four && self.has_flag(Flags::Indirect) {
            return Err("instuction declared as format four and uses Indirect addressing");
        }

        Ok(true)
    }

    fn has_flag(&self, flag: Flags) -> bool {
        self.flags.iter().position(|&f| f == flag) != None
    }
}
