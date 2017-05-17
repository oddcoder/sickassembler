use parking_lot::RwLock;
use symbol::Symbol;
use std::collections::{HashSet, HashMap};

lazy_static!{    
    static ref SYMBOL_TABLE:RwLock<HashMap<String,CsectSymTab>> = RwLock::new(HashMap::new());
}
/// Contains the relations of EXTDEF and EXTREFS

struct CsectSymTab {
    csect: String,
    local_symbols: HashMap<String, Symbol>,
    exported_symbols: HashMap<String, Symbol>,
    imported_symbols: HashSet<String>,
}

impl CsectSymTab {
    fn new(csect: String) -> CsectSymTab {
        CsectSymTab {
            csect: csect,
            local_symbols: HashMap::new(),
            exported_symbols: HashMap::new(),
            imported_symbols: HashSet::new(),
        }
    }

    pub fn insert_local_symbol(&mut self, sym_name: &str, sym_addr: i32, current_csect: String) {
        let sym = Symbol::new(sym_name.to_owned(), sym_addr, current_csect);
        self.local_symbols.insert(sym.get_name(), sym);
        self.update_exports(sym);
    }

    pub fn insert_import_symbol(&mut self, sym_name: &str) {
        self.imported_symbols.insert(sym_name.to_owned());
    }

    pub fn insert_export_symbol(&mut self, sym_name: &str) {
        // Undefined symbols will be caught when used
        // TODO: Undefined symbols will be reported on object record generation
        if self.local_symbols.contains_key(sym_name) {
            let sym: &Symbol = self.local_symbols.get(sym_name).unwrap();
            self.exported_symbols.insert(sym_name.to_owned(), sym.clone());
        } else {
            self.exported_symbols.insert(sym_name.to_owned(),
                                         Symbol::new_uninitialized(sym_name.to_owned()));
        }
    }

    fn find_local(&self, sym_name: &str) -> Option<Symbol> {
        match self.local_symbols.get(sym_name) {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }

    fn find_imported(&self, sym_name: &str) -> Option<Symbol> {
        let master_table: HashMap<String, CsectSymTab> = SYMBOL_TABLE.read();

        for (_, v) in &master_table {
            let v: CsectSymTab = v;
            if !v.exports(sym_name) {
                continue;
            }
            return v.find_exported(sym_name);
        }
    }

    fn find_exported(&self, sym_name: String) -> Option<Symbol> {
        unimplemented!()
    }

    fn exports(&self, sym_name: &str) -> bool {
        self.exported_symbols.contains_key(sym_name)
    }

    fn update_exports(&mut self, sym: Symbol) {
        // Called when a symbol is exported then its
        // definitoin is encountered later in the control section
        if !self.exported_symbols.contains_key(sym.get_name().as_str()) {
            return;
        }

        self.exported_symbols.insert(sym.get_name(), sym);
    }
}

pub fn define_local_symbol(sym_name: &str, addr: i32, csect: &str) -> Result<(), String> {
    let csect_table = access_csect_table(csect);

    if let Err(e) = csect_table {
        return Err(e);
    }

    let csect_table: &CsectSymTab = csect_table.unwrap();
    csect_table.insert_local_symbol(sym_name, addr, csect.to_owned());

    Ok(())
}

pub fn define_exported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
    let csect_table = access_csect_table(csect);

    if let Err(e) = csect_table {
        return Err(e);
    }

    let csect_table: &CsectSymTab = csect_table.unwrap();
    csect_table.insert_export_symbol(sym_name);

    Ok(())
}

pub fn define_imported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
    let csect_table = access_csect_table(csect);

    if let Err(e) = csect_table {
        return Err(e);
    }

    let csect_table: &CsectSymTab = csect_table.unwrap();
    csect_table.insert_import_symbol(sym_name);

    Ok(())
}

pub fn resolve_symbol(name: &str, csect: &str) -> Symbol {
    unimplemented!()
}

fn resolve_local(csect: &str, sym_name: &str) -> Result<Symbol, String> {
    let csect_table = access_csect_table(csect);

    if let Err(e) = csect_table {
        return Err(e);
    }
    let csect_table: &CsectSymTab = csect_table.unwrap();

    match csect_table.find_local(sym_name) {
        Some(sym) => Ok(sym),
        None => {
            Err(format!("Couldn't find local symbol {{ {} }} in control section {{ {} }}",
                        sym_name,
                        csect))
        }
    }
}

fn resolve_from_imports(csect: &str, sym_name: &str) -> Result<Symbol, String> {
    let csect_table = access_csect_table(csect);

    if let Err(e) = csect_table {
        return Err(e);
    }
    let csect_table: &CsectSymTab = csect_table.unwrap();
}

fn access_csect_table(csect: &str) -> Result<&CsectSymTab, String> {
    let master_table: HashMap<String, CsectSymTab> = *SYMBOL_TABLE.read();
    if master_table.contains_key(csect) == false {
        return Err(format!("Undefined control section {{ {} }}", csect));
    }
    Ok(master_table.get(csect).unwrap())
}
