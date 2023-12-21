use std::io::{Result, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::app::{App, DisplayState};

pub fn ui_enter() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn ui_leave(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn render(f: &mut Frame, app: App) {
    let outer_layout = Layout::new()
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .direction(Direction::Vertical)
        .margin(1)
        .split(f.size());
    let inner_layout = Layout::new()
        .constraints([Constraint::Min(10), Constraint::Length(20)])
        .direction(Direction::Horizontal)
        .split(outer_layout[0]);

    let table_to_display = match app.display_state {
        DisplayState::Universe => {
            let mut universe_to_display = String::new();
            for channel in app.universe {
                universe_to_display = universe_to_display + " | " + &channel.to_string();
            }
            Paragraph::new(universe_to_display)
                .wrap(Wrap { trim: true })
                .block(Block::default().title("Universe").borders(Borders::all()))
        }
        DisplayState::Channels => Paragraph::new("Working on it :3")
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Channels").borders(Borders::all())),
    };

    f.render_widget(table_to_display, inner_layout[0])
}
