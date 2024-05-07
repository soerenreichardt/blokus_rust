use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::game::{Board, Game, Position, State};
use crate::ui::{AppEvent, BLOCK, Cursor, Module, ModuleKind, RenderCanvas, UI_OFFSET};
use crate::ui::cursor_scrollbar::CursorScrollbar;

pub struct BoardDisplay {
    cursor: Cursor,
    cursor_scrollbar: CursorScrollbar,
    enabled: bool
}

impl BoardDisplay {
    pub fn new(width: usize, height: usize) -> Self {
        let cursor = Cursor::new(width as i32, height as i32);
        BoardDisplay {
            cursor,
            cursor_scrollbar: CursorScrollbar::default(),
            enabled: true
        }
    }
}

impl Module for BoardDisplay {
    fn update(&mut self, event: AppEvent) {
        if !self.enabled {
            if let AppEvent::PieceSelected(_) = event { self.enabled = true }
        } else {
            match event {
                AppEvent::MoveUp => self.cursor.move_up(),
                AppEvent::MoveDown => self.cursor.move_down(),
                AppEvent::MoveLeft => self.cursor.move_left(),
                AppEvent::MoveRight => self.cursor.move_right(),
                AppEvent::OpenPieceSelection => self.enabled = false,
                _ => ()
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, game: &mut Game) {
        let display_width = ((game.width() as u16) * 2) + UI_OFFSET;
        let display_height = game.height() as u16 + UI_OFFSET;

        let width = display_width.min(area.width);
        let height = display_height.min(area.height);
        let board_render_area = Rect { x: area.x, y: area.y, width, height};
        self.cursor_scrollbar.update_scrollbars(board_render_area, &self.cursor);
        self.cursor_scrollbar.render_scrollbars(frame, display_width, display_height, width, height);

        let mut lines = game.board.render();

        if self.enabled {
            let cursor_position = &self.cursor.position;
            lines[cursor_position.y as usize].spans[cursor_position.x as usize] = Span::styled(BLOCK, Style::default().fg(Color::Red));
        }

        let border_color = if self.enabled { Color::default() } else { Color::Gray };
        frame.render_widget(
            Paragraph::new(lines)
                .scroll(self.cursor_scrollbar.offset())
                .block(Block::default()
                    .title("Board")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(border_color))
                    .padding(Padding::zero())
                ),
            board_render_area
        );
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Board
    }
}

impl RenderCanvas for Board {
    fn render(&self) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = vec![];
        for y in 0..self.height as i32 {
            let mut line = vec![];
            for x in 0..self.width as i32 {
                let color = match self.get_state_on_position(&Position { x, y }).unwrap() {
                    State::Free => Color::Gray,
                    State::Occupied(_) => Color::Blue
                };
                line.push(Span::styled(BLOCK, Style::default().fg(color)))
            }
            lines.push(line.into());
        }
        lines
    }
}