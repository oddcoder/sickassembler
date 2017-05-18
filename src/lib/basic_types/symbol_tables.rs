use parking_lot::{RwLock, RwLockReadGuard};
use symbol::{self, Symbol};
use std::collections::{HashSet, HashMap};
use std::time::Duration;
use std::cell::RefCell;

lazy_static!{    
    // The master table contains all the control sections' symbol tables, it's the main
    // data structure in this module
    static ref MASTER_TABLE:RwLock<HashMap<String,CsectSymTab>> = RwLock::new(HashMap::new());
    static ref LOCK_WAIT_DURATION_MILLIS: Duration = {Duration::from_millis(50)}; // Whateva
}

/// Contains the relations of EXTDEF and EXTREFS
struct CsectSymTab {
    csect: String,
    local_symbols: HashMap<String, Symbol>,
    exported_symbols: HashMap<String, Symbol>,
    imported_symbols: HashSet<String>,
}

impl CsectSymTab {
    fn new(csect: &str) -> CsectSymTab {
        CsectSymTab {
            csect: csect.to_owned(),
            local_symbols: HashMap::new(),
            exported_symbols: HashMap::new(),
            imported_symbols: HashSet::new(),
        }
    }

    pub fn insert_local_symbol(&mut self, sym_name: &str, sym_addr: i32, current_csect: String) {
        let sym = Symbol::new(sym_name, sym_addr, &current_csect);
        self.local_symbols.insert(sym.get_name(), sym.clone());
        self.update_exports(sym);
    }

    pub fn insert_import_symbol(&mut self, sym_name: &str) {
        self.imported_symbols.insert(sym_name.to_owned());
    }

    pub fn insert_export_symbol(&mut self, sym_name: &str) {
        // Undefined symbols will be caught when used
        // TODO: Undefined symbols will be reported on object record generation
        if self.local_symbols.contains_key(sym_name) {
            let sym: Symbol = self.local_symbols.get(sym_name).unwrap().clone();
            self.exported_symbols.insert(sym_name.to_owned(), sym);
        } else {
            self.exported_symbols.insert(sym_name.to_owned(), Symbol::new_uninitialized(sym_name));
        }
    }

    fn find_local(&self, sym_name: &str) -> Result<Symbol, String> {
        match self.local_symbols.get(sym_name) {
            Some(e) => Ok(e.clone()),
            None => Err(format!("Couldn't find local symbol {{ {} }}", sym_name)),
        }
    }

    fn has_local(&self, sym_name: &str) -> bool {
        match self.find_local(sym_name) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn find_imported(&self, sym_name: &str) -> Result<Symbol, String> {
        let lock_result = MASTER_TABLE.try_read_for(*LOCK_WAIT_DURATION_MILLIS);
        if lock_result.is_none() {
            panic!("Lock acquisition timed out!");
        }

        let ref master_table: HashMap<String, CsectSymTab> = *lock_result.unwrap();

        for (_, v) in master_table {
            let v: &CsectSymTab = v;
            if !v.exports(sym_name) {
                continue;
            }
            return v.find_exported(sym_name);
        }
        Err(format!("Couldn't find imported symbol {{ {} }}", sym_name))
    }

    fn find_exported(&self, sym_name: &str) -> Result<Symbol, String> {
        unimplemented!()
    }

    fn exports(&self, sym_name: &str) -> bool {
        self.exported_symbols.contains_key(sym_name)
    }

    fn update_exports(&mut self, sym: Symbol) {
        // Called when a symbol is exported then its
        // definitoin is encountered later in the control section
        if !self.exported_symbols.contains_key(&sym.get_name()) {
            return;
        }

        self.exported_symbols.insert(sym.get_name(), sym);
    }
}

pub fn define_control_section(csect: &str) {
    let lock_result = MASTER_TABLE.try_write_for(*LOCK_WAIT_DURATION_MILLIS);
    if lock_result.is_none() {
        panic!("Lock acquisition timed out!");
    }

    let mut master_table = lock_result.unwrap();
    master_table.insert(csect.to_owned(), CsectSymTab::new(csect));
}

pub fn define_local_symbol(sym_name: &str, addr: i32, csect: &str) -> Result<(), String> {
    let mut master_table;
    match MASTER_TABLE.try_write_for(*LOCK_WAIT_DURATION_MILLIS) {
        None => return Err(format!("Control section isn't defined {{ {} }}", csect)),
        Some(x) => master_table = x,
    };

    let csect_table = master_table.get_mut(csect).unwrap();
    csect_table.insert_local_symbol(sym_name, addr, csect.to_owned());

    Ok(())
}

fn access_csect_table_for_insert(csect: &str) -> Result<CsectSymTab, String> {
    let mut guard = MASTER_TABLE.write();
    let mut master_table: &mut HashMap<String, CsectSymTab> = &mut (*guard);
    let csect_table = master_table.get_mut(csect).unwrap();
    //csect_table

    unimplemented!()
}

pub fn define_exported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
    match access_csect_table_for_insert(csect) {
        Ok(mut csect_table) => csect_table.insert_export_symbol(sym_name),
        Err(e) => return Err(e),
    };
    Ok(())
}

pub fn define_imported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
    match access_csect_table_for_insert(csect) {
        Ok(mut csect_table) => Ok(csect_table.insert_import_symbol(sym_name)),
        Err(e) => Err(e),
    }
}

pub fn get_symbol(name: &str, csect: &str) -> Result<Symbol, String> {
    let result: Result<CsectSymTab, String> = access_csect_table_for_insert(csect);
    if result.is_err() {
        return Err(result.err().unwrap());
    }

    let csect_table: CsectSymTab = result.unwrap();
    let mut errs: Vec<String> = Vec::new();
    let result = csect_table.find_local(name)
        .or_else(|e| {
            errs.push(e);
            csect_table.find_imported(name)
        })
        .or_else(|e| {
            errs.push(e);
            Err(())
        });

    match result {
        Ok(sym) => Ok(sym),
        Err(_) => Err(errs.join("\n")),
    }
}

pub fn exists(sym_name: &str, csect: &str) -> bool {
    // match access_csect_table(csect) {
    //     Ok(csect_table) => csect_table.has_local(sym_name),
    //     Err(_) => false,
    // }
    unimplemented!()
}

pub fn get_all_symbols() -> HashSet<Symbol> {
    // Called to print all the variables in the program
    // TODO: get all keys in the local hashtables of the program
    unimplemented!()
}

fn resolve_local(csect: &str, sym_name: &str) -> Result<Symbol, String> {
    // let lock_result = MASTER_TABLE.try_write_for(*LOCK_WAIT_DURATION_MILLIS);

    // if lock_result.is_none() {
    //     panic!("Lock acquisition timed out!");
    // }

    // let mut csect_table = lock_result.unwrap().get(csect).unwrap();

    // match csect_table.find_local(sym_name) {
    //     Some(sym) => Ok(sym),
    //     None => {
    //         Err(format!("Couldn't find local symbol {{ {} }} in control section {{ {} }}",
    //                     sym_name,
    //                     csect))
    //     }
    // }
    unimplemented!()
}

fn resolve_from_imports(csect: &str, sym_name: &str) -> Result<Symbol, String> {
    // let lock_result = MASTER_TABLE.try_write_for(*LOCK_WAIT_DURATION_MILLIS);

    // if lock_result.is_none() {
    //     panic!("Lock acquisition timed out!");
    // }
    // let mut csect_table = lock_result.unwrap().get(csect).unwrap();

    unimplemented!()
}

#[test]
fn local_symbol() {
    let result = define_local_symbol("x1", 32, String::new().as_str())
        .and_then(|_| resolve_local(String::new().as_str(), "x1"));
    assert!(result.is_ok());
}
