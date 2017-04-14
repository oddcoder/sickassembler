use std::fs::*;
use std::io::{BufReader, BufRead};
use basic_types::instruction_set::*;
use basic_types::instruction::*;
use basic_types::unit_or_pair::*;
use basic_types::register::*;
use basic_types::operands::*;
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
        let mut words:Vec<&str> = line.split_whitespace().collect();
        if words.len() > 3 {
            panic!("Unexpected \"{}\"", words[3]);
        }
        match words[1] {
            "START" => return (words[0].to_owned(), words[2].parse().unwrap()),
            _ => panic!("Expected \"START\" found \"{}\"", words[1]),
        }
    }
    pub fn read_instruction(&mut self) -> Option<Instruction> {
        let line;
        match self.scrap_comment() {
            None => return None,
            Some(x) => line = x,
        }
        let mut words = line.split_whitespace();
        let mut label:String = String::new();
        let mut instruction:String = String::new();
        let mut instruction_def:AssemblyDef;
        let maybe_label = words.next().unwrap().to_string();
        match fetch_instruction(&maybe_label) {
            Err(meh) => {
                label = maybe_label.to_string();
            },
            Ok(def) => {
                instruction = maybe_label.to_string();
                instruction_def = def;
            },
        }
        if instruction.is_empty() {
            instruction = words.next().unwrap().to_owned();
            match fetch_instruction(&instruction) {
                Err(why) => panic!(why.clone()),
                Ok(def) => instruction_def = def,
            }
        }
        let operands:UnitOrPair<AsmOperand> = parse_operands(words.next());
        let mut inst:Instruction = Instruction::new(label, instruction, operands);
        return Some(inst);
    }
    /// Removes comments if found in a line, and returns
    /// an empty string if the line didn't conatin code
    fn scrap_comment(&mut self) -> Option<String> {
        let mut line = String::new();

        match self.buf.read_line(&mut line) {
            Ok(bytes_read) if bytes_read > 0 => {
                match FileHandler::extract_code(&line) {
                    Some(code_line) => {
                        return Some(code_line);
                    }
                    None => Some(String::new()),
                }
            }
            Ok(_) => return None,
            Err(e) => {
                panic!(format!("An OS I/O error occured, this is really bad!, {}",
                               e.to_string()))
            }
        }
    }

    /// Extracts code in source file line
    fn extract_code(line: &String) -> Option<String> {
        let trimmed: String = line.split(".").nth(0).unwrap().trim().to_owned();

        if trimmed.is_empty() {
            return None;
        }

        Some(trimmed)
    }
}

fn parse_operands(operands:Option<&str>) ->UnitOrPair<AsmOperand> {
    let operand_string;
    match operands {
        None => return UnitOrPair::None,
        Some(op) => operand_string = op.to_owned(),
    }
    let ops: Vec<&str> = operand_string.split(",").collect();
    match ops.len() {
        0 => return UnitOrPair::None,
        1 => {
            let op = parse(ops[0].to_owned());
            return UnitOrPair::Unit(op);
        },
        2 => {
            let op1 = parse(ops[0].to_owned());
            let op2 = parse(ops[1].to_owned());
            return UnitOrPair::Pair(op1, op2);
        },
        _ => panic!("expected ; or newline instead of `{}`", ops[2]),
    }
    return UnitOrPair::None;
}

fn parse(op:String) -> AsmOperand {
    match &op as &str {
        "A"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::A)),
        "X"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::X)),
        "L"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::L)),
        "B"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::B)),
        "S"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::S)),
        "T"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::T)),
        "F"=> return AsmOperand::new(OperandType::Register, Value::Register(Register::F)),
        _ => (),
    }
    let mut optype = OperandType::Label;
    let mut index_start = 0;
    match &op[0..1] {
        "#" => {
            optype = OperandType::Immediate;
            index_start = 1;
        },
        "@" =>  {
            optype = OperandType::Indirect;
            index_start = 1;
        }
        "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => optype = OperandType::Raw,
        _ => (),
    }
    let val = op[index_start..op.len()].to_owned();
    let x = usize::from_str_radix(&val[0..1], 10);
    match x {
        Err(_) => return AsmOperand::new(optype, Value::Label(val)),
        Ok(_) => return AsmOperand::new(optype, Value::Raw(val.parse().unwrap())),
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
        let mut line_count = 0;

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

        let mut asm_file = FileHandler::new("src/tests/test1.asm".to_owned());
        loop {
            let wrapped_line = asm_file.read_instruction();
            match wrapped_line {
                None => break,
                Some(ref s) if s.is_empty() => continue,
                Some(s) => {
                    println!("{} {}", line_count, s);
                    line_count += 1;
                }
            }
        }
        println!("{:?} --> {}", lines, lines.len());
        // Check that the output from regex has the same number of lines as the
        // output from the function
        assert_eq!(line_count, lines.len());
    }
}
