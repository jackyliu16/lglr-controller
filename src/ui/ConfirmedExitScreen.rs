use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use crate::app::App;
use crate::ui::centered_rect;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Clear, frame.area());
    let popup_block = Block::default()
        .title("Exit Confirmation")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    let exit_text = Text::styled(
        "Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),
    );
    // the `trim: false` will stop the text from being cut off when over the edge of the block
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}