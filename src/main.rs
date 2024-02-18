use app::App;
use crossterm::event::KeyCode;
use input::key;
use std::{io::Result, sync::mpsc::channel, thread, time::Instant};
use ui::{render, ui_enter, ui_leave};

mod app;
mod command;
mod cue;
mod input;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ui_enter()?;

    let mut app = App::new();
    let (app_tx, app_rx) = channel();

    let renderer = thread::spawn(move || -> Result<()> {
        loop {
            let app_renderer: App = app_rx.recv().unwrap();
            terminal.draw(|f| render(f, app_renderer.clone()))?;
            match &app_renderer.state {
                app::AppState::Quit => {
                    ui_leave(terminal)?;
                    break Ok(());
                }
                _ => (),
            }
        }
    });

    loop {
        let timer = Instant::now();

        match &app.state {
            app::AppState::Quit => break,
            app::AppState::ChangeDisplay => {
                app.change_display_state();
                app.change_state(app::AppState::Normal);
            }
            app::AppState::Normal => {
                if let Some(key) = key()? {
                    match key {
                        KeyCode::Char('q') => app.change_state(app::AppState::Quit),
                        KeyCode::Char('i') => app.change_state(app::AppState::Input),
                        KeyCode::Char('j') => app.scroll_up(),
                        KeyCode::Char('k') => app.scroll_down(),
                        KeyCode::Char('c') => app.clear_command(),
                        _ => (),
                    }
                }
            }
            app::AppState::Input => {
                if let Some(key) = key()? {
                    match key {
                        KeyCode::Esc => app.change_state(app::AppState::Normal),
                        _ => app.command_input(key)?,
                    }
                }
            }
        }

        app.frame_time = timer.elapsed().as_millis();

        app_tx.send(app.clone()).unwrap();

        if timer.elapsed().as_millis() < 8 {
            std::thread::sleep(std::time::Duration::from_millis(8) - timer.elapsed());
        }
    }
    let _ = renderer.join();
    Ok(())
}
