use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};

#[derive(Debug,PartialEq,Eq)]
pub enum SymbolType {
    Local,
    Imported,
    None,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    address: i32,
    control_section: String,
    is_relative: bool,
}

impl Symbol {
    pub fn new(name: &str, addr: i32, csect: &str) -> Symbol {
        let mut csect = csect.to_owned();
        if csect.is_empty() {
            csect = String::new();
        }
        Symbol {
            name: name.to_owned(),
            address: addr,
            control_section: csect,
            is_relative: false,
        }
    }

    pub fn mark_relative(&mut self) {
        self.is_relative = true
    }

    pub fn set_address(&mut self, addr: i32) {
        self.address = addr
    }

    pub fn get_address(&self) -> i32 {
        self.address
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_control_section(&self) -> String {
        self.control_section.clone()
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Symbol {
        Symbol::new(&self.name, self.address, &self.control_section)
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash with both control section and name
        self.name.hash(state);
        self.control_section.hash(state);
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.name == other.name && self.control_section == other.control_section
    }
}
impl Eq for Symbol {}
