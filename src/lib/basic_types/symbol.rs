use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};

pub const DEFAULT_CSECT: String = "#DEFUALT#".to_owned();

#[derive(Debug)]
pub struct Symbol {
    name: String,
    address: i32,
    control_section: String,
}

impl Symbol {
    pub fn new(name: String, addr: i32, csect: String) -> Symbol {
        let mut csect = csect;
        if csect.is_empty() {
            csect = DEFAULT_CSECT;
        }
        Symbol {
            name: name,
            address: addr,
            control_section: csect,
        }
    }

    pub fn new_uninitialized(name: String) -> Symbol {
        Symbol {
            name: name,
            address: -1,
            control_section: DEFAULT_CSECT,
        }
    }

    pub fn set_address(&mut self) {}

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
        Symbol::new(self.name.clone(),
                    self.address,
                    self.control_section.clone())
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