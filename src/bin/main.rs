extern crate sick_lib;
extern crate getopts;
extern crate env_logger;
use getopts::Options;
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
    let mut asm_file = FileHandler::new(input);
    // let (name, loc) = asm_file.read_start();
    // println!("File Name: {}.o", name);
    // println!("Location Counter starts at: {}", loc);
    
    let prog = asm_file.parse_file();

    for inst in &prog.unwrap().program {
        println!("{:?}", inst);
    }

}
