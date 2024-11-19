use crate::model::fleet::Fleet;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Color;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{
    Cell, HighlightSpacing, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
    TableState,
};
use ratatui::widgets::{Cell, HighlightSpacing, Row, ScrollbarState, TableState};
use ratatui::widgets::{ScrollbarState, TableState};
use table_color::TableColors;

const ITEM_HEIGHT: usize = 4;

// BC we will use render_stateful_widget, it is impossible to use `impl Widget for Table` to impl it
pub struct Table {
    pub table_state: TableState,
    pub scrollbar_state: ScrollbarState,
    items: Vec<Fleet>,
    col_name: Vec<String>,
    col_len: Vec<Constraint>,
    colors: TableColors,
}

impl Table {
    pub fn new(items: Vec<Fleet>, col_name: Vec<String>, col_len: Vec<Constraint>) -> Self {
        Self {
            table_state: TableState::default().with_selected(0),
            scrollbar_state: ScrollbarState::new(
                (if items.is_empty() {
                    0
                } else {
                    (items.len() - 1)
                }) * ITEM_HEIGHT,
            ),
            items,
            col_name,
            col_len,
            colors: TableColors::default(),
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scrollbar_state = self.scrollbar_state.position(i * ITEM_HEIGHT);
    }
    pub fn prev_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    0
                }
            }
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scrollbar_state = self.scrollbar_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_col(&mut self) {
        self.table_state.select_next_column();
    }
    pub fn prev_col(&mut self) {
        self.table_state.select_previous_column();
    }

    pub fn set_color(&mut self, color: TableColors) {
        self.colors = color;
    }
}

impl StatefulWidget for &mut Table {
    type State = (TableState, ScrollbarState);

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = self
            .col_name
            .clone()
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";

        let (mut table_state, mut scroll_state) = state;
        ratatui::widgets::Table::new(rows, &self.col_len)
            .header(header)
            .row_highlight_style(selected_row_style)
            .column_highlight_style(selected_col_style)
            .cell_highlight_style(selected_cell_style)
            .highlight_symbol(Text::from(vec![
                "".into(),
                bar.into(),
                bar.into(),
                "".into(),
            ]))
            .bg(self.colors.buffer_bg)
            .highlight_spacing(HighlightSpacing::Always)
            .render(area, buf, &mut table_state);

        // render_scrollbar
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .render(
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 1,
                }),
                buf,
                &mut scroll_state,
            );

        // *state = (&self.scrollbar_state, &self.table_state);
    }
}

impl Widget for &mut Table {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = self
            .col_name
            .clone()
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        ratatui::widgets::Table::new(rows, &self.col_len)
            .header(header)
            .row_highlight_style(selected_row_style)
            .column_highlight_style(selected_col_style)
            .cell_highlight_style(selected_cell_style)
            .highlight_symbol(Text::from(vec![
                "".into(),
                bar.into(),
                bar.into(),
                "".into(),
            ]))
            .bg(self.colors.buffer_bg)
            .highlight_spacing(HighlightSpacing::Always)
            .render(area, buf);
    }
}

mod table_color {
    use ratatui::style::palette::tailwind;
    use ratatui::style::Color;

    pub struct TableColors {
        pub buffer_bg: Color,
        pub header_bg: Color,
        pub header_fg: Color,
        pub row_fg: Color,
        pub selected_row_style_fg: Color,
        pub selected_column_style_fg: Color,
        pub selected_cell_style_fg: Color,
        pub normal_row_color: Color,
        pub alt_row_color: Color,
        pub footer_border_color: Color,
    }

    impl TableColors {
        const fn new(color: &tailwind::Palette) -> Self {
            Self {
                buffer_bg: tailwind::SLATE.c950,
                header_bg: color.c900,
                header_fg: tailwind::SLATE.c200,
                row_fg: tailwind::SLATE.c200,
                selected_row_style_fg: color.c400,
                selected_column_style_fg: color.c400,
                selected_cell_style_fg: color.c600,
                normal_row_color: tailwind::SLATE.c950,
                alt_row_color: tailwind::SLATE.c900,
                footer_border_color: color.c400,
            }
        }

        pub(crate) const fn default() -> Self {
            Self::new(&tailwind::BLUE)
        }
    }
}
