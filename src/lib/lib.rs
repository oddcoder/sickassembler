/**
 * Higher modules will be declared here
 */
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;
extern crate parking_lot;

pub mod pass_one;
pub mod basic_types;
pub mod pass_two;
pub mod filehandler;
pub mod htme;
pub mod semantics_validator;
pub use htme::record_string::string_from_object_code;
use std::fmt::UpperHex;
use std::marker::Sized;
pub fn to_hex<T>(num: T) -> String
    where T: UpperHex + Sized
{
    format!("{:X}", num)
}
