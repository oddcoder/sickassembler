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
    fn scrap_comment(&mut self) -> Option<String> {
        let mut line = String::new();

        // Read until you reach end of file or a non-comment line
        // if the line is just a blank line, skip it
        loop {
            match self.buf.read_line(&mut line) {
                Ok(num) => {
                    if num == 0 {
                        // Nothing Read
                        return None;
                    } else {
                        // Remove the comments form the line read
                        line = line.split(".").nth(0).unwrap().trim().to_owned();

                        if line.is_empty() {
                            continue; // Skip empty lines or lines that contain only comments
                        }

                        return Some(line);
                    }
                }
                Err(e) => panic!("An OS I/O error occured, this is really bad!"),
            }

        }
    }
}

#[test]
#[should_panic]
fn test_file_opening() {
    FileHandler::new("God Damn long file name that should never exit.asm".to_string());
}
