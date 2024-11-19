use std::error;

pub mod fleet;
use fleet::Fleet;
pub mod position;
pub mod screen;
use screen::Screen;
use crate::model::position::Position;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// The current screen loading
    pub curr_screen: Screen,
    pub prev_screen: Screen, // TODO: it is possible to remove it by popup or something?
    /// The list contains all fleet infos
    pub fleet_list: Vec<Fleet>,
}

impl Default for App {
    fn default() -> Self {
        let fleet = Fleet::new(     // TODO REMOVE THIS TEMP VAR
            String::from("审判之矛"),
            String::from("1 号舰队"),
            Position::new(3440, 2922),
            540,
            3245,
            String::new(),
        );
        Self {
            running: true,
            prev_screen: Screen::InstallFleetInfo,
            curr_screen: Screen::InstallFleetInfo,
            fleet_list: vec![fleet],    // TODO REMOVE THIS TEMP VAR
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
