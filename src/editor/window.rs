use std::io::{stdout,BufWriter, Stdout, Write};
use crossterm::{cursor::*, execute, terminal::*};
use std::cmp::min;

pub struct Window{
    top: usize,
    bottom: usize,
    lines: usize,
    update: bool,
    write_buffer: BufWriter<Stdout>,
}

impl Window{
    pub fn new(lines: usize) -> Self {
        let height : usize = (size().expect("size didn't work").1 - 1).into();
        Window {
            top: 0,
            bottom: min(height, lines),
            lines,
            update: true,
            write_buffer: BufWriter::new(stdout()),
        }
    }

    pub fn refresh(&mut self, data: & Vec<String>){
        // update window values
        if !self.update {
            return; 
        }

        // Save cursor clear terminal
        execute!(
            stdout(),
            SavePosition,
            Clear(ClearType::All),
            MoveTo(0, 0),
        ).expect("execute!() Failed");

        // add lines to buffer
        let rows: usize = self.bottom - self.top;
        for (i, line) in data.into_iter().skip(self.top.into()).take(rows).enumerate() {
           write!(
                self.write_buffer,
                "{:>3} {}\r\n",
                self.top + i + 1,
                line
            ).expect("write!() Failed");
        }

        // prints to standard out
        self.write_buffer.flush().expect("flush() Failed");

        execute!(
            stdout(),
            RestorePosition,
        ).expect("execute1() Failed");

        self.update = false;
    }

    pub fn status_bar(&mut self, status_line : String){
        // move to line to print status bar
        execute!(
            stdout(),
            SavePosition,
            MoveTo(0,self.bottom.try_into().unwrap()),
            Clear(ClearType::CurrentLine),
        ).expect("execute!() Failed");

        write!(self.write_buffer, "{}", status_line).expect("write!() Failed");
        self.write_buffer.flush().expect("flush() Failed");

        execute!(
            stdout(),
            RestorePosition,
        ).expect("execute!() Failed");


    }

    pub fn resize(&mut self, _width: usize, height: usize) {
        let rows: usize = self.bottom - self.top;
        match height{
            h if h < rows => self.bottom = self.top + h - 1,
            h if h > rows => self.bottom += (h - rows) - 1,
            _ => {},
        }
        
        self.update = true;
    }

    pub fn scroll_up(&mut self){
        let pos: (u16, u16) = position().expect("position() Failed");
        let cursor_pos : (usize, usize) = (usize::from(pos.0), usize::from(pos.1));
        if cursor_pos.1 > 0 {
            execute!(stdout(), MoveUp(1)).expect("MoveUp() Failed");
            return;
        }
        if self.top > 0 {
            self.top -= 1;
            self.bottom -= 1;
            self.update = true;
        }
        
    }

    pub fn scroll_down(&mut self){
        let pos: (u16, u16) = position().expect("position() Failed");
        let cursor_pos : (usize, usize) =  (usize::from(pos.0), usize::from(pos.1));
        let rows: usize = self.bottom - self.top;
        if cursor_pos.1 < rows{
            execute!(stdout(), MoveDown(1)).expect("MoveDown() Failed");
            return;
        }
        if self.bottom < self.lines{
            self.top += 1;
            self.bottom += 1;
            self.update = true;
        }
    }

    pub fn get_top(&self) -> usize{
        self.top
    }
}
