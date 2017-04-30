extern crate prettytable;
extern crate sick_lib;
extern crate getopts;
extern crate env_logger;
extern crate term;

use term::color;
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

    // let mut output = "a.out".to_string();
    // if matches.opt_present("o") {
    //     output = matches.opt_str("o").expect("missing file name after -o");
    // }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        panic!("Error No input File selected");
    };

    let mut asm_file = FileHandler::new(input);
    let result = asm_file.parse_file();

    let mut t = term::stdout().unwrap();
    t.fg(term::color::RED).unwrap();
    t.attr(term::Attr::Bold).unwrap();
    for err in asm_file.errs {
        write!(t, "{}\n", err).unwrap();
    }
    t.reset().unwrap();


    let result = sick_lib::pass_one::pass_one::pass_one(result);
    let result = result.map_err(|e| panic!("{}", e));

    let (sym_tab, mut raw_program): (_, _) = result.unwrap();

    t.fg(term::color::YELLOW).unwrap();
    write!(t,
           "Prog name:{}, prog length:{:X}, prog start addr:{}\n",
           raw_program.program_name,
           raw_program.program_length,
           raw_program.first_instruction_address)
        .unwrap();
    t.reset().unwrap();

    let errs = sick_lib::pass_two::translator::pass_two(&mut raw_program);

    let mut sym_tab = sym_tab.into_iter()
        .map(|e| (e.0, e.1))
        .collect::<Vec<(String, i32)>>();

    // TODO: don't produce HTME on errors
    t.fg(term::color::BRIGHT_RED).unwrap();
    t.attr(term::Attr::Bold).unwrap();
    for err in &errs {
        println!("{}", err);
    }
    t.reset().unwrap();
    if errs.len() > 0 {
        return;
    }

    // Sort by address
    sym_tab.sort_by(|a, b| a.1.cmp(&b.1));
    // Create the table
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Address"), Cell::new("Name")]));
    for (name, address) in sym_tab {
        table.add_row(Row::new(vec![ 
        Cell::new(&format!("{:04X}", address))
        .with_style(term::Attr::ForegroundColor(color::BRIGHT_BLUE)),Cell::new(&name)]));
    }
    table.printstd();

    print!("\n\n\n");

    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Loc"),
                                Cell::new("Label"),
                                Cell::new("Mnemonic"),
                                Cell::new("Format"),
                                Cell::new("Obj")]));
    for &(ref objcode, ref instr) in &raw_program.program {
        table.add_row(Row::new(vec![Cell::new(&format!("{:04X}", instr.locctr))
                                        .with_style(term::Attr::ForegroundColor(color::BRIGHT_BLUE)),
                                    Cell::new(&instr.label),
                                    Cell::new(&instr.mnemonic),
                                    Cell::new(format!("{:?}",&instr.get_format()).as_str()),
                                    Cell::new(&objcode)
                                        .with_style(term::Attr::ForegroundColor(color::BRIGHT_YELLOW))]));
    }
    table.printstd();
    raw_program.output_to_file();
}
