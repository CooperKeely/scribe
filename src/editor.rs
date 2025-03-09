pub mod window;
pub mod file;

use file::FileIO;
use window::Window;
use std::io::{ Error };
use std::path::Path;
use crossterm::{event::*, cursor::{position}};

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
            use EditorMode::*; 
            match code{
                Up         => self.window.scroll_up(),
                Down       => self.window.scroll_down(),
                Left       => self.window.scroll_left(), 
                Right      => self.window.scroll_right(), 
                Esc        => self.esc_event_handler(),
                Backspace  => self.backspace_event_handler(), 
                Char('i') if self.mode == NORMAL => self.mode = INSERT,
                Char('v') if self.mode == NORMAL => self.mode = VISUAL,
                Char(':') if self.mode == NORMAL => self.mode = COMMAND,
                Char('h') if self.mode == NORMAL => self.window.scroll_left(),
                Char('j') if self.mode == NORMAL => self.window.scroll_down(),
                Char('k') if self.mode == NORMAL => self.window.scroll_up(),
                Char('l') if self.mode == NORMAL => self.window.scroll_right(),
                Char(c)    => self.char_event_handler(c),
                _ => {},
            }
        }else if let Event::Resize(width,height) = event{
            self.window.resize(width.into(), height.into());
        }
        Ok(())
    }
    
    fn backspace_event_handler(&mut self){
        if self.mode == EditorMode::INSERT{
            // get the current cursor positon
            let pos: (u16, u16) = position().expect("couldn't get position");
            let (mut col, mut row): (usize, usize) = (usize::from(pos.0), usize::from(pos.1));
            
            // get the line index for the file 
            row = row + self.window.get_top(); 
            col -= 4; 
            // get the line 
            let mut line : String = self.file.remove(row);
            
            // get the chars out of the array and remove the char
            let chars : Vec<(usize, char)> = line.char_indices().collect();
            if let Some((char_index,_)) = chars.get(col-1){
                line.remove(*char_index);
            }
           
            // reinsert the new line
            self.file.insert(row, line);
            self.window.scroll_left();
            self.window.update_window();
        }       
    }
    
    fn esc_event_handler(&mut self){
        if self.mode != EditorMode::NORMAL {
            self.mode = EditorMode::NORMAL;
        }else {
            self.mode = EditorMode::EXIT;
        }
    }
    
    fn char_event_handler(&mut self, c : char){
        match self.mode{
            EditorMode::INSERT => self.insert_char(c),
            EditorMode::VISUAL => {},
            EditorMode::COMMAND => {},
            EditorMode::NORMAL => {},
            _ => {},
        }
    }

    fn insert_char(&mut self, c : char){
        // get the current positon of the cursor
        let pos : (u16, u16) = position().expect("unable to read positon");
        let (mut col, mut row): (usize, usize) = (usize::from(pos.0), usize::from(pos.1));
        
        // get current positon within file
        row = row + self.window.get_top();
        col -= 3;

        // get line from file:window
        let mut line : String = self.file.remove(row);

        // get the chars out of the array and remove the char
        let chars : Vec<(usize, char)> = line.char_indices().collect();
        if let Some((char_index,_)) = chars.get(col-1){
            line.insert(*char_index, c);
        }
       
        // reinsert the new line
        self.file.insert(row, line);
        self.window.scroll_right();
        self.window.update_window();

    }
}
