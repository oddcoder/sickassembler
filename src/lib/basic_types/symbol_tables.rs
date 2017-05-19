use parking_lot::RwLock;
use symbol::Symbol;
use std::collections::{HashSet, HashMap};
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;
//use parking_lot::{RwLock, RwLockReadGuard};

lazy_static!{    
    // The master table contains all the control sections' symbol tables, it's the main
    // data structure in this module
    static ref MASTER_TABLE:RwLock<MasterTable> = RwLock::new(MasterTable::new());
    static ref LOCK_WAIT_DURATION_MILLIS: Duration = {Duration::from_millis(50)}; // Whateva
}
#[derive(Debug)]
struct MasterTable {
    mapping: HashMap<String, Box<CsectSymTab>>,
}

impl MasterTable {
    pub fn new() -> MasterTable {
        MasterTable { mapping: HashMap::new() }
    }

    /// Called when a CSECT is encountered
    pub fn define_csect(&mut self, csect: &str) {
        let table = Box::new(CsectSymTab::new(csect));
        self.mapping.insert(csect.to_owned(), table);
    }

    /// Called when a label is declared in the current code section
    pub fn define_local_symbol(&mut self, sym_name: &str, addr: i32, csect: &str) {
        let csect_tab: &mut CsectSymTab = self.get_csect_table(csect);
        csect_tab.insert_local_symbol(sym_name, addr, csect);
    }

    /// Called when an EXT REF symbol is encountered
    pub fn define_import_symbol(&mut self, sym_name: &str, csect: &str) {
        let csect_tab: &mut CsectSymTab = self.get_csect_table(csect);
        csect_tab.insert_import_symbol(sym_name);
    }

    /// Called when an EXT DEF symbol is encountered
    pub fn define_export_symbol(&mut self, sym_name: &str, csect: &str) {
        let csect_tab: &mut CsectSymTab = self.get_csect_table(csect);
        csect_tab.insert_export_symbol(sym_name);
    }

    // Finds the parent section of the given symbol name
    fn find_exported(&self, sym_name: &str) -> Result<Symbol, String> {
        for (csect, table) in &self.mapping {
            if table.has_local(sym_name) {
                return Ok(table.find_local(sym_name).unwrap());
            }
        }

        Err(format!("Symbol isn't exported anywhere"))
    }

    // This function will be called in pass2, so we can detect errors here
    pub fn resolve_label(&mut self, sym_name: &str, csect: &str) -> Result<Symbol, String> {
        // Try to find in the csect local
        let mut errs: Vec<String> = Vec::new();

        {
            // Avoid 2 mutable borrows ( here and in the below if )
            let csect_table = self.get_csect_table(csect);
            if csect_table.has_local(sym_name) {
                match csect_table.find_local(sym_name) {
                    Ok(sym) => return Ok(sym),
                    Err(e) => errs.push(e),
                };
            }
        }

        if self.get_csect_table(csect).imports(sym_name) {
            match self.find_exported(sym_name) {
                Ok(sym) => Ok(sym),
                Err(e) => {
                    errs.push(e);
                    Err(errs.join("\n"))
                }
            }
        } else {
            Err(errs.join("\n"))
        }
    }


    fn get_csect_table(&mut self, csect: &str) -> &mut CsectSymTab {
        let table = self.mapping.get_mut(csect).unwrap();
        return table;
    }
}

/// Contains the relations of EXTDEF and EXTREFS
#[derive(Debug)]
struct CsectSymTab {
    csect: String,
    local_symbols: HashMap<String, Symbol>,
    exported_symbols: HashSet<String>,
    imported_symbols: HashSet<String>,
}

impl CsectSymTab {
    fn new(csect: &str) -> CsectSymTab {
        CsectSymTab {
            csect: csect.to_owned(),
            local_symbols: HashMap::new(),
            exported_symbols: HashSet::new(),
            imported_symbols: HashSet::new(),
        }
    }

    pub fn insert_local_symbol(&mut self, sym_name: &str, sym_addr: i32, current_csect: &str) {
        let sym = Symbol::new(sym_name, sym_addr, current_csect);
        self.local_symbols.insert(sym.get_name(), sym);
    }

    pub fn insert_import_symbol(&mut self, sym_name: &str) {
        self.imported_symbols.insert(sym_name.to_owned());
    }

    // Undefined symbols will be caught when used
    // FIXME: TODO: Undefined symbols will be reported on object record generation ONLY
    // in D and R records
    pub fn insert_export_symbol(&mut self, sym_name: &str) {
        self.exported_symbols.insert(sym_name.to_owned());
    }

    fn find_local(&self, sym_name: &str) -> Result<Symbol, String> {
        match self.local_symbols.get(sym_name) {
            Some(e) => Ok(e.clone()),
            None => {
                Err(format!("Couldn't find local symbol {{ {} }} in control section {{ {} }}",
                            sym_name,
                            self.csect))
            }
        }
    }

    fn has_local(&self, sym_name: &str) -> bool {
        self.local_symbols.contains_key(sym_name)
    }

    fn exports(&self, sym_name: &str) -> bool {
        self.exported_symbols.contains(sym_name)
    }

    fn imports(&self, sym_name: &str) -> bool {
        self.imported_symbols.contains(sym_name)
    }
}

// pub fn define_control_section(csect: &str) {
//     let lock_result = MASTER_TABLE.try_write_for(*LOCK_WAIT_DURATION_MILLIS);
//     if lock_result.is_none() {
//         panic!("Lock acquisition timed out!");
//     }

//     let mut master_table = lock_result.unwrap();
//     master_table.insert(csect.to_owned(), CsectSymTab::new(csect));
// }

pub fn define_local_symbol(sym_name: &str, addr: i32, csect: &str) -> Result<(), String> {
    // let mut master_table;
    // match MASTER_TABLE.try_write_for(*LOCK_WAIT_DURATION_MILLIS) {
    //     None => return Err(format!("Control section isn't defined {{ {} }}", csect)),
    //     Some(x) => master_table = x,
    // };

    //     let csect_table = master_table.get_mut(csect).unwrap();
    //     csect_table.insert_local_symbol(sym_name, addr, csect.to_owned());

    //     Ok(())
    unimplemented!()
}

// // fn access_csect_table(csect: &str) -> Result<CsectSymTab, String> {
// //     let mut guard = MASTER_TABLE.write();
// //     let mut master_table: &mut HashMap<String, CsectSymTab> = &mut (*guard);
// //     let csect_table = master_table.get_mut(csect).unwrap();
// //     Ok(csect_table)
// // }

fn access_csect_table(csect: &str) -> Result<RefCell<CsectSymTab>, String> {
    unimplemented!()
}

// pub fn define_exported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
//     match access_csect_table(csect) {
//         Ok(mut csect_table) => csect_table.into_inner().insert_export_symbol(sym_name),
//         Err(e) => return Err(e),
//     };
//     Ok(())
// }

// pub fn define_imported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
//     match access_csect_table(csect) {
//         Ok(csect_table) => {
//             csect_table.into_inner().insert_import_symbol(sym_name);
//             Ok(())
//         }
//         Err(e) => Err(e),
//     }
// }

pub fn get_symbol(name: &str, csect: &str) -> Result<Symbol, String> {
    unimplemented!()
}

// pub fn exists(sym_name: &str, csect: &str) -> bool {
//     match access_csect_table(csect) {
//         Ok(csect_table) => csect_table.into_inner().has_local(sym_name),
//         Err(_) => false,
//     }
// }

pub fn get_all_symbols() -> HashSet<Symbol> {
    // Called to print all the variables in the program
    // TODO: get all keys in the local hashtables of the program
    unimplemented!()
}

// fn resolve_local(csect: &str, sym_name: &str) -> Result<Symbol, String> {
//     let lock_result = access_csect_table(csect);

//     if lock_result.is_err() {
//         return Err(lock_result.unwrap_err());
//     }

//     let csect_table: &mut CsectSymTab = &mut lock_result.unwrap().into_inner();
//     match csect_table.find_local(sym_name) {
//         Ok(sym) => Ok(sym),
//         Err(e) => Err(e),
//     }
// }

// fn resolve_from_imports(csect: &str, sym_name: &str) -> Result<Symbol, String> {
//     let sym: Result<Symbol, String>;
//     match access_csect_table(csect) {
//         Err(e) => return Err(e),
//         Ok(csect_table) => sym = csect_table.into_inner().find_imported(sym_name),
//     }

//     match sym {
//         Err(e) => Err(e),
//         Ok(s) => Ok(s),
//     }
// }

// #[test]
// fn local_symbol() {
//     let result = define_local_symbol("x1", 32, String::new().as_str())
//         .and_then(|_| resolve_local(String::new().as_str(), "x1"));
//     assert!(result.is_ok());
// }
