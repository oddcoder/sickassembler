/**
 * Higher modules will be declared here
 */
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate log;
extern crate parking_lot;

use std::fmt::UpperHex;
use std::marker::Sized;
use regex::Regex;

pub mod pass_one;
pub mod basic_types;
pub mod pass_two;
pub mod filehandler;
pub mod htme;
pub mod semantics_validator;
pub mod operand_parsing;
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

/// Tells whether a token is a valid label or an instruction
/// returns:
/// Err -> invalid token (Not a label nor instruction)
/// Result ->
///     - true : label
///     - false : instruction
pub fn is_label(suspect: &str) -> bool {
    return LABEL_STREAM.is_match(suspect);
}

/// A literal is a byte/chars preceeded by an '=' sign
pub fn is_literal(op: &str) -> bool {
    return op.starts_with("=") && is_ascii_or_word_operand(op.trim_left_matches("="));
}

/// An ascii operand is on the form  (C|X)'...'
pub fn is_ascii_or_word_operand(op: &str) -> bool {
    return CHAR_OPERAND_STREAM.is_match(&op) || HEX_OPERAND_STREAM.is_match(&op);
}

/// A decimal is a string of the form [0-9]+
pub fn is_decimal(op: &str) -> bool {
    return DECIMAL_STREAM.is_match(&op);
}

/// A decimal is a string of the form [[xdigit]]+
pub fn is_hex(op: &str) -> bool {
    return HEX_STREAM.is_match(&op);
}

lazy_static!{
    static ref CHAR_OPERAND_STREAM:Regex = Regex::new(r"^C'[[:alnum:]]+'$").unwrap();
    static ref HEX_OPERAND_STREAM:Regex = Regex::new(r"^X'[[:xdigit:]]+'$").unwrap();
    static ref DECIMAL_STREAM:Regex = Regex::new(r"^-?[[:digit:]]+$").unwrap();
    static ref HEX_STREAM:Regex = Regex::new(r"^[[:xdigit:]]+$").unwrap();
    static ref LABEL_STREAM:Regex = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap();
}
