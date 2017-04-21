use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};

#[derive(Debug,Clone)]
pub struct Literal {
    pub label: String, // Label chosen by the assembler
    pub value: String, // Object code value
    pub external_name: String, // Value in code, ex. C'EOF'
    pub address: u32,
    pub length_in_bytes: i32,
}

impl Literal {
    fn length_in_bytes(&self) -> u32 {
        return (self.value.len() / 2) as u32;
    }
}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        self.value == other.value
    }
}

impl Eq for Literal {}
