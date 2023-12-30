use app::App;
use input::key;
use std::{io::Result, time::Instant};
use ui::{render, ui_enter, ui_leave};

mod app;
mod cue;
mod input;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ui_enter()?;

    let mut app = App::new();

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
                    match key.as_str() {
                        "q" => app.change_state(app::AppState::Quit),
                        "i" => app.change_state(app::AppState::Input),
                        "j" => app.scroll_up(),
                        "k" => app.scroll_down(),
                        _ => (),
                    }
                }
            }
            app::AppState::Input => {
                app.command_input()?;
                if let Some(key) = key()? {
                    if &key == "esc" {
                        app.change_state(app::AppState::Normal);
                    }
                }
            }
        }

        app.frame_time = timer.elapsed().as_millis();

        terminal.draw(|f| render(f, app.clone()))?;

        if timer.elapsed().as_millis() < 8 {
            std::thread::sleep(std::time::Duration::from_millis(8) - timer.elapsed());
        }
    }

    ui_leave(terminal)?;
    Ok(())
}
