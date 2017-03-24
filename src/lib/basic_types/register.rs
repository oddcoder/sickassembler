#![allow(dead_code)]

/**
 * SIC/XE registers
 */
<<<<<<< Updated upstream
#[derive(Debug,PartialEq)]
=======
#[derive(Debug,PartialEq,Clone,Copy)]
#[repr(u8)]
>>>>>>> Stashed changes
pub enum Register {
    A = 0,
    X = 1,
    L = 2,
    B = 3,
    S = 4,
    T = 5,
    F = 6,
}
