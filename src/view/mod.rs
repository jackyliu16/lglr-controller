pub mod ConfirmedExitScreen;
mod helloworld;
mod tables;

use crate::model::fleet::Fleet;
use crate::model::position::Position;
use crate::model::{screen::Screen, App};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Clear, Tabs};
use ratatui::{
    layout::Alignment,
    prelude::{Stylize, Widget},
    style::{palette::tailwind, Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use strum::IntoEnumIterator;

/// Renders the user interface widgets.
///
///                       TOTAL SCREEN
///   ┌────────────────────────────────────────┬─────20───────┐
///   │ Tab1 Tab2 Tab3 Tab4   HEADER           │    title     1
///   ┼────────────────────────────────────────┴──────────────┼
///   │                                                       │
///   │                    INNER SCREEN                       │
///   │           (display base on tabs and apps)             │
///   │                                                       │
///   ┼───────────────────────────────────────────────────────┼
///   │                       FOOTER                          1
///   └───────────────────────────────────────────────────────┘
///
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    // TODO: DELETE THIS
    let fleet = Fleet::new(
        String::from("审判之矛"),
        String::from("1 号舰队"),
        Position { x: 2578, y: 2905 },
        450,
        3850,
        "".to_string(),
    );
    app.fleet_list.insert(0, fleet);

    use Constraint::{Length, Min};
    let vertical_chunks = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [header_area, inner_area, footer_area] = vertical_chunks.areas(frame.area());

    let horizontal = Layout::horizontal([Min(0), Length(20)]);
    let [tabs_area, title_area] = horizontal.areas(header_area);

    render_tabs(app, tabs_area, frame.buffer_mut());
    render_title(title_area, frame.buffer_mut());
    render_inner(app, inner_area, frame);
    render_footer(app, footer_area, frame.buffer_mut());
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

fn render_tabs(app: &App, area: Rect, buf: &mut Buffer) {
    // Use the strum library to get all the elements of Screen
    let titles = Screen::iter()
        .filter(|val| *val != Screen::ConfirmedExitScreen)
        .map(|name| {
            Line::from(format!("  {name}  "))
                .fg(tailwind::SLATE.c200)
                .bg(tailwind::BLUE.c900)
        });
    let highlight_style = (Color::default(), tailwind::BLUE.c500);
    let select_tab_index = app.curr_screen as usize;
    Tabs::new(titles)
        .highlight_style(highlight_style)
        .select(select_tab_index)
        .padding("", "")
        .divider(" ")
        .render(area, buf);
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "LGLR Commander Control TUI".bold().render(area, buf);
}

fn render_inner(app: &App, area: Rect, frame: &mut Frame) {
    match app.curr_screen {
        Screen::ConfirmedExitScreen => ConfirmedExitScreen::render(frame),
        _ => {
            // helloworld::render(area, frame);
            let mut tab = tables::Table::new(
                app.fleet_list.clone(),
                vec![
                    String::from("Belonger"),
                    String::from("name"),
                    String::from("position"),
                    String::from("conventional speed"),
                    String::from("curvature speed"),
                    String::from("info"),
                ],
                vec![
                    Constraint::Length(12),
                    Constraint::Length(12),
                    Constraint::Length(12),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Min(12),
                ],
            );
            frame.render_stateful_widget(
                &mut tab,
                area,
                &mut (tab.table_state.clone(), tab.scrollbar_state.clone()),
            );
        }
    }
}

fn render_footer(app: &App, area: Rect, buf: &mut Buffer) {
    let footer_msg = match app.curr_screen {
        Screen::InstallFleetInfo => "tab to switch between fleet and target | Ctrl+N to next",
        Screen::InstallTargetInfo => {
            "tab to switch between fleet and target | Ctrl+N to next | Ctrl+P to previous"
        }
        Screen::SelectTargetAndFleet => {
            "Ctrl+[N]ext | Ctrl+[P]revious | Ctrl+[L/R] for switch between blocks | Enter to select"
        }
        Screen::FleetRunningTimeScreen => "Ctrl+N to next screen | Ctrl+P to previous screen",
        Screen::FleetDepartureCountdown => "Ctrl+N to next screen | Ctrl+P to previous screen",
        Screen::ConfirmedExitScreen => {
            "Press 'Y' to exit the application or 'N' to return to the previous page."
        }
    };

    Line::raw(footer_msg).centered().render(area, buf);
}
