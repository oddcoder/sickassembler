// Make the linter silent
#![allow(dead_code)]


use basic_types::formats::Formats;
use basic_types::flags::Flags;
use basic_types::operands::Operand;


const BYTE_SIZE_TO_BITS: u8 = 8; // In the SIC machine, a byte is 3 bits

/**
 * Resembles a SIC/XE instruction, this object is immutable,
 * Each method that mutates the state should return a new object
 */

pub struct Instruction {
    label: String,
    format: Formats,
    instruction: String,
    flags: Vec<Flags>,

    // Operands are used in pass2, thus those fields are made public
    pub op1: Operand,
    pub op2: Operand,
}

impl Instruction {
    /**
     * new A plain new instruction
     */
    pub fn new(label: String, instruction: String, op1: Operand, op2: Operand) -> Instruction {
        Instruction {
            label: label,
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

        if self.format == Formats::None {
            panic!("Instruction format isnt specified");
        }

        // Decimal value resulting from decoding the flags
        let mut total_value: u32 = 0;

        // Calculate the instruciton length in bits
        let fmt_num = self.format;
        let instr_len: u8 = fmt_num as u8 * BYTE_SIZE_TO_BITS;

        // check if BaseRelative and PcRelative flags are set, indicate errors
        match self.check_invalid_flags() {
            Err(st) => return Err(st),
            _ => (), // Continue execution on success
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

    fn check_invalid_flags(&self) -> Result<(), &str> {

        // TODO Extract all the errors in the instruction,
        // don't return just a sinle string and use Vec<string>

        if self.has_flag(Flags::BaseRelative) && self.has_flag(Flags::PcRelative) {
            return Err("PC relative and Base relative flags are set together");
        }

        if self.format == Formats::Three && self.has_flag(Flags::Extended) {
            return Err("Extended bit is set in a Format 3 instruction");
        }

        // Check if a format 4 instruction has any invalid flags
        if self.format == Formats::Four && !self.has_flag(Flags::Extended) {
            return Err("E flag isn't set in a format 4 instruction");
        }

        if self.format == Formats::Four &&
           (self.has_flag(Flags::Indirect) || self.has_flag(Flags::Indexed)) {
            return Err("Indirect/Indexed addressing used in a format 4 instruction");
        }

        if self.format == Formats::Four && (self.has_flag(Flags::BaseRelative)) {
            return Err("Base relative addressing used in a format 4 instruction");
        }

        // TODO confirm correctness
        if self.format == Formats::Four && self.has_flag(Flags::PcRelative) {
            return Err("PC relative addressing used in a format 4 instruction");
        }

        if self.format == Formats::Four && self.has_flag(Flags::Indexed) {
            return Err("Indexed addressing used in a format 4 instruction");
        }

        if self.format == Formats::Four && self.has_flag(Flags::Indirect) {
            return Err("Indirect addressing used in a format 4 instruction");
        }

        Ok(())
    }

    fn has_flag(&self, flag: Flags) -> bool {
        self.flags.iter().position(|&f| f == flag) != None
    }
}