pub mod window;
pub mod file;

use file::FileIO;
use window::Window;
use std::io::{ stdout, Error };
use std::path::Path;
use crossterm::{event::*, execute, cursor::{MoveLeft, MoveRight}};

#[derive(Debug)]
#[derive(PartialEq)]
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
    file_io: FileIO,
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
            file_io: file_io,
            file: file, 
        }
    }

    pub fn event_loop(&mut self){
        loop{ 
            self.window.refresh(&self.file);
            let status_line: String = format!("{:?}",self.mode);
            self.window.status_bar(status_line);

            let event = match read(){
                Ok(event) => event,
                Err(e) => panic!("Error: {}", e),
            };

            match self.event_manager(event){
                Ok(()) => {},
                Err(e) => panic!("Error: {}", e),
            }
            if self.mode == EditorMode::EXIT {break}
        }
    }

    fn event_manager(&mut self, event: Event) -> Result<(), Error>{
        use KeyCode::*;

        if let Event::Key(KeyEvent {code, ..}) = event{
            match code{
                Up => self.window.scroll_up(),
                Down => self.window.scroll_down(),
                Left => execute!(stdout(), MoveLeft(1))?,
                Right => execute!(stdout(), MoveRight(1))?,
                Esc => self.mode = EditorMode::EXIT,
                Char(c) => match c{
                    'i' => self.mode = EditorMode::INSERT,
                    'v' => self.mode = EditorMode::VISUAL,
                    ':' => self.mode = EditorMode::COMMAND,
                    _ => {},
                }, 
                _ => {},
            }
        }else if let Event::Resize(width,height) = event{
            self.window.resize(width.into(), height.into());
        }
        Ok(())
    }

}
