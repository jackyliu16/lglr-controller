use std::cmp::PartialEq;
use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::screen::Screen;


/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            // app.quit();
            app.change_screen(Screen::ConfirmedExitScreen);
        },
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        },
        // Other handlers you could add here.
        KeyCode::Char('y') if app.curr_screen == Screen::ConfirmedExitScreen => {
            app.quit();
        },
        KeyCode::Char('n') if app.curr_screen == Screen::ConfirmedExitScreen => {
            app.curr_screen = app.prev_screen.clone();
        },

        _ => {}
    }
    Ok(())
}
