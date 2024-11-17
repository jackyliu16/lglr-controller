pub mod InstallFleetInfo;
pub mod ConfirmedExitScreen;

use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use crate::model::{App, screen::Screen};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    match app.curr_screen {
        Screen::ConfirmedExitScreen => ConfirmedExitScreen::render(app, frame),
        _ => {
            frame.render_widget(
                Paragraph::new(Text::from("Hello, World !"))
                    .block(
                        Block::bordered()
                            .title("Template")
                            .title_alignment(Alignment::Center)
                            .border_type(BorderType::Rounded),
                    )
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                    .centered(),
                frame.area(),
            );
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
/// return the center chunk of (x, y) box
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}