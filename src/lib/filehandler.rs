use std::fs::*;
use std::io::*;
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
    pub fn read_instruction(&mut self) -> Option<Result<String>> {
        let line;
        let wrapped_line = self.scrap_comment();
        if wrapped_line.is_none() {
            return None;
        }
        match wrapped_line.unwrap() { // keep in mind when A line is read ..
            Err(e) => return Some(Err(e)),                    // it is totally consumed and no way to get it back!
            Ok(s) => line = s,
        }
        return Some(Ok(line));
    }
    fn scrap_comment (&mut self) -> Option<Result<String>> {
        let mut line;
        loop {
            let commented_line;
            let wrapped_line =  (&mut self.buf).lines().nth(0);
            if wrapped_line.is_none() {
                return None;
            }
            match wrapped_line.unwrap() {
                Err(e) => return Some(Err(e)),
                Ok(s) => commented_line = s,
            }
            line = commented_line.split(".").nth(0).unwrap().trim().to_string();
            if !line.is_empty(){
                break;
            }
        }
        return Some(Ok(line));
    }
}
#[test]
#[should_panic]
fn test_file_opening() {
    FileHandler::new("God Damn long file name that should never exit.asm".to_string());
}
