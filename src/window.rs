use std::io::{self, BufWriter, Stdout, Write};
use crossterm::{cursor::*, execute, terminal::*};
use std::cmp::min;

pub struct Handler {
    top: usize,
    bottom: usize,
    lines: usize,
    update: bool,
}

impl Handler {
    pub fn new(lines: usize) -> Self {
        let height : usize = (size().expect("size didn't work").1 - 1).into();
        Self {
            top: 0,
            bottom: min(height, lines),
            lines,
            update: true,
        }
    }

    pub fn refresh(
        &mut self,
        data: & Vec<String>,
        write_buffer: &mut BufWriter<Stdout>
    ) -> io::Result<()> {
        // update window values
        if !self.update {
            return Ok(());
        }

        // Save cursor clear terminal
        execute!(
            io::stdout(),
            SavePosition,
            Clear(ClearType::All),
            MoveTo(0, 0),
        ).expect("execute!() Failed");

        // add lines to buffer
        let rows: usize = (self.bottom - self.top).into();
        for (i, line) in data.into_iter().skip(self.top.into()).take(rows).enumerate() {
           write!(
                write_buffer,
                "{:>3} {}\r\n",
                self.top + i + 1,
                line
            ).expect("write!() Failed");
        }

        // prints to standard out
        write_buffer.flush().expect("flush() Failed");

        execute!(
            io::stdout(),
            RestorePosition,
        ).expect("execute1() Failed");

        self.update = false;
        Ok(())
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
            execute!(io::stdout(), MoveUp(1)).expect("MoveDown() Failed");
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
            execute!(io::stdout(), MoveDown(1)).expect("MoveDown() Failed");
            return;
        }
        if self.bottom < self.lines{
            self.top += 1;
            self.bottom += 1;
            self.update = true;
        }
    }
}
