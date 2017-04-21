use std::collections::HashMap;
use instruction::AsmOperand;
use formats::{Format, get_bit_count};
use operands::{self, OperandType, Value};
use unit_or_pair::{self, UnitOrPair};

// The operands of the instruction will be indicated as a bit vector
// as inferred from the instruction set operands can be classified to
// basic minimal units, which are
// r1, r2, m, n constructing -> r1 | r1,r2 | m | r1,n only,
// a bit flag for each instruction indicating the used construct will
// be suffecient

/// Definition of an assembly instruction in the instruction set
#[derive(Debug,Clone)]
pub struct AssemblyDef {
    pub mnemonic: String,
    pub format: UnitOrPair<Format>,
    pub operands: UnitOrPair<OperandType>,
    pub op_code: u32,
}

impl AssemblyDef {
    fn new(mnemonic: String,
           formats: UnitOrPair<Format>,
           operands: UnitOrPair<OperandType>,
           op_code: u32)
           -> AssemblyDef {

        AssemblyDef {
            op_code: op_code,
            mnemonic: mnemonic,
            format: formats,
            operands: operands,
        }
    }
    pub fn dummy() -> AssemblyDef {
        AssemblyDef::new("DUMMY".to_owned(), UnitOrPair::None, UnitOrPair::None, 0xFF)
    }

    /// Validates the operands of a given instruction
    pub fn has_valid_operands(&self, operands: &UnitOrPair<AsmOperand>) -> bool {
        // TODO: delete this function, it checks for programmers errors
        let others_ops: Vec<Value> = unit_or_pair::unwrap_to_vec(operands)
            .iter()
            .map(|o| o.clone().val)
            .collect::<Vec<Value>>();
        let def_operands: Vec<OperandType> = unit_or_pair::unwrap_to_vec(&self.operands);

        if others_ops.len() != def_operands.len() {
            return false;
        }

        // Merge the 2 collections
        let zipped = def_operands.iter().zip(others_ops);

        for (def_operand, inst_op) in zipped {
            if operands::match_value(&def_operand, &inst_op) == false {
                return false;
            }
        }

        true
    }

    /// Checks if a given format of input instruction matches
    /// the instruction set format(s)
    pub fn match_format(&self, format: &Format) -> bool {

        match (&self.format, format) {
            (&UnitOrPair::Unit(Format::One), &Format::One) => true,
            (&UnitOrPair::Unit(Format::Two), &Format::Two) => true,
            (&UnitOrPair::Pair(Format::Three, Format::Four), &Format::Three) => true,
            (&UnitOrPair::Pair(Format::Three, Format::Four), &Format::Four) => true,
            _ => false,
        }

    }

    /// Gets the value of the opcode in the instruction
    pub fn get_opcode_value(&self, format: Format) -> i32 {
        let bit_count: i32 = get_bit_count(format);
        (self.op_code << (bit_count - 8 as i32)) as i32
    }
}


/// Checks if a provided instruction exists in the Instruction set and returns it or an error
/// NOTE: The caller should check for complaince with the instruction set with respect
/// to the number of operands, and so
pub fn fetch_instruction<'a>(instr_mnemonic: &String) -> Result<AssemblyDef, &'a str> {
    let mnemonic = &instr_mnemonic.to_uppercase().to_owned();
    if INSTRUCTION_SET.contains_key(mnemonic) == false {
        warn!("Failed to find mnemonic {:?}", instr_mnemonic.as_str());
        return Err("Mnemonic isn't defined in the instruction set");
    }

    Ok(INSTRUCTION_SET.get(mnemonic).unwrap().clone())
}

lazy_static!{
    static ref INSTRUCTION_SET: HashMap<String,AssemblyDef > = {
            let isa :HashMap <String, AssemblyDef> = [
                ("ADD".to_owned(),      AssemblyDef::new("ADD".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x18)),
                ("ADDF".to_owned(),     AssemblyDef::new("ADDF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x58)),
                ("ADDR".to_owned(),     AssemblyDef::new("ADDR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x90)),
                ("AND".to_owned(),      AssemblyDef::new("AND".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x40)),
                ("CLEAR".to_owned(),    AssemblyDef::new("CLEAR".to_owned(),    UnitOrPair::Unit(Format::Two),                      UnitOrPair::Unit(OperandType::Register),                        0xB4)),
                ("COMP".to_owned(),     AssemblyDef::new("COMP".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x28)),
                ("COMPF".to_owned(),    AssemblyDef::new("COMPF".to_owned(),    UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x88)),
                ("COMPR".to_owned(),    AssemblyDef::new("COMPR".to_owned(),    UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0xA0)),
                ("DIV".to_owned(),      AssemblyDef::new("DIV".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x24)),
                ("DIVF".to_owned(),     AssemblyDef::new("DIVF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x64)),
                ("DIVR".to_owned(),     AssemblyDef::new("DIVR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x9C)),
                ("FIX".to_owned(),      AssemblyDef::new("FIX".to_owned(),      UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                               0xC4)),
                ("FLOAT".to_owned(),    AssemblyDef::new("FLOAT".to_owned(),    UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                               0xC0)),
                ("HIO".to_owned(),      AssemblyDef::new("HIO".to_owned(),      UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                               0xF4)),
                ("J".to_owned(),        AssemblyDef::new("J".to_owned(),        UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x3C)),
                ("JEQ".to_owned(),      AssemblyDef::new("JEQ".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x30)),
                ("JGT".to_owned(),      AssemblyDef::new("JGT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x34)),
                ("JLT".to_owned(),      AssemblyDef::new("JLT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x38)),
                ("JSUB".to_owned(),     AssemblyDef::new("JSUB".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x48)),
                ("LDA".to_owned(),      AssemblyDef::new("LDA".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x00)),
                ("LDB".to_owned(),      AssemblyDef::new("LDB".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x68)),
                ("LDCH".to_owned(),     AssemblyDef::new("LDHC".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x50)),
                ("LDF".to_owned(),      AssemblyDef::new("LDF".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x70)),
                ("LDL".to_owned(),      AssemblyDef::new("LDL".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x08)),
                ("LDS".to_owned(),      AssemblyDef::new("LDS".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x6C)),
                ("LDT".to_owned(),      AssemblyDef::new("LDT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x74)),
                ("LDX".to_owned(),      AssemblyDef::new("LDX".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x04)),
                ("LPS".to_owned(),      AssemblyDef::new("LPS".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xD0)),
                ("MUL".to_owned(),      AssemblyDef::new("MUL".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x20)),
                ("MULF".to_owned(),     AssemblyDef::new("MULF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x60)),
                ("MULR".to_owned(),     AssemblyDef::new("MULR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x98)),
                ("NORM".to_owned(),     AssemblyDef::new("NORM".to_owned(),     UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                               0xC8)),
                ("OR".to_owned(),       AssemblyDef::new("OR".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x44)),
                ("RD".to_owned(),       AssemblyDef::new("RD".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xD8)),
                ("RMO".to_owned(),      AssemblyDef::new("RMO".to_owned(),      UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0xAC)),
                ("RSUB".to_owned(),     AssemblyDef::new("RSUB".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x4C)),
                ("SHIFTL".to_owned(),   AssemblyDef::new("SHIFTL".to_owned(),   UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Immediate),0xA4)),
                ("SHIFTR".to_owned(),   AssemblyDef::new("SHIRFT".to_owned(),   UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Immediate),0xA8)),
                ("SIO".to_owned(),     AssemblyDef::new("SIO".to_owned(),       UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                               0xF0)),
                ("SSK".to_owned(),      AssemblyDef::new("SSK".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xEC)),
                ("STA".to_owned(),      AssemblyDef::new("STA".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x0C)),
                ("STB".to_owned(),      AssemblyDef::new("STB".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x78)),
                ("STCH".to_owned(),     AssemblyDef::new("STCH".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x54)),
                ("STF".to_owned(),      AssemblyDef::new("STF".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x80)),
                ("STI".to_owned(),      AssemblyDef::new("STI".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xD4)),
                ("STL".to_owned(),      AssemblyDef::new("STL".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x14)),
                ("STS".to_owned(),      AssemblyDef::new("STS".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x7C)),
                ("STSW".to_owned(),     AssemblyDef::new("STSW".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xE8)),
                ("STT".to_owned(),      AssemblyDef::new("STT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x84)),
                ("STX".to_owned(),      AssemblyDef::new("STX".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x10)),
                ("SUB".to_owned(),      AssemblyDef::new("SUB".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x1C)),
                ("SUBF".to_owned(),     AssemblyDef::new("SUBF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x5C)),
                ("SUBR".to_owned(),     AssemblyDef::new("SUBR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x94)),
                ("SVC".to_owned(),      AssemblyDef::new("SVC".to_owned(),      UnitOrPair::Unit(Format::Two),                      UnitOrPair::Unit(OperandType::Immediate),                       0xB0)),
                ("TD".to_owned(),       AssemblyDef::new("TD".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xE0)),
                ("TIO".to_owned(),      AssemblyDef::new("TIO".to_owned(),      UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                               0xF8)),
                ("TIX".to_owned(),      AssemblyDef::new("TIX".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0x2C)),
                ("TIXR".to_owned(),     AssemblyDef::new("TIXR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Unit(OperandType::Register),                        0xB8)),
                ("WD".to_owned(),       AssemblyDef::new("WD".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                       0xDC))
                    ].iter().cloned().collect();
            return isa;
        };
    }

/// Checks if a provided directive exists in the Directive table and returns it or an error
/// if the mnemonic doesn't exist
/// NOTE: The caller should check for complaince with the directory table with respect
/// to the number of operands, and so
pub fn fetch_directive<'a>(instr_mnemonic: &String) -> Result<AssemblyDef, &'a str> {

    let mnemonic = &instr_mnemonic.to_uppercase().to_owned();
    if ASSEMBLER_DIRECTIVES.contains_key(mnemonic) == false {
        warn!("Failed to find directive {:?}", instr_mnemonic.as_str());

        return Err("Directive isn't defined in the instruction set");
    }

    Ok(ASSEMBLER_DIRECTIVES.get(mnemonic).unwrap().clone())
}

/// Assembler directives that will trigger a special action
pub fn is_base_mode_directive(directive_mnemonic: &str) -> Option<String> {

    let upper_cased = directive_mnemonic.to_uppercase();

    match upper_cased.as_str() {
        "BASE" | "NOBASE" => Some(upper_cased.to_owned()),
        _ => None,
    }
}

lazy_static!{
    static ref ASSEMBLER_DIRECTIVES: HashMap<String,AssemblyDef > = {
            let assembler_directives :HashMap <String, AssemblyDef> = [
            ("END".to_owned(),
                  AssemblyDef::new("END".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::Immediate),0xFF)),
            ("BYTE".to_owned(),
                  AssemblyDef::new("BYTE".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::Bytes),0xFF)),
            ("WORD".to_owned(),
                  AssemblyDef::new("WORD".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::Immediate),0xFF)),
            ("RESB".to_owned(),
                  AssemblyDef::new("RESB".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::Immediate),0xFF)),
            ("RESW".to_owned(),
                  AssemblyDef::new("RESW".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::Immediate),0xFF)),
            ("BASE".to_owned(),
                  AssemblyDef::new("BASE".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::Label),0xFF)),
            ("NOBASE".to_owned(),
                  AssemblyDef::new("NOBASE".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::None),0xFF)),
            ("LTORG".to_owned(),
                  AssemblyDef::new("LTORG".to_owned(),
                  UnitOrPair::Unit(Format::None),
                  UnitOrPair::Unit(OperandType::None),0xFF)),
                    ].iter().cloned().collect();
            return assembler_directives;
        };
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directive_table_check_positive() {
        let result: Result<AssemblyDef, &str> = fetch_directive(&"base".to_owned());
        assert!(result.is_ok());
    }

    #[test]
    fn directive_table_check_negative() {
        let result: Result<AssemblyDef, &str> = fetch_directive(&"ended".to_owned());
        assert!(result.is_err());
    }
}
