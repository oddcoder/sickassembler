use std::fs::*;
use std::io::{BufReader, BufRead};
use basic_types::instruction_set::*;
use basic_types::instruction::*;

#[derive(Debug)]
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

    pub fn read_instruction(&mut self) -> Option<String> {
        return self.scrap_comment();
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
