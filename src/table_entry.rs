#![allow(dead_code)]

/**
 * Represents an entry in the symbol table / literal table
 *
 * An entry will simply have the label, value, address
 */
struct TableEntry {
    address: u32,
    lebel: String,
    value: i32,
}
