use std::fs::File;
use std::io::{BufReader, BufRead, ErrorKind};
use std::path::Path;


pub struct FileIO{
    file: File,
}

impl FileIO{
    pub fn new(file_path: &Path) -> Self{ 
        let file: File = Self::open(file_path);

        Self{
            file,
        }
    }

    fn open(file_path: &Path) -> File{
        match File::open(file_path){
            Ok(file) => file,
            Err(e) => match e.kind(){
                ErrorKind::NotFound => match File::create(file_path){
                    Ok(file) => file,
                    Err(e) => panic!("Error: {}", e),
                }
                e => panic!("Error: {}", e),
            }
        }
    }

    pub fn read_lines(&self) -> Vec<String>{
        let buf_reader = BufReader::new(&self.file);
        buf_reader.lines()
            .map(|l| l.expect("Couldn't Parse Line"))
            .collect()
    }

    pub fn save(&self){} // TODO
}
