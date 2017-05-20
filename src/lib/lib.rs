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
pub use basic_types::symbol;
pub use basic_types::symbol_tables;

pub fn to_hex_string<T>(num: T) -> String
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
    return LABEL_STREAM.is_match(suspect); // TODO: && suspect.len() < 7; ?
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

pub fn is_expression(op: &str) -> bool {
    return EXPRESSION.is_match(&op);
}

/// Removes the container of a WORD/BYTE oeprand, the prefix, the '
/// X'asdas' -> asdas ,and so on
/// This doesn't take a reference just to save copying the string
fn remove_literal_container(byte_operand: &mut String) {
    byte_operand.remove(0);
    byte_operand.remove(0);
    byte_operand.pop();
}

lazy_static!{
    // Regex reference: http://kbknapp.github.io/doapi-rs/docs/regex/index.html
    static ref CHAR_OPERAND_STREAM:Regex = Regex::new(r"^(C|c)'[[:alnum:]]+'$").unwrap();
    static ref HEX_OPERAND_STREAM:Regex = Regex::new(r"^(X|x)'[[:xdigit:]]+'$").unwrap();
    static ref DECIMAL_STREAM:Regex = Regex::new(r"^-?[[:digit:]]+$").unwrap();
    static ref HEX_STREAM:Regex = Regex::new(r"^[[:xdigit:]]+$").unwrap();
    static ref LABEL_STREAM:Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z_0-9]*$").unwrap();
    static ref COMMENT_REGEX:Regex = Regex::new(r"\.(\s|.)+").unwrap();
    static ref COMMA_WHITESPACE_REGEX:Regex = Regex::new(r",\s+").unwrap();
    static ref SPLIT_SOURCE_LINE_SPLIT_REGEX:Regex = Regex::new(r"(\s+|\n+|\t+)").unwrap();
    pub static ref EXPRESSION:Regex = Regex::new(r"^-?([a-zA-Z_][a-zA-Z_0-9]*)(?:(?:\+|-)([a-zA-Z_][a-zA-Z_0-9]*))*$").unwrap();
}
