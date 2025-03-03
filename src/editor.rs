pub mod window;
pub mod file;

use file::FileIO;
use window::Window;
use std::io::BufWriter;
use std::io;
use std::io::Stdout;
use std::path::Path;
use crossterm::{event::*};

enum EditorMode{
    INSERT,
    NORMAL,
    VISUAL,
    COMMAND,
    EXIT
}

pub struct Editor{
    mode: EditorMode,
    window: Window,
    writeBuffer: BufWriter<Stdout>,
    fileIO : FileIO, 
    file: Vec<String>,
}

impl Editor{
    pub fn new(file_path: &Path) -> Self{
        let file_io: FileIO = FileIO::new(file_path);
        let file = file_io.read_lines();
        let lines = file.len();
        Editor{
            mode: EditorMode::NORMAL,
            window: Window::new(lines),
            writeBuffer: BufWriter::new(io::stdout()),
            fileIO: file_io,
            file: file, 
        }
    }
    pub fn event_loop(&mut self){
        loop{ 
            self.window.refresh(&self.file,&mut self.writeBuffer);
            let event = match read(){
                Ok(event) => event,
                Err(e) => panic!("Error: {}", e),
            };
            self.event_manager(event);
            match self.mode{
                EditorMode::EXIT => break,
                _ => {},
            }

        }
    }

    fn event_manager(&self, _event: Event){
    }

}
