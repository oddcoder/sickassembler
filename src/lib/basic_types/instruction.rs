// Make the linter silent
#![allow(dead_code)]
use basic_types::formats::Format;
use basic_types::flags::Flags;
use basic_types::operands::{OperandType, Value};
use basic_types::unit_or_pair::UnitOrPair;
use std::clone::Clone;
const BYTE_SIZE_TO_BITS: u8 = 8; // In the SIC machine, a byte is 3 bits


#[derive(Debug,Clone)]
pub struct AsmOperand {
    pub opr_type: OperandType,
    pub val: Value,
}

/**
 * Resembles a SIC/XE instruction, this object is immutable,
 * Each method that mutates the state should return a new object
 */
#[derive(Debug)]
pub struct Instruction {
    flags: Vec<Flags>, // Set and Get through functoins

    pub label: String,
    pub mnemonic: String,
    pub format: Format,
    pub operands: UnitOrPair<AsmOperand>, // Group oerands in one field
}

impl Instruction {
    /**
     * new A plain new instruction
     * use builder pattern? ( as it's transromed in phases and to make testing less verbose)
     */
    pub fn new(label: String, mnemonic: String, operands: UnitOrPair<AsmOperand>) -> Instruction {
        Instruction {
            label: label,
            format: Format::None,
            mnemonic: mnemonic,
            flags: Vec::new(),
            operands: operands,
        }
    }

    /*
     *   Creates an instruction with only a mnemonic
     */
    pub fn new_simple(mnemonic: String) -> Instruction {
        Instruction {
            label: String::new(),
            format: Format::None,
            mnemonic: mnemonic,
            flags: Vec::new(),
            operands: UnitOrPair::None,
        }
    }

    /**
    * to_pc_relative returns a new instructions object with PC
    * relative flag set to 1
    */
    pub fn set_flag(&mut self, flag: Flags) -> Result<(), &str> {

        if (*self).format != Format::Four && (*self).format != Format::Three {
            warn!("Format 1 or 2 can't have flags set");
            return Err("Format 1 or 2 can't have flags set");
        }

        // Check for flag duplication, this is will be an error of a previous function/module
        for flag_iter in &self.flags {
            if flag == *flag_iter {
                warn!("Flag {:?} already exists for this instuction", flag_iter);
                return Err("Duplicate flag on instruction");
            }
        }

        self.flags.push(flag);

        info!("Added flag {:?} to instruction {:?}", flag, self);
        Ok(())
    }

    ///
    /// set_format set the formats of the instruction
    ///
    pub fn set_format(&mut self, instruction_format: Format) -> Result<(), &str> {

        if self.format != Format::None {
            warn!("Format was already set for {:?}", self);
            return Err("Format was already set");
        }

        self.format = instruction_format;

        info!("Set format of {:?} as {:?}", self, instruction_format);
        Ok(())
    }

    pub fn add_label(&mut self, label: String) -> Result<(), &str> {

        if self.label.len() > 0 {
            warn!("Resetting label as {:?} for {:?}", label, self.label);
            return Err("Label was already set");
        }

        Ok(self.label = label)
    }

    /**
    *   Adds an operand to the instruction as appropriate
    **/
    pub fn add_operand(&mut self, op_type: OperandType, op_value: Value) -> Result<(), &str> {

        let op = AsmOperand {
            opr_type: op_type,
            val: op_value,
        };

        // Match on a copy, modify the original
        match self.operands.clone() {
            UnitOrPair::Unit(asm) => Ok(self.operands = UnitOrPair::Pair(asm, op)),

            UnitOrPair::Pair(..) => {
                warn!("Instruction {:?} can't have more than 2 operands", self);
                return Err("Instruction can't have more than 2 operands");
            }
            UnitOrPair::None => Ok(self.operands = UnitOrPair::Unit(op)),
        }
    }

    /**
     *  get_flags_value returns the numeric value of the flags
     *  declared on this instruction
     */
    pub fn get_flags_value(&self) -> Result<u32, String> {
        // TODO Create an extra check to see if conflicting flags exist
        // Set the flags if the instuction is not any of the special cases
        // i.e set the Indirect and Immediate flags to 1

        //Format4: n i x b p e | 20-addr - 0-indexed
        //Format3: n i x b p e | 12-addr - 0-indexed
        if self.format == Format::None {
            warn!("Instruction {:?} format isnt specified", self);
            return Err("Instruction format wasn't specified".to_owned());
        }

        // Decimal value resulting from decoding the flags
        let mut total_value: u32 = 0;

        // Calculate the instruciton length in bits
        let fmt_num = self.format;
        let instr_len: u8 = fmt_num as u8 * BYTE_SIZE_TO_BITS;

        // check if BaseRelative and PcRelative flags are set, indicate errors
        match self.check_invalid_flags() {
            Err(st) => {
                warn!("Error when checking flags of {:?};", self);
                return Err(st);
            }
            _ => (), // Continue execution on success
        };

        for flag_iter in &self.flags {

            // Note that the flag location is counted from left to write
            // to avoid relating it to the instuction length F3 / F4
            // so the total length of the instuction in bits - location
            // from left to right will give us the right value to OR

            total_value += 1 << (instr_len - *flag_iter as u8);

        }

        info!("Value of flags in {:?} is {:?}", self, total_value);
        Ok(total_value)
    }

    fn check_invalid_flags<'a>(&'a self) -> Result<(), String> {

        // TODO Extract all the errors in the instruction flags,
        // don't return just a sinle string and use Vec<string>
        let mut errors: Vec<&str> = Vec::new();

        if self.has_flag(Flags::BaseRelative) && self.has_flag(Flags::PcRelative) {
            errors.push("PC relative and Base relative flags are set together");
        }

        if self.format == Format::Three && self.has_flag(Flags::Extended) {
            errors.push("Extended bit is set in a Format 3 instruction");
        }

        // Check if a format 4 instruction has any invalid flags
        if self.format == Format::Four && !self.has_flag(Flags::Extended) {
            errors.push("E flag isn't set in a format 4 instruction");
        }

        if self.format == Format::Four &&
           (self.has_flag(Flags::Indirect) || self.has_flag(Flags::Indexed)) {
            errors.push("Indirect/Indexed addressing used in a format 4 instruction");
        }

        if self.format == Format::Four && (self.has_flag(Flags::BaseRelative)) {
            errors.push("Base relative addressing used in a format 4 instruction");
        }

        // TODO confirm correctness
        if self.format == Format::Four && self.has_flag(Flags::PcRelative) {
            errors.push("PC relative addressing used in a format 4 instruction");
        }

        if self.format == Format::Four && self.has_flag(Flags::Indexed) {
            errors.push("Indexed addressing used in a format 4 instruction");
        }

        if self.format == Format::Four && self.has_flag(Flags::Indirect) {
            errors.push("Indirect addressing used in a format 4 instruction");
        }

        if errors.len() > 0 {
            let errs: String = errors.join(", ");
            warn!("Found errors in {:?} flags: {:?}", self, errs);
            return Err(errs);
        }

        Ok(())
    }


    fn has_flag(&self, flag: Flags) -> bool {
        self.flags.iter().position(|&f| f == flag) != None
    }

    // FIXME This function simply checks that
    // the Enum Variant of the operands and instruction match
    pub fn unwrap_operands(&self) -> Vec<AsmOperand> {
        let operands = match &self.operands {
            // Possible register cases ( from the IS )
            // For clear
            // Unit <> Unit
            &UnitOrPair::None => vec![],
            &UnitOrPair::Unit(ref o1) => vec![o1.clone()],
            &UnitOrPair::Pair(ref o1, ref o2) => vec![o1.clone(), o2.clone()],
        };

        info!("Operands of {:?} are {:?}", self, operands);
        operands
    }
}
