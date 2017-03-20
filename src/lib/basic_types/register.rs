#![allow(dead_code)]

/**
 * SIC/XE registers
 */
#[derive(Debug,PartialEq)]
pub enum Register {
    A = 0,
    X = 1,
    L = 2,
    B = 3,
    S = 4,
    T = 5,
    F = 6,
}
