use std::fs::File;
pub struct FileHandler {
    path: String,
    file: File,
}
impl FileHandler {
    fn new (Path: String) -> FileHandler {
        let file = File::open(&Path).unwrap();
        return FileHandler{
            path: Path,
            file: file,
        }
    }

}
