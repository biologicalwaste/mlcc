use crossterm::event::{self, Event, KeyCode};
use std::io::Result;
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::app::App;

pub fn key() -> Result<Option<String>> {
    if event::poll(std::time::Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => return Ok(Some("enter".to_string())),
                KeyCode::Esc => return Ok(Some("esc".to_string())),
                KeyCode::Char(c) => return Ok(Some(c.to_string())),
                _ => (),
            }
        }
    }
    Ok(None)
}

impl App {
    pub fn command_input(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(0))? {
            let event = event::read()?;
            self.command.handle_event(&event);
        }
        Ok(())
    }
}
