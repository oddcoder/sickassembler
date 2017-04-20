/// When translator meets a base at a given
/// location couter (xtr) it'll ask the state
/// if a base is available for xtr

use std::u32;
use parking_lot::RwLock;

/// Keeps track of the state
lazy_static!{
    static ref BASE_VEC:RwLock<Vec<Range>> = RwLock::new(Vec::new());
}

pub fn get_base_at(locctr: u32) -> Option<u32> {
    // Return the available base at locctr
    for base in BASE_VEC.read().iter() {
        println!("{:?}", base);
        if base.start < locctr && base.end > locctr {
            return Some(base.value);
        }
    }
    println!("{:?}", locctr);
    None
}

// Gets the last item in the basevec
// and check its end, if it has no end (MAX_VAL)
// set its end and insert a new base entry at
// locctr
pub fn set_base(locctr: u32, value: u32) {
    update_last(locctr);
    let last_base = Range::new(locctr, value);
    {
        println!("PUSH BASE {:?}", last_base);
        BASE_VEC.write().push(last_base);
    }

}

// Gets the last item in the basevec
// and check its end, if it has an end (NOT MAX_VAL)
// panic/indicate error, as this will be NO BASE called
// twice
pub fn end_base(locctr: u32) {
    if update_last(locctr) == false {
        println!("No base encountered twice in a row");
        panic!("No base encountered twice in a row");
    }
}

/// Update the last base entry with the location counter
/// if it had the default ending value
fn update_last(locctr: u32) -> bool {
    let mut bases = BASE_VEC.write();

    if let Some(mut base) = bases.pop() {
        let mut is_success = false;

        if base.end == u32::max_value() {
            base.end = locctr;
            is_success = true;
        }

        // Restore the popped value
        bases.push(base);
        return is_success;
    }
    return true;
}


#[derive(Debug,Clone,Copy)]
struct Range {
    start: u32,
    end: u32,
    value: u32,
}
impl Range {
    fn new(start_loc: u32, value: u32) -> Range {
        Range {
            start: start_loc,
            end: u32::max_value(),
            value: value,
        }
    }
}


/// Note: tests run on multiple threads, as the BASE_VEC is
/// static, test results won't be realistic if the
/// test was split in many functions
#[test]
pub fn test_base() {
    set_base(0, 12);
    end_base(25);
    set_base(30, 31); // Endless base

    assert_eq!(get_base_at(14).unwrap(), 12);
    assert!(get_base_at(26).is_none());
    assert_eq!(get_base_at(35).is_some(), true);
}
#[test]
#[should_panic]
fn double_end_base() {
    set_base(0, 12);
    end_base(25);
    end_base(26);
}
