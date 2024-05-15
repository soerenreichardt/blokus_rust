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
        let rows_shown = board_render_area.height - UI_OFFSET;

        let cursor_position = cursor.position.y;

        let relative_cursor_position = cursor_position - self.offset as i32;
        if relative_cursor_position as u16 >= rows_shown {
            self.offset += 1;
            self.scrollbar_state = self.scrollbar_state.position(cursor_position as usize + 1);
        }
        if relative_cursor_position < 0 {
            self.offset -= 2;
            self.scrollbar_state = self.scrollbar_state.position(cursor_position as usize + 1);
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