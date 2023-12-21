#[derive(Clone)]
pub struct App {
    pub state: AppState,
    pub universe: [u8; 512],
    pub display_state: DisplayState,
    pub table_offset: u8,
    pub command: String,
}

#[derive(Clone)]
pub enum AppState {
    Quit,
    ChangeDisplay,
    Awaiting,
}

#[derive(Clone)]
pub enum DisplayState {
    Universe,
    Channels,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Awaiting,
            universe: [255; 512],
            display_state: DisplayState::Universe,
            table_offset: 0,
            command: String::new(),
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
}
