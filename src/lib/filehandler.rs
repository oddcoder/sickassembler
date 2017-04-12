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

#[test]
#[should_panic]
fn test_file_opening() {
    FileHandler::new("God Damn long file name that should never exit.asm".to_string());
}
