use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::game::{Game, Position, State};
use crate::ui::{AppEvent, Cursor, Module, UI_OFFSET};
use crate::ui::cursor_scrollbar::CursorScrollbar;

const BLOCK: &str = "██";

pub struct BoardDisplay {
    cursor: Cursor,
    cursor_scrollbar: CursorScrollbar
}

impl BoardDisplay {
    pub fn new(width: usize, height: usize) -> Self {
        let cursor = Cursor::new(width as i32, height as i32);
        BoardDisplay {
            cursor,
            cursor_scrollbar: CursorScrollbar::default()
        }
    }
}

impl Module for BoardDisplay {
    fn update(&mut self, event: AppEvent) {
        match event {
            AppEvent::MoveUp => self.cursor.move_up(),
            AppEvent::MoveDown => self.cursor.move_down(),
            AppEvent::MoveLeft => self.cursor.move_left(),
            AppEvent::MoveRight => self.cursor.move_right(),
            _ => ()
        }
    }

    fn render(&mut self, frame: &mut Frame, game: &mut Game) {
        let display_width = ((game.width() as u16) * 2) + UI_OFFSET;
        let display_height = game.height() as u16 + UI_OFFSET;

        let width = display_width.min(frame.size().width);
        let height = display_height.min(frame.size().height);
        let board_render_area = Rect { x: 0, y: 0, width, height};
        self.cursor_scrollbar.update_scrollbars(board_render_area, &self.cursor);
        self.cursor_scrollbar.render_scrollbars(frame, display_width, display_height, width, height);

        let board = &game.board;
        let width = board.width as i32;
        let height = board.height as i32;

        let cursor_position = self.cursor.position;
        let mut lines: Vec<Line<'_>> = vec![];
        for y in 0..height {
            let mut line = vec![];
            for x in 0..width {
                if cursor_position.x == x && cursor_position.y == y {
                    line.push(Span::styled(BLOCK, Style::default().fg(Color::Red)))
                }
                let color = match board.get_state_on_position(Position { x, y }).unwrap() {
                    State::Free => Color::Gray,
                    State::Occupied(_) => Color::Blue
                };
                line.push(Span::styled(BLOCK, Style::default().fg(color)))
            }
            lines.push(line.into());
        }

        frame.render_widget(
            Paragraph::new(lines).scroll(self.cursor_scrollbar.offset()).block(Block::default().borders(Borders::ALL).padding(Padding::zero())),
            board_render_area
        );
    }
}