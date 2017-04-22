/**
 * Higher modules will be declared here
 */
use std::fmt::UpperHex;
use std::marker::Sized;

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

// Re-exports
pub use htme::record_string::string_from_object_code;
pub use htme::raw_program::RawProgram;

// Re-export sub modules, to make imports neater
pub use basic_types::instruction;
pub use basic_types::formats;
pub use basic_types::operands;
pub use basic_types::flags;
pub use basic_types::register;
pub use basic_types::instruction_set;
pub use basic_types::unit_or_pair;
pub use basic_types::literal_table;
pub use basic_types::literal;
pub use basic_types::base_table;

pub use pass_two::translator;

pub fn to_hex<T>(num: T) -> String
    where T: UpperHex + Sized
{
    format!("{:X}", num)
}
