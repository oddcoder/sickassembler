#[macro_use]
extern crate prettytable;
extern crate env_logger;
extern crate getopts;
extern crate sick_lib;
extern crate term;

use getopts::Options;

use prettytable::{Cell, Row, Table};
use term::color;

//use instruction::Instruction;
//use operands::OperandType;
use sick_lib::filehandler::FileHandler;
use std::env;
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options] file", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let exit_on_error: bool = false;
    env_logger::init();
    // credits goes to here:-
    // https://doc.rust-lang.org/getopts/getopts/index.html
    // Time will come where I will fully understant this!
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "name");
    opts.optflag("c", "csect", "print control section details");
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
    print_errs(&asm_file.errs, exit_on_error);

    let result = sick_lib::pass_one::pass_one::pass_one(result.unwrap());
    let result = result.map_err(|e| print_error(&e, exit_on_error));
    if result.is_err() {
        return;
    }

    let (sym_tab, mut raw_program): (_, _) = result.unwrap();

    t.fg(term::color::YELLOW).unwrap();
    write!(
        t,
        "Prog name:{}, prog length:{:#X}, prog start addr:{:#X}\n",
        raw_program.program_name, raw_program.program_length, raw_program.first_instruction_address
    ).unwrap();
    t.reset().unwrap();

    let errs = sick_lib::pass_two::translator::pass_two(&mut raw_program);

    let mut sym_tab = sym_tab
        .into_iter()
        .map(|e| (e.get_name(), e.get_address(), e.get_control_section()))
        .collect::<Vec<(String, i32, String)>>();

    print_errs(&errs, exit_on_error);

    // Print control sections info
    if matches.opt_present("c") {
        print_csect_info();
    }

    // Sort by address
    sym_tab.sort_by(|a, b| a.1.cmp(&b.1));
    // Create the table
    let mut table = Table::new();
    table.add_row(row!["Address", "Name", "Control Section"]);
    for (name, address, csect) in sym_tab {
        table.add_row(row![
            Cell::new(&format!("{:04X}", address))
                .with_style(term::Attr::ForegroundColor(color::BRIGHT_BLUE)),
            &name,
            &csect,
        ]);
    }
    table.printstd();

    print!("\n\n\n");

    let mut table = Table::new();
    table.add_row(row!["Loc", "Label", "Mnemonic", "Format", "Obj",]);
    for &(ref objcode, ref instr) in &raw_program.program {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{:04X}", instr.locctr))
                .with_style(term::Attr::ForegroundColor(color::BRIGHT_BLUE)),
            Cell::new(&instr.label),
            Cell::new(&instr.mnemonic),
            Cell::new(format!("{:?}", &instr.get_format()).as_str()),
            Cell::new(&objcode).with_style(term::Attr::ForegroundColor(color::BRIGHT_YELLOW)),
        ]));
    }
    table.printstd();
    raw_program.output_to_file();
}

fn print_csect_info() {
    let csects_info = sick_lib::pass_one::pass_one::get_csects_info();
    for sect_info in csects_info {
        println!("{}", sect_info);
    }
}

fn print_error(err: &String, exit_on_error: bool) {
    if err.len() == 0 {
        return;
    }
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_RED).unwrap();
    t.attr(term::Attr::Bold).unwrap();
    println!("{}", err);
    t.reset().unwrap();
    if exit_on_error {
        std::process::exit(-1);
    }
}

fn print_errs(errs: &Vec<String>, exit_on_error: bool) {
    if errs.len() == 0 {
        return;
    }
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_RED).unwrap();
    t.attr(term::Attr::Bold).unwrap();
    for err in errs {
        println!("{}", err);
    }
    t.reset().unwrap();
    if exit_on_error {
        std::process::exit(-1);
    }
}
