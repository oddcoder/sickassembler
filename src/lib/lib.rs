/**
 * Higher modules will be declared here
 */
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;

pub mod basic_types;
pub mod pass_two;
pub mod filehandler;
pub mod htme;

pub mod semantics_validator;

use std::fmt::UpperHex;
use std::marker::Sized;
pub fn to_hex<T>(num: T) -> String
    where T: UpperHex + Sized
{
    format!("{:X}", num)
}
