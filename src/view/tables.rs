use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::Color;
use ratatui::widgets::{ScrollbarState, TableState};
use table_color::TableColors;
use crate::model::fleet::Fleet;

const ITEM_HEIGHT: usize = 4;

// BC we will use render_stateful_widget, it is impossible to use `impl Widget for Table` to impl it
struct Table {
    table_state: TableState,
    scrollbar_state: ScrollbarState,
    items: &'static Vec<Fleet>,
    col_name: Vec<&'static str>,
    col_len: Vec<usize>,
    color: TableColors,
}

impl Table {
    fn new(items: &'static Vec<Fleet>, col_name: Vec<&'static str>, col_len: Vec<usize>) -> Self {
        Self {
            table_state: TableState::default().with_selected(0),
            scrollbar_state: ScrollbarState::new((items.len() - 1) * ITEM_HEIGHT),
            items,
            col_name,
            col_len,
            color: TableColors::default(),
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
            },
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
            },
            _ => 0,
        };
        self.table_state.select(Some(i));
        self.scrollbar_state = self.scrollbar_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_col(&mut self) { self.table_state.select_next_column(); }
    pub fn prev_col(&mut self) { self.table_state.select_previous_column(); }

    pub fn set_color(&mut self, color: TableColors) { self.color = color; }

}

mod table_color {
    use ratatui::style::Color;
    use ratatui::style::palette::tailwind;

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