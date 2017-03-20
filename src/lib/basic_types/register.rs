#![allow(dead_code)]

/**
 * SIC/XE registers
 */
<<<<<<< a4e007a4506e0f770ef11b17c9d529aa07e7b0d1
#[derive(Debug,PartialEq,Clone)]
=======
#[derive(Debug,PartialEq)]
>>>>>>> Pass 2 preparation
pub enum Register {
    A = 0,
    X = 1,
    L = 2,
    B = 3,
    S = 4,
    T = 5,
    F = 6,
}
