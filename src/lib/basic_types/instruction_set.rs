
use std::collections::HashMap;

// The operands of the instruction will be indicated as a bit vector
// as inferred from the instruction set operands can be classified to
// basic minimal units, which are
// r1, r2, m, n constructing -> r1 | r1,r2 | m | r1,n only,
// a bit flag for each instruction indicating the used construct will
// be suffecient

bitflags!{
   pub flags OperandFlags:u8{
        const OP_M = 1<<0,
        const OP_NUM = 1<<1,
        const OP_R1 = 1<<2,
        const OP_R2 = 1<<3,
        }
    }

lazy_static!{
    static ref INSTRUCTION_SET: HashMap<&'static str,OperandFlags > = {
            let mut m = HashMap::new();
            m.insert("ADD",OP_M);
            // add other instructions
            m
        };
    }

pub fn exists(mnemonic: &str) -> bool {
    return INSTRUCTION_SET.contains_key(mnemonic);
}
