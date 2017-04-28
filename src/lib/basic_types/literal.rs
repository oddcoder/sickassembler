use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};
use std::f32;

#[derive(Debug,Clone)]
pub struct Literal {
    pub label: String, // Label chosen by the assembler
    pub value: String, // Object code value
    pub external_name: String, // Value in code, ex. =C'EOF'
    pub address: u32,
}

impl Literal {
    pub fn new(label: String, value: String, ext_name: String, addr: u32) -> Literal {
        Literal {
            label: label,
            value: value,
            external_name: ext_name,
            address: addr,
        }
    }

    pub fn length_in_bytes(&self) -> i32 {
        let len: f32 = self.value.len() as f32 / 2.0;
        return (len.ceil()) as i32;
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
