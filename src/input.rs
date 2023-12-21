use crossterm::event::{self, Event, KeyCode};
use std::io::Result;

pub fn key() -> Result<Option<char>> {
    if event::poll(std::time::Duration::from_millis(4))? {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char(c) = key.code {
                return Ok(Some(c))
            }
        }
    }
    Ok(None)
}
