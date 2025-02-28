use std::io::{self, BufWriter, Stdout, Write};
use crossterm::{cursor::*, execute, terminal::*};

pub struct status_bar{
    row: u16,
    format: String,
}

impl status_bar{
    pub fn new(row: u16) -> Self{
        Self{
            row: row
        }

    } 
}








