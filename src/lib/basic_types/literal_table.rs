use std::collections::HashSet;
use std::iter::Iterator;
use parking_lot::RwLock;
use std::ops::DerefMut;
use pass_two::translator::translate_literal;
use basic_types::literal::Literal;

lazy_static!{
    static ref LITERAL_TABLE: RwLock<HashSet<Literal>> = RwLock::new(HashSet::new());
    static ref TEMP_LITERALS: RwLock<HashSet<String>> = RwLock::new(HashSet::new());    
    static ref LIT_ID: RwLock<u32> = RwLock::new(0);
}

pub fn insert_literal(literal: String, address: u32) {
    let lit: Literal;
    {
        let mut temp = LIT_ID.write();
        let mut literal_id: &mut u32 = temp.deref_mut();
        lit = Literal {
            name: "lit_".to_owned() + &literal_id.to_string(),
            value: translate_literal(&literal),
            external_name: literal,
            address: address,
        };
        *literal_id = *literal_id + 1;
    }

    {
        LITERAL_TABLE.write().insert(lit);
    }
}

/// Insert a literal name to the temp literal table
pub fn insert_unresolved(literal_name: String) {
    TEMP_LITERALS.write().insert(literal_name);
}

/// Called when encountering LTORG or end of file
pub fn get_unresolved() -> Vec<String> {
    let ret: Vec<String> =
        TEMP_LITERALS.read().iter().map(|li| li.to_owned()).collect::<Vec<String>>();
    TEMP_LITERALS.write().clear();
    ret
}

pub fn get_literal(name: &String) -> Option<Literal> {
    let val: String;
    {
        val = translate_literal(name);
    }

    let table = LITERAL_TABLE.read();

    for lit in table.iter() {
        if *lit.value == val || lit.name == *name {}
        return Some(lit.clone());
    }
    None
}

#[test]
fn add_get_literal() {
    insert_unresolved("C'EOF'".to_owned());

    for s in get_unresolved().iter() {
        insert_literal(s.to_owned(), 45);
    }

    assert!(get_literal(&"C'EOF'".to_owned()).is_some());
    assert!(get_literal(&"X'454F46'".to_owned()).is_some());
}
