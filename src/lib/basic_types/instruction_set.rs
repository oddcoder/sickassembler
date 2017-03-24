
use std::collections::HashMap;
use basic_types::instruction::Instruction;
use basic_types::formats::Format;
use basic_types::operands::OperandType;
use basic_types::unit_or_pair::UnitOrPair;

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
           operands: UnitOrPair<OperandType>, // If an instruction can be on format 3 or 4, format 3 must come first (as in the IS)
           op_code: u32)
           -> AssemblyDef {

        AssemblyDef {
            op_code: op_code,
            mnemonic: mnemonic,
            format: formats,
            operands: operands,
        }
    }

    fn has_valid_operands(&self, operands: UnitOrPair<OperandType>) -> bool {

        // FIXME This function simply checks that
        // the Enum Variant of the operands and instruction match
        fn check_operands(a: &UnitOrPair<OperandType>, b: &UnitOrPair<OperandType>) -> bool {
            match (a, b) {
                // Possible register cases ( from the IS )

                // For clear
                (&UnitOrPair::Unit(OperandType::Register), 
                    &UnitOrPair::Unit(OperandType::Register))  // Unit <> Unit
                =>true,
                
                (&UnitOrPair::Pair(OperandType::Register,OperandType::Register),
                    &UnitOrPair::Pair(OperandType::Register,OperandType::Register))    // Pair <> Pair
                =>true,
                
                // For shifts
                (&UnitOrPair::Pair(OperandType::Register,OperandType::Raw),
                    &UnitOrPair::Pair(OperandType::Register,OperandType::Raw))
                =>true,
                _ => false,
            }
        }

        check_operands(&self.operands, &operands)
    }

    fn match_format(&self, format: &Format) -> bool {

        match (&self.format, format) {
            (&UnitOrPair::Unit(Format::One), &Format::One) => true,
            (&UnitOrPair::Unit(Format::Two), &Format::Two) => true,
            (&UnitOrPair::Pair(Format::Three, Format::Four), &Format::Three) => true,
            (&UnitOrPair::Pair(Format::Three, Format::Four), &Format::Four) => true,
            _ => false,
        }

    }
}


/// Checks if a provided instruction exists in the Instruction set and returns it or an error
pub fn fetch_instruction(instr: Instruction) -> Result<&'static AssemblyDef, &'static str> {

    if INSTRUCTION_SET.contains_key(&instr.mnemonic) == false {
        return Err("Mnemonic isn't defined in the instruction set");
    }

    match INSTRUCTION_SET.get(&instr.mnemonic) {
        Some(asm_def) => Ok(asm_def),
        _ => Err("Instruction not found"),
    }
}

// Dead code, will be used later
// TODO Create a Vec<(Fn->bool, &str) to store the checks and get the error
// message if a check fails
// if !asm_def.match_format(instr.format) {
//     return Err("Instruction format is invalid for this mnemonic");
// }
// if !asm_def.has_valid_operands(instr.operands) {
//     return Err("Operands field is invalid");
// }
// Ok(asm_def.op_code)

lazy_static!{
    static ref INSTRUCTION_SET: HashMap<String,AssemblyDef > = {
            let isa :HashMap <String, AssemblyDef> = [
                ("ADD".to_owned(),      AssemblyDef::new("ADD".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x18)),
                ("ADDF".to_owned(),     AssemblyDef::new("ADDF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x58)),
                ("ADDR".to_owned(),     AssemblyDef::new("ADDR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x90)),
                ("AND".to_owned(),      AssemblyDef::new("AND".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x40)),
                ("CLEAR".to_owned(),    AssemblyDef::new("CLEAR".to_owned(),    UnitOrPair::Unit(Format::Two),                      UnitOrPair::Unit(OperandType::Register),                          0xB4)),
                ("COMP".to_owned(),     AssemblyDef::new("COMP".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x28)),
                ("COMPF".to_owned(),    AssemblyDef::new("COMPF".to_owned(),    UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x88)),
                ("COMPR".to_owned(),    AssemblyDef::new("COMPR".to_owned(),    UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0xA0)),
                ("DIV".to_owned(),      AssemblyDef::new("DIV".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x24)),
                ("DIVF".to_owned(),     AssemblyDef::new("DIVF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x64)),
                ("DIVR".to_owned(),     AssemblyDef::new("DIVR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x9C)),
                ("FIX".to_owned(),      AssemblyDef::new("FIX".to_owned(),      UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                                   0xC4)),
                ("FLOAT".to_owned(),    AssemblyDef::new("FLOAT".to_owned(),    UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                                   0xC0)),
                ("HIO".to_owned(),      AssemblyDef::new("HIO".to_owned(),      UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                                   0xF4)),
                ("J".to_owned(),        AssemblyDef::new("J".to_owned(),        UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x3C)),
                ("JEQ".to_owned(),      AssemblyDef::new("JEQ".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x30)),
                ("JGT".to_owned(),      AssemblyDef::new("JGT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x34)),
                ("JLT".to_owned(),      AssemblyDef::new("JLT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x38)),
                ("JSUB".to_owned(),     AssemblyDef::new("JSUB".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x48)),
                ("LDA".to_owned(),      AssemblyDef::new("LDA".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x00)),
                ("LDB".to_owned(),      AssemblyDef::new("LDB".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x68)),
                ("LDHC".to_owned(),     AssemblyDef::new("LDHC".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x50)),
                ("LDF".to_owned(),      AssemblyDef::new("LDF".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x70)),
                ("LDL".to_owned(),      AssemblyDef::new("LDL".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x08)),
                ("LDS".to_owned(),      AssemblyDef::new("LDS".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x6C)),
                ("LDT".to_owned(),      AssemblyDef::new("LDT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x74)),
                ("LDX".to_owned(),      AssemblyDef::new("LDX".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x04)),
                ("LPS".to_owned(),      AssemblyDef::new("LPS".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xD0)),
                ("MUL".to_owned(),      AssemblyDef::new("MUL".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x20)),
                ("MULF".to_owned(),     AssemblyDef::new("MULF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x60)),
                ("MULR".to_owned(),     AssemblyDef::new("MULR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x98)),
                ("NORM".to_owned(),     AssemblyDef::new("NORM".to_owned(),     UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                                   0xC8)),
                ("OR".to_owned(),       AssemblyDef::new("OR".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x44)),
                ("RD".to_owned(),       AssemblyDef::new("RD".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xD8)),
                ("RMO".to_owned(),      AssemblyDef::new("RMO".to_owned(),      UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0xAC)),
                ("RSUB".to_owned(),     AssemblyDef::new("RSUB".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x4C)),
                ("SHIFTL".to_owned(),   AssemblyDef::new("SHIFTL".to_owned(),   UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Immediate),      0xA4)),
                ("SHIFTR".to_owned(),   AssemblyDef::new("SHIRFT".to_owned(),   UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Immediate),      0xA8)),
                ("SIO".to_owned(),     AssemblyDef::new("SIO".to_owned(),       UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                                   0xF0)),
                ("SSK".to_owned(),      AssemblyDef::new("SSK".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xEC)),
                ("STA".to_owned(),      AssemblyDef::new("STA".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x0C)),
                ("STB".to_owned(),      AssemblyDef::new("STB".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x78)),
                ("STCH".to_owned(),     AssemblyDef::new("STCH".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x54)),
                ("STF".to_owned(),      AssemblyDef::new("STF".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x80)),
                ("STI".to_owned(),      AssemblyDef::new("STI".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xD4)),
                ("STL".to_owned(),      AssemblyDef::new("STL".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x14)),
                ("STS".to_owned(),      AssemblyDef::new("STS".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x7C)),
                ("STSW".to_owned(),     AssemblyDef::new("STSW".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xE8)),
                ("STT".to_owned(),      AssemblyDef::new("STT".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x84)),
                ("STX".to_owned(),      AssemblyDef::new("STX".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x10)),
                ("SUB".to_owned(),      AssemblyDef::new("SUB".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x1C)),
                ("SUBF".to_owned(),     AssemblyDef::new("SUBF".to_owned(),     UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x5C)),
                ("SUBR".to_owned(),     AssemblyDef::new("SUBR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Pair(OperandType::Register, OperandType::Register), 0x94)),
                ("SVC".to_owned(),      AssemblyDef::new("SVC".to_owned(),      UnitOrPair::Unit(Format::Two),                      UnitOrPair::Unit(OperandType::Immediate),                         0xB0)),
                ("TD".to_owned(),       AssemblyDef::new("TD".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xE0)),
                ("TIO".to_owned(),      AssemblyDef::new("TIO".to_owned(),      UnitOrPair::Unit(Format::One),                      UnitOrPair::None,                                                   0xF8)),
                ("TIX".to_owned(),      AssemblyDef::new("TIX".to_owned(),      UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0x2C)),
                ("TIXR".to_owned(),     AssemblyDef::new("TIXR".to_owned(),     UnitOrPair::Unit(Format::Two),                      UnitOrPair::Unit(OperandType::Register),                          0xB8)),
                ("WD".to_owned(),       AssemblyDef::new("WD".to_owned(),       UnitOrPair::Pair(Format::Three, Format::Four),      UnitOrPair::Unit(OperandType::Immediate),                         0xDC))
                    ].iter().cloned().collect();
            return isa;
        };
    }


// Quick test for enum type equality
#[test]
fn enum_variant_matching() {
    let a = OperandType::Immediate;
    let b = OperandType::Immediate(Some(6));

    // Taken by reference to avoid borrowing, TODO create a AssemblyDef struct and use its function
    fn check_operand(a: &Operand, b: &Operand) -> bool {
        match (a, b) {
            (&OperandType::Register, &OperandType::Register) => true,
            (&OperandType::Immediate, &OperandType::Immediate) => true,
            (&OperandType::Label, &OperandType::Label) => true,
            (&OperandType::None, &OperandType::None) => true,
            _ => false,
        }
    }

    assert!(check_operand(&OperandType::Immediate, &OperandType::Immediate(Some(7))));
}
