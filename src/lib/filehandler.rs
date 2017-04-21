use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use basic_types::instruction_set::*;
use basic_types::instruction::*;
use basic_types::unit_or_pair::*;
use basic_types::register::*;
use basic_types::operands::*;
use basic_types::formats::*;
use basic_types::literal_table::{get_unresolved, insert_literal, insert_unresolved};
use basic_types::base_table::{set_base, end_base};

pub struct FileHandler {
    path: String,
    buf: BufReader<File>,
}

impl FileHandler {
    pub fn new(path: String) -> FileHandler {
        let file = File::open(&path).unwrap();
        let f = BufReader::new(file);
        return FileHandler {
            path: path,
            buf: f,
        };
    }
    pub fn read_start(&mut self) -> (String, usize) {
        let line;
        match self.scrap_comment() {
            None => panic!("Excepted START line"),
            Some(x) => line = x,
        }
        let mut words: Vec<&str> = line.split_whitespace().collect();
        if words.len() > 3 {
            panic!("Unexpected \"{}\"", words[3]);
        }
        match words[1] {
            "START" => return (words[0].to_owned(), words[2].parse().unwrap()),
            _ => panic!("Expected \"START\" found \"{}\"", words[1]),
        }
    }
    pub fn read_instruction(&mut self) -> Option<Instruction> {
        //TODO refactor later ...
        let line;
        match self.scrap_comment() {
            None => return None,
            Some(x) => line = x,
        }

        let mut words = line.split_whitespace();
        let mut label: String = String::new();
        let mut instruction: String = String::new();
        let mut instruction_def: AssemblyDef = AssemblyDef::dummy();
        let mut is_format_4 = false;
        let mut maybe_instruction = words.next().unwrap().to_string();
        let mut is_directive = false;
        if &maybe_instruction[0..1] == "+" {
            is_format_4 = true;
            maybe_instruction.pop();
        }
        match fetch_instruction(&maybe_instruction) {
            Err(meh) => {
                if is_format_4 {
                    panic!("Label can not start with a +");
                }
                match fetch_directive(&maybe_instruction) {
                    Err(meh) => label = maybe_instruction.to_owned(),
                    Ok(def) => {
                        instruction = maybe_instruction.to_owned();
                        instruction_def = def;
                        is_directive = true;
                    }
                }
            }
            Ok(def) => {
                instruction = maybe_instruction.to_owned();
                instruction_def = def;
            }
        }
        if !label.is_empty() {
            instruction = words.next().unwrap().to_owned();
            if &instruction[0..1] == "+" {
                is_format_4 = true;
                instruction.pop();
            }
            match fetch_instruction(&instruction) {
                Err(_) => {
                    match fetch_directive(&instruction) {
                        Err(_) => {
                            panic!("{} is neither instruction nor pseudo-instruction",
                                   instruction)
                        }
                        Ok(def) => {
                            instruction_def = def;
                            is_directive = true;
                        }
                    }
                }
                Ok(def) => instruction_def = def,
            }
        }
        let mut operands: UnitOrPair<AsmOperand> = parse_operands(words.next(), &is_directive);
        /*TODO fix has_valid_operands
        if !instruction_def.has_valid_operands(&operands) {
            panic!("Invalid operands for istruction {}", instruction_def.mnemonic);
        }
        */
        // TODO: Check for action directives, don't add them to instruction vector
        // if let Some(directive) = is_action_directive(instruction) {
        //     // LTORG,BASE,NOBASE, those will be ignored in translation
        // }

        let mut inst: Instruction = Instruction::new(label, instruction, operands);
        if is_format_4 {
            inst.set_format(Format::Four);
        } else {
            let format = unwrap_to_vec(&instruction_def.format);
            match format.len() {
                0 => (),
                1 => inst.format = format[0],
                2 => inst.format = Format::Three,
                _ => {
                    panic!("We Just found an instruction that had more than 2 formats! you are \
                            screwed")
                }
            }
        }
        return Some(inst);
    }
    /// Removes comments if found in a line, and skips
    /// empty lines.
    fn scrap_comment(&mut self) -> Option<String> {
        let mut line = String::new();

        loop {
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

fn parse_operands(operands: Option<&str>, is_directive: &bool) -> UnitOrPair<AsmOperand> {
    let operand_string;
    match operands {
        None => return UnitOrPair::None,
        Some(op) => operand_string = op.to_owned(),
    }
    let ops: Vec<&str> = operand_string.split(",").collect();
    match ops.len() {
        0 => return UnitOrPair::None,
        1 => {
            let op = parse(ops[0].to_owned(), &is_directive);
            return UnitOrPair::Unit(op);
        }
        2 => {
            if *is_directive {
                panic!("Assembler directives can't have 2 Operands");
            }
            let op1 = parse(ops[0].to_owned(), &false);
            let op2 = parse(ops[1].to_owned(), &false);
            return UnitOrPair::Pair(op1, op2);
        }
        _ => panic!("expected . or newline instead of `{}`", ops[2]),
    }
    return UnitOrPair::None;
}

fn parse(op: String, is_directive: &bool) -> AsmOperand {
    match &op as &str {
        "A" => return AsmOperand::new(OperandType::Register, Value::Register(Register::A)),
        "X" => return AsmOperand::new(OperandType::Register, Value::Register(Register::X)),
        "L" => return AsmOperand::new(OperandType::Register, Value::Register(Register::L)),
        "B" => return AsmOperand::new(OperandType::Register, Value::Register(Register::B)),
        "S" => return AsmOperand::new(OperandType::Register, Value::Register(Register::S)),
        "T" => return AsmOperand::new(OperandType::Register, Value::Register(Register::T)),
        "F" => return AsmOperand::new(OperandType::Register, Value::Register(Register::F)),
        _ => (),
    }
    let mut optype = OperandType::Label;
    if *is_directive {
        let char_stream = Regex::new(r"^C'[[:alnum:]]*'$").unwrap();
        let hex_stream = Regex::new(r"^X'[[:xdigit:]]*'$").unwrap();

        if op.starts_with("=") &&
           (char_stream.is_match(&op[1..]) || hex_stream.is_match(&op[1..])) {
            // TODO: insert to literal table here

        } else if char_stream.is_match(&op) || hex_stream.is_match(&op) {
            optype = OperandType::Bytes;
        }

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
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            if *is_directive {
                optype = OperandType::Immediate;
            } else {
                optype = OperandType::Raw;
            }
        }
        _ => (),
    }
    let val = op[index_start..op.len()].to_owned();
    let mut x = usize::from_str_radix(&val[0..1], 10);
    match x {
        Err(_) => {

            if val.starts_with("=") {
                // TODO: check for littab entry
            }
            return AsmOperand::new(optype, Value::Label(val));
        }
        Ok(_) => {
            if *is_directive {
                return AsmOperand::new(optype,
                                       Value::Raw(usize::from_str_radix(&val, 16).unwrap() as u32));
                return AsmOperand::new(optype, Value::Raw(val.parse().unwrap()));
            } else {
                return AsmOperand::new(optype, Value::Raw(val.parse().unwrap()));
            }
        }
    }
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
            .collect::<Vec<&str>>();
        
        // Without regex
        let mut asm_file = FileHandler::new("src/tests/test1.asm".to_owned());
        let mut instruction_count = 0;
        let start = asm_file.read_start();
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
            // match wrapped_line {
            //     None => break,
            //     Some(ref s) if s.is_empty() => continue,
            //     Some(s) => {}
            // }
            // Check that the output from regex has the same number of lines as the
            // output from the function
        }
        println!("{:?} --> {}", lines, lines.len());
        assert_eq!(instruction_count, lines.len());
    }
}
