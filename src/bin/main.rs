#[macro_use]
extern crate prettytable;
extern crate sick_lib;
extern crate getopts;
extern crate env_logger;
extern crate term;

use term::{Attr, color};
use getopts::Options;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

//use instruction::Instruction;
//use operands::OperandType;
use sick_lib::filehandler::FileHandler;
use std::env;
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options] file", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    env_logger::init().unwrap();
    // credits goes to here:-
    // https://doc.rust-lang.org/getopts/getopts/index.html
    // Time will come where I will fully understant this!
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "name");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut output = "a.out".to_string();
    if matches.opt_present("o") {
        output = matches.opt_str("o").expect("missing file name after -o");
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        panic!("Error No input File selected");
    };

    let asm_file = FileHandler::new(input);
    let (sym_tab, mut raw_program) = sick_lib::pass_one::pass_one::pass_one(asm_file);
    let errs = sick_lib::pass_two::translator::pass_two(&mut raw_program);

    let mut sym_tab = sym_tab.into_iter()
        .map(|e| (e.0, e.1))
        .collect::<Vec<(String, i32)>>();

    // Sort by address
    sym_tab.sort_by(|a, b| a.1.cmp(&b.1));
    // Create the table
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Address"), Cell::new("Name")]));
    for (name, address) in sym_tab {
        table.add_row(Row::new(vec![ 
        Cell::new(&format!("{:04X}", address)).with_style(term::Attr::ForegroundColor(color::BRIGHT_BLUE)),Cell::new(&name)]));
    }
    table.printstd();

    print!("\n\n\n");

    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Loc"),
                                Cell::new("Label"),
                                Cell::new("Mnemonic"),
                                Cell::new("Obj")]));
    for (objcode, instr) in raw_program.program {
        table.add_row(Row::new(vec![Cell::new(&format!("{:04X}", instr.locctr))
                                        .with_style(term::Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                    Cell::new(&instr.label),
                                    Cell::new(&instr.mnemonic),
                                    Cell::new(&objcode)
                                        .with_style(term::Attr::ForegroundColor(color::BRIGHT_YELLOW))]));
    }
    table.printstd();

    // TODO: don't produce HTME on errors
    for err in errs {
        println!("{}", err);
    }
}
