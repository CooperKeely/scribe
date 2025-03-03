mod editor;

use editor::Editor;

use std::io;
use std::env;
use std::path::Path;
use crossterm::{cursor::*, event::*, execute, terminal::*};



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let _query: &String = &args[0];

    // get file path
    let file_path: &Path = Path::new(&args[1]);

    open_scribe().expect("open_scribe() Failed");
    let mut editor = Editor::new(file_path);
    editor.event_loop();
    close_scribe().expect("close_scribe() Failed");

    Ok(())
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
