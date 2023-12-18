use std::io::Result;

use ui::{key, ui_start, ui_stop};

mod ui;

fn main() -> Result<()> {
    let mut terminal = ui_start()?;
    let mut should_quit = false;
    while !should_quit {
        terminal.hello_world()?;
        if key('q')? {
            should_quit = true;
        }
    }
    ui_stop()?;
    Ok(())
}
