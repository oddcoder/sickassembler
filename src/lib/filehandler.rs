
use std::fs::File;
use std::io::BufReader;
use std::{u32, usize};

use std::io::BufRead;

use instruction_set::{AssemblyDef, fetch_directive, fetch_instruction};
use instruction::*;
use unit_or_pair::*;
use formats::*;
use operand_parsing::{parse_directive_operand, parse_instruction_operand};
use super::*;

pub struct FileHandler {
    buf: BufReader<File>,
    pub errs: Vec<String>,
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

        let line = self.process_file().unwrap();
        let (name, start_addr) = self.read_start(line)
            .map_err(|e| self.errs.push(e))
            .unwrap_or((String::new(), usize::MAX));

        prog.program_name = name;
        prog.first_instruction_address = start_addr as u32;

        while let Some(line) = self.process_file() {
            if let Some(instruction) = self.read_instruction(line) {
                prog.program.push((String::new(), instruction));
            }
        }

        Ok((prog, start_addr))
    }

    fn read_start(&mut self, line: String) -> Result<(String, usize), String> {
        let words: Vec<&str> = line.trim().split_whitespace().collect();

        if words.len() > 3 {
            return Err(format!("Unexpected \"{}\"", words[3]));
        }

        match words[1] {
            "START" => return Ok((words[0].to_owned(), words[2].parse().unwrap())),
            _ => return Err(format!("Expected \"START\" found \"{}\"", words[1])),
        }
    }

    #[allow(unused_mut)]
    fn read_instruction(&mut self, line: String) -> Option<Instruction> {

        let mut inst: Instruction;
        let mut def: AssemblyDef;
        match self.parse_line_of_code(line) {
            None => return None,
            Some((instruction, defi)) => {
                inst = instruction;
                def = defi;
            }
        };

        set_format(&mut inst, def);

        return Some(inst);
    }

    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    fn parse_line_of_code(&mut self, line: String) -> Option<(Instruction, AssemblyDef)> {

        let mut words: Vec<String> = line.split_whitespace()
            .map(|x| x.trim().to_owned())
            .collect::<Vec<String>>();

        let mut label: String = String::new();
        let mut instruction: String;
        let mut instruction_def: AssemblyDef = AssemblyDef::dummy();
        let mut is_format_4 = false;
        let mut is_directive = false;

        // FIXME: this method of splitting is ruined, splitting by white_space can cause
        // errors easily
        // FIXME: hex immediate operands #X'F1' -> convert to immediate ( signed int ) / Raw
        // FIXME: label rsub
        let mut operands: UnitOrPair<AsmOperand> = UnitOrPair::None;

        if words.len() > 3 || words.is_empty() {
            self.errs.push(format!("Invalid code at line #{}, {:?}", self.line_number, words));
        }

        if words.len() == 3 {
            let temp: String = words.drain(0..1).collect();
            if is_label(&temp) {
                label = temp;
            } else {
                self.errs.push(format!("Invalid label token at line #{}", self.line_number));
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
                self.errs.push(format!("{}", e));
                return None;
            }
        }

        if !words.is_empty() {
            match parse_operands(words.drain(0..1).collect(), is_directive) {
                Ok(e) => operands = e,
                Err(e) => self.errs.push(e),
            };
        }

        let mut inst = Instruction::new(label, instruction, operands);
        if is_format_4 {
            inst.set_format(Format::Four);
        }

        Some((inst, instruction_def))
    }

    /// Reads a line of code, removing the comments and bypassing empty lines
    fn process_file(&mut self) -> Option<String> {
        // Returns ->
        // None -> EOF
        // Some -> Code
        // Panic -> I/O error

        let mut line: String = String::new();

        while self.buf.read_line(&mut line).unwrap() > 0 {
            self.line_number = self.line_number + 1;
            let temp = (*COMMENT_REGEX.replace_all(line.as_str(), "")).to_owned();
            let temp = temp.trim();
            line.clear();
            if temp.is_empty() {
                continue;
            }
            return Some(temp.to_owned());
        }
        None
    }
}

fn parse_operands(operand_string: String,
                  is_directive: bool)
                  -> Result<UnitOrPair<AsmOperand>, String> {
    let ops: Vec<&str> = operand_string.split(",").collect();
    let mut errs: Vec<String> = Vec::new();

    match ops.len() {
        0 => return Ok(UnitOrPair::None),
        1 => {
            let op = if is_directive {
                parse_directive_operand(ops[0])
            } else {
                parse_instruction_operand(ops[0])
            };
            match op {
                Ok(o) => return Ok(UnitOrPair::Unit(o)),
                Err(e) => {
                    errs.push(e);
                    return Err(errs.join("\n"));
                }
            }
        }
        2 => {
            if is_directive {
                panic!("Assembler directives can't have 2 Operands");
            }
            let op1 = parse_instruction_operand(ops[0]);
            let op2 = parse_instruction_operand(ops[1]);
            if op1.is_ok() && op2.is_ok() {
                return Ok(UnitOrPair::Pair(op1.unwrap(), op2.unwrap()));
            } else {
                let _ = op1.map_err(|e| errs.push(e));
                let _ = op2.map_err(|e| errs.push(e));
                return Err(errs.join("\n"));
            }

        }
        _ => {
            errs.push(format!("expected . or newline instead of `{}`", ops[2]));
            return Err(errs.join("\n"));
        }
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

#[allow(unused_mut)] // Compiler generates false warnings
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
        return Err(format!("\"{}\" isn't an instruction nor directive", inst));
    }
    return Ok((instruction_def, is_format_4, is_directive));
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
    fn test_literals() {
        assert!(is_literal("=C'EOF'"));
        assert!(is_literal("=X'1EF'"));
        assert!(is_literal("=X'10'"));
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
