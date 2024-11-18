use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn render(area: Rect, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(Text::from("Hello, World !"))
            .block(
                Block::bordered()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded)
                ,
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered()
        , area
    )
}