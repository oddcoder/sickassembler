use std::fs::File;
pub struct FileHandler {
    path: String,
    file: File,
}
impl FileHandler {
    pub fn new (path: String) -> FileHandler {
        let file = File::open(&path).unwrap();
        return FileHandler{
            path: path,
            file: file,
        }
    }

}
#[test]
#[should_panic]
fn test_file_opening() {
    FileHandler::new ("God Damn long file name that should never exit.asm".to_string());
}
