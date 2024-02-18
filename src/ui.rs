use std::{
    io::{Result, Stdout},
    ops::{Add, AddAssign},
    time::Instant,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
};

use itertools::Itertools;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::app::{App, AppState, DisplayState};

pub fn ui_enter() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetTitle("Miraculous Lighting Control Console")
    )?;
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
    let timer = Instant::now();

    let outer_layout = Layout::new()
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .direction(Direction::Vertical)
        .margin(1)
        .split(f.size());
    let inner_layout = Layout::new()
        .constraints([Constraint::Min(10), Constraint::Length(20)])
        .direction(Direction::Horizontal)
        .split(outer_layout[0]);
    let cues_messages_layout = Layout::new()
        .constraints([Constraint::Percentage(75), Constraint::Length(10)])
        .direction(Direction::Vertical)
        .split(inner_layout[1]);

    let mut command_line_title = String::from("Command ");
    match app.state {
        AppState::Normal => command_line_title = command_line_title.add("[NORMAL]"),
        AppState::Input => command_line_title = command_line_title.add("[INPUT]"),
        _ => command_line_title = command_line_title.add("[]"),
    }

    f.render_widget(
        Paragraph::new(app.command.value()).block(
            Block::default()
                .title(command_line_title)
                .borders(Borders::all()),
        ),
        outer_layout[1],
    );

    let channels_block = Block::default()
        .title("Channels [LIVE]")
        .borders(Borders::all());
    let channels_block_area = channels_block.inner(inner_layout[0]);

    let channels_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5); 11])
        .split(channels_block_area);

    let num_columns: usize = { channels_block_area.width / 9 }.into();

    let cols = {
        let mut cols = Vec::new();
        for _ in 1..=num_columns {
            cols.push(Constraint::Min(9));
        }
        cols
    };

    let channels = channels_rows
        .iter()
        .flat_map(|&area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(cols.clone())
                .split(area)
                .iter()
                .copied()
                .collect_vec()
        })
        .collect_vec();

    f.render_widget(channels_block, inner_layout[0]);

    for (mut i, area) in channels.iter().enumerate() {
        i = i.add(app.table_offset * num_columns);
        f.render_widget(
            Paragraph::new(if let Some(a) = app.universe.get(i) {
                a.to_string()
            } else {
                break;
            })
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .title(i.to_string())
                    .title_style(Style::default().black().on_white())
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded),
            ),
            *area,
        )
    }

    let render_time = timer.elapsed().as_millis();

    f.render_widget(
        List::new([
            ListItem::new(render_time.to_string()),
            ListItem::new(app.frame_time.to_string()),
        ])
        .block(Block::default().title("Cue List").borders(Borders::all())),
        cues_messages_layout[0],
    );
    f.render_widget(
        List::new(
            app.messages
                .iter()
                .map(|i| ListItem::new(i.to_string()))
                .collect_vec(),
        )
        .block(Block::default().title("Messages").borders(Borders::all())),
        cues_messages_layout[1],
    )
}
