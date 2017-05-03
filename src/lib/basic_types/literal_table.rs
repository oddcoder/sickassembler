use std::collections::HashSet;
use std::iter::Iterator;
use parking_lot::RwLock;
use std::ops::DerefMut;
use pass_two::operand_translator::translate_literal;
use literal::Literal;
use regex::RegexSet;

lazy_static!{
    /// HashMap -> Keep the stored values unique
    static ref LITERAL_TABLE: RwLock<HashSet<Literal>> = RwLock::new(HashSet::new());
    static ref TEMP_LITERALS: RwLock<HashSet<String>> = RwLock::new(HashSet::new());    
    static ref LIT_ID: RwLock<u32> = RwLock::new(0);
    
    static ref LIT_REGEX:RegexSet = RegexSet::new(&[r"^=C'[[:alnum:]]+'$",
                                                    r"^=X'[[:xdigit:]]+'$"]).unwrap();
}

pub fn insert_literal(literal: &String, address: u32) {

    let mut temp = LIT_ID.write();
    let mut lit_table = LITERAL_TABLE.write();

    let mut literal_id: &mut u32 = temp.deref_mut();
    let lit_val = translate_literal(&literal[1..]); // Don't translate using the = sign

    let lit: Literal = Literal::new(("lit_".to_owned() + &literal_id.to_string()),
                                    lit_val,
                                    literal.clone(),
                                    address);

    *literal_id = *literal_id + 1;


    // Ignore the error if insertion returns None,
    // this means that the literal existed in the table
    lit_table.insert(lit);

}

/// Insert a literal name to the temp literal table
pub fn insert_unresolved(literal_name: &str) {
    TEMP_LITERALS.write().insert(literal_name.to_owned());
}

/// Called when encountering LTORG or end of file
pub fn get_unresolved() -> Vec<String> {
    let ret: Vec<String> =
        TEMP_LITERALS.read().iter().map(|li| li.to_owned()).collect::<Vec<String>>();
    TEMP_LITERALS.write().clear();
    ret
}

pub fn get_literal(name: &str) -> Option<Literal> {

    let val: String = translate_literal(&name[1..]); // Remove the = sign
    let table = LITERAL_TABLE.read();

    for lit in table.iter() {
        if *lit.value == val || lit.label == *name {
            return Some(lit.clone());
        }
    }
    None
}

pub fn is_literal(st: &str) -> bool {
    LIT_REGEX.is_match(st)
}

#[test]
fn add_get_literal() {
    insert_unresolved(&"=C'EOF'".to_owned());

    for s in get_unresolved().iter() {
        insert_literal(&s.clone(), 45);
    }

    assert!(get_literal(&"=C'EOF'".to_owned()).is_some());
    assert!(get_literal(&"=X'454F46'".to_owned()).is_some());
}
