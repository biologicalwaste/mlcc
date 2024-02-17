use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::app::App;

pub fn key() -> Result<Option<KeyCode>> {
    if event::poll(std::time::Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            return Ok(Some(key.code));
        }
    }
    Ok(None)
}

impl App {
    pub fn command_input(&mut self, keycode: KeyCode) -> Result<()> {
        self.command
            .handle_event(&Event::Key(KeyEvent::new(keycode, KeyModifiers::empty())));
        Ok(())
    }
}
