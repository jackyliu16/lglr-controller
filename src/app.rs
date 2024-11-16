use std::error;
use crate::screen::Screen;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub curr_screen: Screen,
    pub prev_screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            prev_screen: Screen::InstallFleetInfo,
            curr_screen: Screen::InstallFleetInfo,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Copy Original curr_screen to prev and set as next_screen
    pub fn change_screen(&mut self, next_screen: Screen) {
        self.prev_screen = self.curr_screen;
        self.curr_screen = next_screen;
    }
}
