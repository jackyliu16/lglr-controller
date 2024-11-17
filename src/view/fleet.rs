use ratatui::buffer::Buffer;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Color;
use ratatui::style::palette::tailwind;
use crate::model::App;

impl Widget for Fleet {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        todo!()
    }
}
