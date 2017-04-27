use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::u32;

use instruction_set::*;
use instruction::*;
use unit_or_pair::*;
use register::*;
use operands::*;
use formats::*;
use literal_table::insert_unresolved;
use super::RawProgram;

pub struct FileHandler {
    buf: BufReader<File>,
    errs: Vec<String>,
    line_number: i32,
}

impl FileHandler {
    pub fn new(path: String) -> FileHandler {
        let file = File::open(&path).unwrap();
        let f = BufReader::new(file);
        return FileHandler {
            buf: f,
            errs: Vec::new(),
            line_number: 0,
        };
    }

    /// Returns:
    ///     - the program instructions
    ///     - address in START instruction
    ///     - address in END instruction
    pub fn parse_file(&mut self) -> Result<(RawProgram, usize), String> {
        let mut prog: RawProgram = RawProgram {
            program_name: String::new(),
            starting_address: u32::MAX,
            program_length: u32::MAX,
            program: Vec::new(),
            first_instruction_address: u32::MAX,
        };
        let mut line_number: i32 = 1;
        let (name, start_addr) = self.read_start();
        prog.program_name = name;
        prog.first_instruction_address = start_addr as u32;

        {
            while let Some(instruction) = self.read_instruction() {
                prog.program.push((String::new(), instruction));
            }
        };

        Ok((prog, start_addr))
    }

    fn read_start(&mut self) -> (String, usize) {
        let line;
        match self.scrap_comment() {
            None => panic!("Excepted START line"),
            Some(x) => line = x,
        }
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() > 3 {
            panic!("Unexpected \"{}\"", words[3]);
        }

        match words[1] {
            "START" => return (words[0].to_owned(), words[2].parse().unwrap()),
            _ => panic!("Expected \"START\" found \"{}\"", words[1]),
        }
    }

    fn read_instruction(&mut self) -> Option<Instruction> {
        //TODO refactor later

        let line;
        match self.scrap_comment() {
            None => return None,
            Some(x) => line = x,
        }

        let mut inst: Instruction;
        let mut def: AssemblyDef;
        match self.parse_line_of_code(line) {
            None => return None,    // TODO: change to error
            Some((instruction, defi)) => {
                inst = instruction;
                def = defi;
            }
        };

        set_format(&mut inst, def);

        return Some(inst);
    }

    fn parse_line_of_code(&mut self, line: String) -> Option<(Instruction, AssemblyDef)> {

        let mut words: Vec<String> = line.split_whitespace()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        let mut label: String = String::new();
        let mut instruction: String;
        let mut instruction_def: AssemblyDef = AssemblyDef::dummy();
        let mut is_format_4 = false;
        let mut is_directive = false;

        // TODO: this method of splitting is ruined, splitting by white_space can cause
        // errors easily
        let mut operands: UnitOrPair<AsmOperand> = UnitOrPair::None;
        let mut inst: Instruction;

        if words.len() > 3 || words.is_empty() {
            self.errs.push(format!("Invalid code at line {}", self.line_number));
        }

        if words.len() == 3 {
            let temp: String = words.drain(0..1).collect();
            if let Ok(true) = is_label(&temp) {
                label = temp;
            } else {
                self.errs.push(format!("Invalid label token at line {}", self.line_number));
                return None;
            }
        }

        instruction = words.drain(0..1).collect();
        match get_def(&mut instruction) {
            Ok((def, is_4, is_dir)) => {
                instruction_def = def;
                is_format_4 = is_4;
                is_directive = is_dir;
            }
            Err(e) => {
                self.errs.push(format!("Error at line {}, {}", e, self.line_number));
                return None;
            }
        }

        if !words.is_empty() {
            operands = parse_operands(words.drain(0..1).collect(), is_directive);
        }

        let mut inst = Instruction::new(label, instruction, operands);
        if is_format_4 {
            inst.set_format(Format::Four);
        }

        Some((inst, instruction_def))
    }

    /// Removes comments if found in a line, and skips
    /// empty lines.
    fn scrap_comment(&mut self) -> Option<String> {
        let mut line = String::new();

        loop {
            self.line_number = self.line_number + 1;
            match self.buf.read_line(&mut line) {
                Ok(num) => {
                    if num == 0 {
                        return None;
                    } else {
                        line = line.split(".").nth(0).unwrap().trim().to_owned();

                        if line.is_empty() {
                            continue;
                        }

                        return Some(line);
                    }
                }
                Err(e) => {
                    panic!(format!("An OS I/O error occured, this is really bad!, {}",
                                   e.to_string()))
                }
            }

        }
    }
}

fn parse_operands(operand_string: String, is_directive: bool) -> UnitOrPair<AsmOperand> {
    let ops: Vec<&str> = operand_string.split(",").collect();

    match ops.len() {
        0 => return UnitOrPair::None,
        1 => {
            let op = parse(ops[0].to_owned(), is_directive);
            return UnitOrPair::Unit(op);
        }
        2 => {
            if is_directive {
                panic!("Assembler directives can't have 2 Operands");
            }
            let op1 = parse(ops[0].to_owned(), false);
            let op2 = parse(ops[1].to_owned(), false);
            return UnitOrPair::Pair(op1, op2);
        }
        _ => panic!("expected . or newline instead of `{}`", ops[2]),
    }
}

fn parse(op: String, is_directive: bool) -> AsmOperand {
    if !is_directive {
        match parse_register(&op) {
            Some(x) => return x,
            None => (),
        };
    }

    let mut optype = OperandType::Label;

    // TODO: possibly redundant
    if is_ascii_or_word_operand(&op) {
        optype = OperandType::Bytes;
    }

    let mut index_start = 0;

    match &op[0..1] {
        "#" => {
            optype = OperandType::Immediate;
            index_start = 1;
        }
        "@" => {
            optype = OperandType::Indirect;
            index_start = 1;
        }
        "=" if is_literal(&op) => {
            // Literals will be treated as labels
            // Add the = sign at the start of name to avoid errors in pass_one

        //     insert_unresolved(&(op.to_owned()));
        // }
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            if is_directive {
                // Directives are used with decimal values
                optype = OperandType::Immediate;
            } else {
                // Shift operations
                optype = OperandType::Raw;
            }
        }
        // TODO: fix this SUPER ugly code
        "A" | "B" | "C" | "D" | "E" | "F" if is_directive && !is_literal(&op) &&
                                             !is_ascii_or_word_operand(&op) => {
            optype = OperandType::Raw; // Start / End
            return AsmOperand::new(optype, Value::Raw(u32::from_str_radix(&op, 16).unwrap()));
        }
        _ => (),
    }

    let val = op[index_start..op.len()].to_owned();
    let x = usize::from_str_radix(&val[0..1], 10);

    match x {
        Err(_) => {
            // Isn't a number -> label or literal
            if is_ascii_or_word_operand(&op) {
                optype = OperandType::Bytes;
                return AsmOperand::new(optype, Value::Bytes(val));
            } else {
                return AsmOperand::new(optype, Value::Label(val));
            }
        }
        Ok(_) => {
            if is_directive {
                // Directive operands are in decimal
                return AsmOperand::new(optype,
                                       Value::SignedInt(i32::from_str_radix(&val, 10).unwrap()));
            } else {
                // Source file doesn't contain hexadecimal instruction parameters
                // Only immediate -> Value::SignedInt
                return AsmOperand::new(optype, Value::SignedInt(val.parse().unwrap()));
            }
        }
    }
}

fn parse_register(op: &String) -> Option<AsmOperand> {
    match &op as &str {
        "A" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::A))),
        "X" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::X))),
        "L" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::L))),
        "B" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::B))),
        "S" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::S))),
        "T" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::T))),
        "F" => Some(AsmOperand::new(OperandType::Register, Value::Register(Register::F))),
        _ => None,
    }
}

fn set_format(inst: &mut Instruction, instruction_def: AssemblyDef) {
    if inst.get_format() != Format::None {
        return;
    }

    let format = unwrap_to_vec(&instruction_def.format);
    match format.len() {
        0 => (),
        1 => inst.set_format(format[0]),
        2 => inst.set_format(Format::Three),
        _ => panic!("We Just found an instruction that had more than 2 formats! you are screwed"),
    }
}

fn get_def(inst: &mut String) -> Result<(AssemblyDef, bool, bool), String> {
    let mut instruction_def: AssemblyDef;
    let mut is_format_4 = false;
    let mut is_directive = false;

    if inst.starts_with("+") {
        is_format_4 = true;
        *inst = inst.trim_left_matches("+").to_owned(); // Ignore the '+' sign
    }

    if let Ok(def) = fetch_instruction(&inst) {
        instruction_def = def;
    } else if let Ok(def) = fetch_directive(&inst) {
        is_directive = true;
        instruction_def = def;
    } else {
        return Err("supplied code isn't an instruction nor directive".to_owned());
    }
    return Ok((instruction_def, is_format_4, is_directive));
}

/// Tells whether a token is a valid label or an instruction
/// returns:
/// Err -> invalid token (Not a label nor instruction)
/// Result ->
///     - true : label
///     - false : instruction
fn is_label(suspect: &String) -> Result<bool, String> {
    // TODO: replace with existing matching in above functions
    let not_decodable = fetch_directive(suspect).is_err() && fetch_instruction(suspect).is_err();
    let is_valid_name = LABEL_STREAM.is_match(suspect);
    if suspect.is_empty() {
        return Ok(false);
    }
    if not_decodable && !is_valid_name {
        Err(format!("Invalid token {}", suspect))
    } else {
        Ok(is_valid_name)
    }
}

/// A literal is a byte/chars preceeded by an '=' sign
fn is_literal(op: &String) -> bool {
    return op.starts_with("=") && is_ascii_or_word_operand(op.trim_left_matches("="));
}

/// An ascii operand is on the form  (C|X)'...'
fn is_ascii_or_word_operand(op: &str) -> bool {
    return CHAR_STREAM.is_match(&op) || HEX_STREAM.is_match(&op);
}

lazy_static!{
    static ref CHAR_STREAM:Regex = Regex::new(r"^C'[[:alnum:]]+'$").unwrap();
    static ref HEX_STREAM:Regex = Regex::new(r"^X'[[:xdigit:]]+'$").unwrap();
    static ref LABEL_STREAM:Regex = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*; // Use all your parent's imports
    use regex::Regex;
    use std::io::Read;

    #[test]
    #[should_panic]
    fn test_file_opening() {
        FileHandler::new("God Damn long file name that should never exit.asm".to_string());
    }

    #[test]
    fn line_count_correct() {
        let lines = with_regex();

        // Without regex
        let mut asm_file = FileHandler::new("src/tests/test1.asm".to_owned());
        let mut instruction_count = 0;

        asm_file.read_start();
        instruction_count = instruction_count + 1;

        loop {
            let instruction = asm_file.read_instruction();
            match instruction {
                None => break,
                Some(ref s) => {
                    println!("{:?} {:?}", instruction_count, s);
                    instruction_count += 1;
                }
            }
        }
        println!("{:?} --> {}", lines, lines.len());
        assert_eq!(instruction_count, lines.len());
    }

    #[test]
    fn test_parse_file() {
        let lines = with_regex();
        // Without regex
        let mut asm_file = FileHandler::new("src/tests/test1.asm".to_owned());

        // Start and End are not included in parse_file result
        let instruction_count_without_start = lines.len() - 2;

        let (prog, _) = asm_file.parse_file().unwrap();

        assert_eq!(prog.program.len(), instruction_count_without_start);

    }

    /// Extracts code in file using regex
    fn with_regex() -> Vec<String> {
        /// Matches the number of instructions that come out from code
        /// with the number of instructions in file

        let mut asm_file = FileHandler::new("src/tests/test1.asm".to_owned());

        // Regex reference: http://kbknapp.github.io/doapi-rs/docs/regex/index.html
        // Escape all empty lines or comment lines
        let empty_lines_regex = Regex::new(r"(?m)^\s*\n|^\s+").unwrap();
        let comment_regex = Regex::new(r"(?m)\..+").unwrap();
        let mut file_content: String = String::new();
        match asm_file.buf.read_to_string(&mut file_content) {
            Err(e) => println!("{}", e),
            _ => (),
        };

        let empty_lines_cleared = empty_lines_regex.replace_all(&file_content, "");
        let comments_cleared = comment_regex.replace_all(&empty_lines_cleared, "");

        let lines = comments_cleared.split("\n")
            .filter(|s: &&str| !s.is_empty())
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        lines
    }
}
