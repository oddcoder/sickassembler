// Make the linter silent
#![allow(dead_code)]
use formats::Format;
use flags::Flags;
use std::collections::HashSet;
use operands::{OperandType, Value};
use unit_or_pair::{UnitOrPair, unwrap_to_vec};
use register::Register;
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
    flags: HashSet<Flags>, // Set and Get through functoins

    pub label: String,
    pub mnemonic: String,
    format: Format,
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
            // SIC/XE defaults ind. and imm. falgs to 1
            flags: HashSet::new(),

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
            flags: HashSet::new(),
            locctr: 0,
            operands: UnitOrPair::None,
        }
    }

    ///
    /// set_format set the formats of the instruction
    /// this is typically called after setting the operands
    ///
    pub fn set_format(&mut self, instruction_format: Format) {

        self.format = instruction_format;

        for operand in &self.unwrap_operands() {
            let flag: Option<Flags> = match operand.opr_type {
                OperandType::Immediate => Some(Flags::Immediate),
                OperandType::Indirect => Some(Flags::Indirect),
                OperandType::Register => {
                    // If the instruction has 2 operands and the second is register X
                    if operand.val == Value::Register(Register::X) &&
                       self.unwrap_operands().len() == 2 {
                        //  Remove the X register from the operands
                        // LDCH BUFFER,X 53C003
                        self.remove_op_register_x();
                        // Set the indexed flag
                        Some(Flags::Indexed)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            flag.map(|f| self.flags.insert(f));
        }

        if instruction_format == Format::One || instruction_format == Format::Two {
            return;
        }

        // If no immediate/indeirect flags were specified
        if self.flags.contains(&Flags::Immediate) == false &&
           self.flags.contains(&Flags::Indirect) == false {
            // Normal SIC/XE instructions have indirect, immediate flags set
            self.flags.extend(vec![Flags::Indirect, Flags::Immediate].into_iter());
        }

        // Format 4 can have immediate operands, and label operands
        // +LDT #4096 75101000
        // 105F LDT LENGTH 774000 where 0033 LENGTH
        if instruction_format == Format::Four {
            self.flags.insert(Flags::Extended);
        }

        info!("Set format of {:?} as {:?}", self, instruction_format);
    }

    pub fn get_format(&self) -> Format {
        self.format
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

    pub fn set_base_relative(&mut self) {
        self.flags.insert(Flags::BaseRelative);
    }

    pub fn set_pc_relative(&mut self) {
        self.flags.insert(Flags::PcRelative);
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

    /// The register X isn't converted to an object code
    /// if the instruction is in indexed mode,
    /// so it must be removed from the operand list
    /// LDCH BUFFER,X 53C003 -> Buffer is at loc 3
    pub fn remove_op_register_x(&mut self) {
        let oprs = self.unwrap_operands();
        for opr in &oprs {
            match opr.val {
                Value::Register(Register::X) => continue,
                _ => self.operands = UnitOrPair::Unit(opr.clone()),
            }
        }

    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:8} {:8} {:8} {:8} {:?}",
               self.format,
               self.locctr,
               self.label,
               self.mnemonic,
               self.operands)
    }
}
