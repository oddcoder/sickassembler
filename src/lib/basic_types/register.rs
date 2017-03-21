/**
 * SIC/XE registers
 */
<<<<<<< ead3018a2ce231ba32b1b3cf132be66c9b936cc2
#[derive(Debug,PartialEq,Clone,Copy)]
#[repr(u8)]
=======
#[derive(Debug,PartialEq)]
>>>>>>> Pass2 instruction translation (#4)
pub enum Register {
    A = 0,
    X = 1,
    L = 2,
    B = 3,
    S = 4,
    T = 5,
    F = 6,
}
