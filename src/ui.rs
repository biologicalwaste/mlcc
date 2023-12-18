use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{stdout, Result, Stdout};

pub struct UI {
    term: Terminal<CrosstermBackend<Stdout>>,
}

impl UI {
    pub fn hello_world(&mut self) -> Result<()> {
        self.term.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello world!")
                    .style(Style::new().white().on_blue())
                    .block(Block::new().title("Hiiiiiiii").borders(Borders::all())),
                area,
            )
        })?;
        Ok(())
    }
}

pub fn ui_start() -> Result<UI> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(UI { term: terminal })
}

pub fn ui_stop() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn key(key: char) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(8))? {
        if let event::Event::Key(k) = event::read()? {
            if k.kind == KeyEventKind::Press && k.code == KeyCode::Char(key) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
