use std::collections::VecDeque;

use tui_input::Input;

#[derive(Clone)]
pub struct App {
    pub state: AppState,
    pub universe: [u8; 512],
    pub display_state: DisplayState,
    pub table_offset: usize,
    pub command: Input,
    pub messages: VecDeque<String>,
    pub current_cue: f32,
    pub frame_time: u128,
}

#[derive(Clone)]
pub enum AppState {
    Quit,
    ChangeDisplay,
    Normal,
    Input,
}

#[derive(Clone)]
pub enum DisplayState {
    Universe,
    Channels,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Normal,
            universe: [255; 512],
            display_state: DisplayState::Universe,
            table_offset: 0,
            command: Input::new("".to_string()),
            messages: VecDeque::new(),
            current_cue: 0.0,
            frame_time: 0,
        }
    }

    pub fn change_state(&mut self, new_state: AppState) {
        self.state = new_state;
    }

    pub fn change_display_state(&mut self) {
        match self.display_state {
            DisplayState::Universe => self.display_state = DisplayState::Channels,
            DisplayState::Channels => self.display_state = DisplayState::Universe,
        }
    }

    pub fn set_channel(&mut self, channel: u8, value: u8) {
        self.universe[usize::from(channel)] = value;
    }

    pub fn scroll_down(&mut self) {
        self.table_offset = self.table_offset.saturating_sub(1);
    }

    pub fn scroll_up(&mut self) {
        self.table_offset = self.table_offset + 1;
    }

    pub fn get_command(&self) -> &str {
        self.command.value()
    }

    pub fn clear_command(&mut self) {
        self.command.reset();
    }

    pub fn print_message(&mut self, message: &str) {
        self.messages.push_front(message.to_string());
    }
}
