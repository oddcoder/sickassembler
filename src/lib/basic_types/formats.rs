use std::fmt;

#[derive(Debug,PartialEq,Copy,Clone)]
#[repr(u8)]
pub enum Format {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    None,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_bit_count(format: Format) -> i32 {
    (format as i32) * 8
}
