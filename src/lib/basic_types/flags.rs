#![allow(dead_code)]

use std::fmt;
const OP_CODE_LEN_F3_F4: u8 = 6; // Length of the opcode field in format 3/4

// Be printable
#[derive(Debug,Copy,Clone)]
// Be equatable
#[derive(PartialEq)]
// Be in 8 bytes
#[repr(u8)]
pub enum Flags {
    /**
     * Each enum value will represent the bit number to be set calculated from left to
     * write to avoid conflict with format 3 and 4 bit locations
     *
     * The op code length is added for clarity
     */
    Indirect = OP_CODE_LEN_F3_F4+1,
    Immediate = OP_CODE_LEN_F3_F4+2,
    Indexed = OP_CODE_LEN_F3_F4+3,
    BaseRelative = OP_CODE_LEN_F3_F4+4,
    PcRelative = OP_CODE_LEN_F3_F4+5,
    Extended = OP_CODE_LEN_F3_F4+6,
}

// TODO I'd like to use the bitflags! macro, but it's not currently
// possible as the instruction length is required

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Flags::Indirect => write!(f, "Indirect"),
            Flags::Immediate => write!(f, "Immediate"),
            Flags::Indexed => write!(f, "Indexed"),
            Flags::Extended => write!(f, "Extended"),
            Flags::BaseRelative => write!(f, "BaseRelative"),
            Flags::PcRelative => write!(f, "PcRelative"),
        }
    }
}
