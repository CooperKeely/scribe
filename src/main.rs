use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, BufRead, ErrorKind};
use std::path::Path;

use crossterm::{cursor::*, event::*, execute, terminal::*};

mod window;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let _query: &String = &args[0];

    // get file path
    let file_path: &String = &args[1];

    // read file data into a vector
    let data = read_lines(file_path);

    let mut write_buffer = BufWriter::new(io::stdout());

    open_scribe().expect("open_scribe() Failed");
    // create the window handler
    let mut current_window = window::Handler::new(data.len());

    // main loop of the editor
    loop {
        current_window.refresh(& data,&mut write_buffer)?;
        if event_handler(read()?, &mut current_window)? {
            break;
        }
    }

    close_scribe().expect("close_scribe() Failed");

    Ok(())
}
// TODO correctly catch the file not found error
fn read_lines(filename: impl AsRef<Path>) -> Vec<String> 
{
    let file = match File::open(filename){
        Ok(file) => file,
        Err(E) => match E.kind() {
            ErrorKind::NotFound => match File::create(filename){
                Ok(file) => file,
                Err(e) => panic!("Error: {}", e),
            }
            e => panic!("Error: {}", e),
        } 
    };
    let buf_reader = BufReader::new(file);
    buf_reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn event_handler(read_event: Event, window: &mut window::Handler) -> io::Result<bool> {
    use KeyCode::*;

    if let Event::Key(KeyEvent {code, ..}) = read_event {
        match code{
            Up => window.scroll_up(),
            Down => window.scroll_down(),
            Left => execute!(io::stdout(), MoveLeft(1)).expect("execute!() Failed"),
            Right => execute!(io::stdout(), MoveRight(1)).expect("execute!() Failed"), 
            Esc => return Ok(true),
            _ => {}
        }
    } else if let Event::Resize(width, height) = read_event{
        window.resize(width.into(), height.into());
    }
    Ok(false)
}

fn open_scribe() -> io::Result<()> {
    match enable_raw_mode() {
        Ok(()) => (),
        Err(e) => println!("Error {e}: Couldn't enable raw mode"),
    }

    match execute!(
        io::stdout(),
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
        EnableBlinking,
        SetCursorStyle::SteadyBlock,
        EnterAlternateScreen,
        SetTitle("Scribe"),
        DisableLineWrap
    ) {
        Ok(()) => (),
        Err(e) => println!("Error {e}: Couldn't create new terminal process"),
    }
    Ok(())
}

fn close_scribe() -> io::Result<()> {
    match execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture,
        DisableBlinking,
        EnableLineWrap
    ) {
        Ok(()) => (),
        Err(e) => println!("Error {e}: Couldn't exit terminal process"),
    }

    match disable_raw_mode() {
        Ok(()) => (),
        Err(e) => println!("Error {e}: Couldn't diable raw mode"),
    }
    Ok(())
}
