// Make the linter silent
#![allow(dead_code)]
use basic_types::formats::Format;
use basic_types::flags::Flags;
use basic_types::operands::{OperandType, Value};
use basic_types::unit_or_pair::{UnitOrPair, unwrap_to_vec};
use basic_types::register::Register;
use std::clone::Clone;
use std::fmt;

const BYTE_SIZE_TO_BITS: u8 = 8; // In the SIC machine, a byte is 3 bits


#[derive(Clone)]
pub struct AsmOperand {
    pub opr_type: OperandType,
    pub val: Value,
}

impl AsmOperand {
    pub fn new(op_t: OperandType, value: Value) -> AsmOperand {
        AsmOperand {
            opr_type: op_t,
            val: value,
        }
    }
}

impl fmt::Debug for AsmOperand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {:?} {:?} }}", self.opr_type, self.val)
    }
}

/**
 * Resembles a SIC/XE instruction, this object is immutable,
 * Each method that mutates the state should return a new object
 */
#[derive(Clone)]
pub struct Instruction {
    flags: Vec<Flags>, // Set and Get through functoins

    pub label: String,
    pub mnemonic: String,
    pub format: Format,
    pub operands: UnitOrPair<AsmOperand>, // Group oerands in one field
    pub locctr: i32, // Signed because it'll be subtracted from signed quantities
}

impl Instruction {
    /**
     * new A plain new instruction
     * use builder pattern? ( as it's transromed in phases and to make testing less verbose)
     */
    pub fn new(label: String, mnemonic: String, operands: UnitOrPair<AsmOperand>) -> Instruction {
        let mut inst = Instruction {
            label: label,
            format: Format::None,
            mnemonic: mnemonic,
            locctr: 0,
            flags: Vec::new(),
            // Set the operands with the add_operand function to raise the flags
            operands: UnitOrPair::None,
        };

        for op in unwrap_to_vec(&operands) {
            if let Err(e) = inst.add_operand(op) {
                panic!("{}", e);
            }
        }
        inst
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
            locctr: 0,
            operands: UnitOrPair::None,
        }
    }

    ///
    /// set_format set the formats of the instruction
    ///
    pub fn set_format(&mut self, instruction_format: Format) {

        self.format = instruction_format;

        if instruction_format == Format::Four {
            self.flags.push(Flags::Extended);
        }

        info!("Set format of {:?} as {:?}", self, instruction_format);
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
    fn add_operand(&mut self, op: AsmOperand) -> Result<(), &str> {

        let mut flag_type: Option<Flags> = None;

        match op.opr_type {
            OperandType::Immediate => flag_type = Some(Flags::Immediate),
            OperandType::Indirect => flag_type = Some(Flags::Indirect),
            OperandType::Register if op.val == Value::Register(Register::X) => {
                self.flags.push(Flags::Indexed);
                return Ok(());
            }
            _ => (),
        }

        if let Some(flag) = flag_type {
            // TODO: should we check the inverse condition?
            // will it cause bugs in case of adding operands before
            // setting the instruction format
            if self.format == Format::One || self.format == Format::Two {
                warn!("Format 1 or 2 can't have flags set");
                return Err("Format 1 or 2 can't have flags set");
            }

            self.flags.push(flag);
            info!("Added flag {:?} to instruction {:?}", flag, self);
        }

        // Match on a copy, modify the original
        match self.operands.clone() {
            UnitOrPair::Unit(asm) => Ok(self.operands = UnitOrPair::Pair(asm, op)),
            UnitOrPair::None => Ok(self.operands = UnitOrPair::Unit(op)),
            _ => panic!("Invalid operands"),
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

    fn has_flag(&self, flag: Flags) -> bool {
        self.flags.iter().position(|&f| f == flag) != None
    }

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

    pub fn set_addressing_mode(&mut self, flag: Flags) {
        if flag != Flags::PcRelative && flag != Flags::BaseRelative {
            panic!("This function doesn't set any flags other than P,B");
        }

        self.flags.push(flag);
    }

    fn check_invalid_flags<'a>(&'a self) -> Result<(), String> {

        // TODO: Extract all the errors in the instruction flags,
        // as an array of (bool , fn)
        let mut errors: Vec<&str> = Vec::new();

        if self.has_flag(Flags::BaseRelative) && self.has_flag(Flags::PcRelative) {
            errors.push("PC relative and Base relative flags are set together");
        }

        if self.has_flag(Flags::Indexed) && self.has_flag(Flags::Immediate) {
            errors.push("Indexed and immediate flags are set together");
        }


        if self.has_flag(Flags::Indexed) && self.has_flag(Flags::Indirect) {
            errors.push("Indexed and extended flags are set together");
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

    /// Add the A register for instructions that are format 2 but take one operand
    /// this fixes object code generation as the first register parameter doesn't get
    /// padded with a zero, ex. TIXR T should be B850 but is calculated as B805
    /// adding the A register will pad it with 0
    pub fn add_reg_a(&mut self) {
        self.add_operand(AsmOperand::new(OperandType::Register, Value::Register(Register::A)))
            .unwrap();
    }

    pub fn get_first_operand(&self) -> AsmOperand {
        unwrap_to_vec(&self.operands)[0].clone()
    }

    pub fn get_second_operand(&self) -> AsmOperand {
        unwrap_to_vec(&self.operands)[1].clone()
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               " F: {}, Loc: {}, Label: {}, {} {:?}",
               self.format,
               self.locctr,
               self.label,
               self.mnemonic,
               self.operands)
    }
}
