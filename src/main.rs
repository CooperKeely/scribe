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

    open_scribe();
    
    let mut editor = Editor::new(file_path);
    editor.event_loop();
    
    close_scribe();

    Ok(())
}
    
fn open_scribe(){ 
    enable_raw_mode().expect("Error {e}: Couldn't enable raw mode"); 

    execute!(
        io::stdout(),
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
        EnableBlinking,
        SetCursorStyle::SteadyBlock,
        EnterAlternateScreen,
        SetTitle("Scribe"),
        DisableLineWrap
    ).expect("Error {e}: Couldn't create new terminal process");
}

fn close_scribe() {
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture,
        DisableBlinking,
        EnableLineWrap
    ).expect("Error {e}: Couldn't exit terminal process");

    disable_raw_mode().expect("Error {e}: Couldn't diable raw mode");
}
