use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};

use crate::ui::{Cursor, UI_OFFSET};

#[derive(Default)]
pub struct VerticalScrollBar {
    offset: u16,
    scrollbar_state: ScrollbarState,
    enabled: bool
}

impl VerticalScrollBar {
    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn update_scrollbar(&mut self, board_render_area: Rect, cursor: &Cursor) {
        let rows_displayed = board_render_area.height - UI_OFFSET;

        if rows_displayed < cursor.area.height {
            // TODO: switch to warning display mode
            panic!("Display too small!")
        }

        // scroll up
        if (rows_displayed + self.offset) < (cursor.area.y + cursor.area.height) {
            self.offset = (cursor.area.y + cursor.area.height) - rows_displayed;
            self.scrollbar_state = self.scrollbar_state.position(cursor.area.y as usize + 1);
        }

        // scroll down
        if cursor.area.y < self.offset {
            let diff = self.offset - cursor.area.y;
            self.offset -= diff;
            self.scrollbar_state = self.scrollbar_state.position(cursor.area.y as usize + 1);
        }
    }

    pub fn render_scrollbar(&mut self, frame: &mut Frame, content_height: u16, widget_area: Rect) {
        let remaining_height = content_height.saturating_sub(widget_area.height);

        self.enabled = remaining_height > 0;
        if self.enabled {
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                widget_area,
                &mut self.scrollbar_state
                    .viewport_content_length(widget_area.height as usize)
                    .content_length(content_height as usize)
            );
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}