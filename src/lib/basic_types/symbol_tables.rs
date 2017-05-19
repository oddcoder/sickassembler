use parking_lot::Mutex;
use symbol::{Symbol, SymbolType};
use std::collections::{HashSet, HashMap};
use std::time::Duration;

const LOCK_DURATION_MILLIS: u64 = 50;

lazy_static!{    
    // The master table contains all the control sections' symbol tables, it's the main
    // data structure in this module
    static ref MASTER_TABLE:Mutex<MasterTable> = Mutex::new(MasterTable::new());
    static ref LOCK_DURATION: Duration = {Duration::from_millis(LOCK_DURATION_MILLIS)};
}

#[derive(Debug)]
pub struct TableResult {
    pub symbol: Symbol,
    pub symbol_type: SymbolType,
}

impl TableResult {
    fn new(symbol: Symbol, sym_type: SymbolType) -> TableResult {
        TableResult {
            symbol: symbol,
            symbol_type: sym_type,
        }
    }

    pub fn get_address(&self) -> i32 {
        self.symbol.get_address()
    }

    pub fn get_control_section(&self) -> String {
        self.symbol.get_control_section()
    }

    pub fn get_name(&self) -> String {
        self.symbol.get_name()
    }
}

#[derive(Debug)]
pub struct MasterTable {
    mapping: HashMap<String, Box<CsectSymTab>>,
}

impl MasterTable {
    fn new() -> MasterTable {
        let mut table = MasterTable { mapping: HashMap::new() };
        table.define_csect(&String::new()).unwrap(); // Define the default section
        assert!(table.has_csect(&String::new()));
        table
    }

    /// Called when a CSECT is encountered
    fn define_csect(&mut self, csect: &str) -> Result<(), String> {

        if self.has_csect(csect) {
            return Err(format!("Redifinition of control section {{ {} }} ", csect));
        }

        let table = Box::new(CsectSymTab::new(csect));
        let mapping = self.mapping.insert(csect.to_owned(), table);

        assert!(mapping.is_none());
        Ok(())
    }

    /// Called when a label is declared in the current code section
    fn define_local_symbol(&mut self,
                           sym_name: &str,
                           addr: i32,
                           csect: &str)
                           -> Result<(), String> {
        let csect_tab: &mut CsectSymTab = self.get_csect_table_write(csect);

        if csect_tab.has_local(sym_name) {
            return Err(format!("Redifinition of label {{ {} }}", sym_name));
        }

        let sym: Symbol = Symbol::new(sym_name, addr, csect);
        csect_tab.insert_local_symbol(sym);

        Ok(())
    }

    /// Called when an EXT REF symbol is encountered
    fn define_import_symbol(&mut self, sym_name: &str, csect: &str) -> Result<(), String> {
        let csect_tab: &mut CsectSymTab = self.get_csect_table_write(csect);

        if csect_tab.imports(csect) {
            return Err(format!("Multiple imports of {{ {} }} in {{ {} }}", sym_name, csect));
        }

        csect_tab.insert_import_symbol(sym_name);

        Ok(())
    }

    /// Called when an EXT DEF symbol is encountered
    fn define_export_symbol(&mut self, sym_name: &str, csect: &str) -> Result<(), String> {
        let csect_tab: &mut CsectSymTab = self.get_csect_table_write(csect);

        if csect_tab.exports(sym_name) {
            return Err(format!("Multiple exports of {{ {} }} in {{ {} }}", sym_name, csect));
        }

        csect_tab.insert_export_symbol(sym_name);

        Ok(())
    }

    /// This function will be called in pass2, so we can detect errors here
    fn resolve_label(&mut self, sym_name: &str, csect: &str) -> Result<TableResult, String> {
        // Try to find in the csect local
        let mut errs: Vec<String> = Vec::new();
        let csect_table = self.get_csect_table_read(csect);

        match csect_table.find_local(sym_name) {
            Ok(sym) => return Ok(TableResult::new(sym.clone(), SymbolType::Local)),
            Err(e) => errs.push(e),
        };

        match self.resolve_imported(sym_name, csect) {
            Ok(sym) => return Ok(TableResult::new(sym.clone(), SymbolType::Imported)),
            Err(e) => errs.push(e),
        }

        Err(errs.join("\n"))
    }

    /// Finds the parent section of the given symbol name
    fn resolve_exported(&self, sym_name: &str) -> Result<&Symbol, String> {
        for (_, table) in &self.mapping {
            let table: &CsectSymTab = table;
            if table.has_local(sym_name) {
                return Ok(table.find_local(sym_name).unwrap());
            }
        }

        Err(format!("Symbol isn't exported anywhere"))
    }

    /// Resolves an externally defined symbol for a given control section
    fn resolve_imported(&self, sym_name: &str, csect: &str) -> Result<&Symbol, String> {
        if !self.get_csect_table_read(csect).imports(sym_name) {
            return Err(format!("{{ {} }} isn't imported", sym_name));
        }

        match self.resolve_exported(sym_name) {
            Ok(sym) => Ok(sym),
            Err(e) => Err(e),
        }
    }

    fn get_all_symbols(&self) -> HashSet<Symbol> {
        let mut result: HashSet<Symbol> = HashSet::new();
        for (_, table) in &self.mapping {
            let table: &CsectSymTab = table;
            table.local_symbols
                .iter()
                .map(|(_, sym)| result.insert(sym.clone()))
                .collect::<Vec<bool>>();
        }
        result
    }

    /// Used before accesing a symbol table
    fn has_csect(&self, csect: &str) -> bool {
        self.mapping.contains_key(csect)
    }

    /// Returns the symbol table of the given control section for editing
    fn get_csect_table_write(&mut self, csect: &str) -> &mut CsectSymTab {
        // println!("Sect: \"{}\" Table:\n {:#?}", csect, self.mapping);
        let table = self.mapping.get_mut(csect).unwrap();
        return table;
    }

    /// Returns a read-only view of the symbol table of the given control section
    fn get_csect_table_read(&self, csect: &str) -> &CsectSymTab {
        let table = self.mapping.get(csect).unwrap();
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

    fn insert_local_symbol(&mut self, sym: Symbol) {
        self.local_symbols.insert(sym.get_name(), sym);
    }

    fn insert_import_symbol(&mut self, sym_name: &str) {
        // Note: if a symbol isn't defined as a local and it's imported, this
        // should be discovered by object code generation phase, meaning; this
        // function is annotative and not descriptive
        self.imported_symbols.insert(sym_name.to_owned());
    }

    fn insert_export_symbol(&mut self, sym_name: &str) {
        self.exported_symbols.insert(sym_name.to_owned());
    }

    fn find_local(&self, sym_name: &str) -> Result<&Symbol, String> {
        if !self.has_local(sym_name) {
            return Err(format!("{{ {} }} Isn't a local symbol for {{ {} }}",
                               sym_name,
                               self.csect));
        }
        Ok(self.local_symbols.get(sym_name).unwrap())
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

pub fn define_control_section(csect: &str) -> Result<(), String> {
    let ref mut master_table: MasterTable = *MASTER_TABLE.try_lock_for(*LOCK_DURATION).unwrap();
    master_table.define_csect(csect)
}

pub fn define_local_symbol(sym_name: &str, addr: i32, csect: &str) -> Result<(), String> {
    let ref mut master_table: MasterTable = *MASTER_TABLE.try_lock_for(*LOCK_DURATION).unwrap();
    master_table.define_local_symbol(sym_name, addr, csect)
}


pub fn define_exported_symbols(symbols: &Vec<&str>, csect: &str) -> Result<(), String> {
    let mut errs: Vec<String> = Vec::new();

    let result = symbols.iter()
        .map(|item| {
            define_exported_symbol(item, csect).or_else(|e| {
                errs.push(e);
                Err(())
            })
        })
        .all(|r| r.is_ok());
    if result { Ok(()) } else { Err(errs.join("\t")) }
}


pub fn define_exported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
    let ref mut master_table: MasterTable = *MASTER_TABLE.try_lock_for(*LOCK_DURATION).unwrap();
    master_table.define_export_symbol(sym_name, csect)
}

pub fn define_imported_symbols(symbols: &Vec<&str>, csect: &str) -> Result<(), String> {
    let mut errs: Vec<String> = Vec::new();

    let result = symbols.iter()
        .map(|item| {
            define_imported_symbol(item, csect).or_else(|e| {
                errs.push(e);
                Err(())
            })
        })
        .all(|r| r.is_ok());
    if result { Ok(()) } else { Err(errs.join("\t")) }
}


pub fn define_imported_symbol(sym_name: &str, csect: &str) -> Result<(), String> {
    let ref mut master_table: MasterTable = *MASTER_TABLE.try_lock_for(*LOCK_DURATION).unwrap();
    master_table.define_import_symbol(sym_name, csect)
}

pub fn get_symbol(sym_name: &str, csect: &str) -> Result<TableResult, String> {
    let ref mut master_table: MasterTable = *MASTER_TABLE.try_lock_for(*LOCK_DURATION).unwrap();
    match master_table.resolve_label(sym_name, csect) {
        Ok(table_res) => Ok(table_res),
        Err(e) => Err(e),
    }
}

pub fn get_all_symbols() -> HashSet<Symbol> {
    let ref mut master_table: MasterTable = *MASTER_TABLE.try_lock_for(*LOCK_DURATION).unwrap();
    master_table.get_all_symbols()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DEFAULT_CONTROL_SECTION: &str = "";
    #[test]
    fn simple_local_symbol() {
        let (name, csect) = create_local_variable("X1", DEFAULT_CONTROL_SECTION);
        let sym = get_symbol(name, csect).unwrap();
        check_var(name, csect, sym);
    }

    #[test]
    fn simple_export() {
        let (name, csect) = create_local_variable("X2-0", DEFAULT_CONTROL_SECTION);
        define_exported_symbol(name, csect).unwrap();

        let ext_csect = "csect2";
        define_control_section(ext_csect).unwrap();
        define_imported_symbol(name, ext_csect).unwrap();

        let sym = get_symbol(name, ext_csect).unwrap();
        check_var(&name, &csect, sym);
    }

    #[test]
    #[should_panic]
    fn simple_failing_export() {
        let (name, csect) = ("X2", DEFAULT_CONTROL_SECTION); // Undecalred variable
        define_exported_symbol(name, csect).unwrap();

        let ext_csect = "csect2";
        define_control_section(ext_csect).unwrap();
        define_imported_symbol(name, ext_csect).unwrap();

        let sym = get_symbol(name, ext_csect).unwrap();
        check_var(&name, &csect, sym);
    }

    #[test]
    fn simple_export_again() {
        let (name, csect) = create_local_variable("X2-1", "TROL_SECTION");

        define_exported_symbol(name, csect).unwrap();

        let ext_csect = "csect2-1";
        define_control_section(ext_csect).unwrap();
        define_imported_symbol(name, ext_csect).unwrap();

        let sym = get_symbol(name, ext_csect).unwrap();
        check_var(&name, &csect, sym);
    }

    fn check_var(expected_name: &str, expected_csect: &str, found: TableResult) {
        assert!(expected_csect == found.get_control_section());
        assert!(expected_name == found.get_name());
    }

    fn create_local_variable<'a>(name: &'a str, csect: &'a str) -> (&'a str, &'a str) {
        if !csect.is_empty() {
            define_control_section(csect).unwrap();
        }
        define_local_symbol(name, 0, csect).unwrap();
        (name, csect)
    }

    fn create_local_variable_with_addr(name: &str, addr: i32, csect: &str) -> (String, String) {
        if !csect.is_empty() {
            define_control_section(csect).unwrap();
        }
        define_local_symbol(name, addr, csect).unwrap();
        (name.to_owned(), csect.to_owned())
    }
}
