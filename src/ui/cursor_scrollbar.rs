use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};

use crate::game::Position;
use crate::ui::{Cursor, UI_OFFSET};

#[derive(Default)]
pub struct CursorScrollbar {
    offset: Position,
    vertical_scroll_state: ScrollbarState,
    horizontal_scroll_state: ScrollbarState
}

impl CursorScrollbar {

    pub fn offset(&self) -> (u16, u16) {
        (self.offset.y as u16, self.offset.x as u16)
    }

    pub fn update_scrollbars(&mut self, board_render_area: Rect, cursor: &Cursor) {
        let rows_shown = board_render_area.height - UI_OFFSET;
        let columns_shown = board_render_area.width - UI_OFFSET;

        let cursor_position = &cursor.position;

        let relative_cursor_y = cursor_position.y - self.offset.y;
        if relative_cursor_y as u16 >= rows_shown {
            self.vertical_scroll_state = self.vertical_scroll_state.position(cursor_position.y as usize);
            self.offset.y += 1;
        }
        if relative_cursor_y < 0 {
            self.vertical_scroll_state = self.vertical_scroll_state.position(cursor_position.y as usize);
            self.offset.y -= 2;
        }

        let relative_cursor_x = cursor_position.x - self.offset.x;
        if relative_cursor_x as u16 >= columns_shown {
            self.vertical_scroll_state = self.vertical_scroll_state.position(cursor_position.x as usize);
            self.offset.x += 1;
        }
        if relative_cursor_x < 0 {
            self.vertical_scroll_state = self.vertical_scroll_state.position(cursor_position.x as usize);
            self.offset.x -= 2;
        }
    }

    pub fn render_scrollbars(&mut self, frame: &mut Frame, display_width: u16, display_height: u16, width: u16, height: u16) {
        let frame_size = frame.size();
        let remaining_width = display_width.saturating_sub(width);
        let remaining_height = display_height.saturating_sub(height);

        if remaining_width > 0 {
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
                frame_size,
                &mut self.horizontal_scroll_state
                    .viewport_content_length((width - remaining_width) as usize)
                    .content_length(width as usize)
            )
        }
        if remaining_height > 0 {
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                frame_size,
                &mut self.vertical_scroll_state
                    .viewport_content_length((height - remaining_height) as usize)
                    .content_length(height as usize)
            );
        }
    }
}