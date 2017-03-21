
use std::collections::HashMap;
use basic_types::instruction::Instruction;
use basic_types::formats::Format;
use basic_types::operands::Operand;
use basic_types::unit_or_pair::UnitOrPair;

// The operands of the instruction will be indicated as a bit vector
// as inferred from the instruction set operands can be classified to
// basic minimal units, which are
// r1, r2, m, n constructing -> r1 | r1,r2 | m | r1,n only,
// a bit flag for each instruction indicating the used construct will
// be suffecient

/// Definition of an assembly instruction in the instruction set
#[derive(Debug)]
pub struct AssemblyDef {
    pub mnemonic: String,
    pub format: UnitOrPair<Format>,
    pub operands: UnitOrPair<Operand>,
    pub op_code: u32,
}

impl AssemblyDef {
    fn new(mnemonic: String,
           formats: UnitOrPair<Format>,
           operands: UnitOrPair<Operand>, // If an instruction can be on format 3 or 4, format 3 must come first (as in the IS)
           op_code: u32)
           -> AssemblyDef {

        AssemblyDef {
            op_code: op_code,
            mnemonic: mnemonic,
            format: formats,
            operands: operands,
        }
    }

    fn has_valid_operands(&self, operands: UnitOrPair<Operand>) -> bool {

        // FIXME This function simply checks that
        // the Enum Variant of the operands and instruction match
        fn check_operand(a: &UnitOrPair<Operand>, b: &UnitOrPair<Operand>) -> bool {
            match (a, b) {
                // Possible register cases ( from the IS )

                // For clear
                (&UnitOrPair::Unit(Operand::Register(..)), 
                    &UnitOrPair::Unit(Operand::Register(..)))  // Unit <> Unit
                =>true,
                
                (&UnitOrPair::Pair(Operand::Register(..),Operand::Register(..)),
                    &UnitOrPair::Pair(Operand::Register(..),Operand::Register(..)))    // Pair <> Pair
                =>true,
                
                // For shifts
                (&UnitOrPair::Pair(Operand::Register(..),Operand::Raw(..)),
                    &UnitOrPair::Pair(Operand::Register(..),Operand::Raw(..)))
                =>true,
                


                // (&Operand::Register(..), &Operand::Register(..)) => true,
                // (&Operand::Immediate(..), &Operand::Immediate(..)) => true,
                // (&Operand::Label(..), &Operand::Label(..)) => true,
                // (&Operand::None, &Operand::None) => true,
                _ => false,
            }
        }

        check_operand(&self.operands, &operands)
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
// TODO Create a Vec<(Fn(..)->bool, &str) to store the checks and get the error
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
            let mut m = HashMap::new();
            
            let add_def=AssemblyDef::new(
                "ADD".to_owned(),
                 UnitOrPair::Unit(Format::Two),
                UnitOrPair::Unit(Operand::Label(None)),
                0x18);
            
            m.insert(add_def.mnemonic.to_owned(),add_def);
            m
        };
    }


// Quick test for enum type equality
#[test]
fn enum_variant_matching() {
    let a = Operand::Immediate(None);
    let b = Operand::Immediate(Some(6));

    // Taken by reference to avoid borrowing, TODO create a AssemblyDef struct and use its function
    fn check_operand(a: &Operand, b: &Operand) -> bool {
        match (a, b) {
            (&Operand::Register(..), &Operand::Register(..)) => true,
            (&Operand::Immediate(..), &Operand::Immediate(..)) => true,
            (&Operand::Label(..), &Operand::Label(..)) => true,
            (&Operand::None, &Operand::None) => true,
            _ => false,
        }
    }

    assert!(check_operand(&Operand::Immediate(None), &Operand::Immediate(Some(7))));
}
