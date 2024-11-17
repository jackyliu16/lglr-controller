use crate::model::fleet::Fleet;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::palette::tailwind;
use ratatui::style::Color;
use ratatui::Frame;

impl Widget for Fleet {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
