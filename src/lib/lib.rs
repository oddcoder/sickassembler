/**
 * Higher modules will be declared here
 */
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;

pub mod pass_one;
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

use basic_types::formats::{Format, get_bit_count};
pub fn to_hex_padded<T>(num: T, format: Format) -> String
    where T: UpperHex + Sized
{
    let mut s: String = format!("{:X}", num);

    // Pad with zeros if necessary, division by 4 -> to get the number number of hex chars
    let pad_count = (get_bit_count(format) / 4) - s.len() as i32;

    if pad_count == 0 {
        return s;
    }

    s = vec!["0";pad_count as usize].join("") + &s;
    s
}
