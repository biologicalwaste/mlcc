use app::App;
use input::key;
use std::io::Result;
use ui::{render, ui_enter, ui_leave};

mod app;
mod cue;
mod input;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ui_enter()?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| render(f, app.clone()))?;

        if let Some(key) = key()? {
            match key {
                'q' => app.change_state(app::AppState::Quit),
                'c' => app.change_state(app::AppState::ChangeDisplay),
                _ => (),
            }
        }
        match app.state {
            app::AppState::Quit => break,
            app::AppState::ChangeDisplay => {
                app.change_display_state();
                app.change_state(app::AppState::Awaiting);
            }
            app::AppState::Awaiting => (),
        }
    }

    ui_leave(terminal)?;
    Ok(())
}
