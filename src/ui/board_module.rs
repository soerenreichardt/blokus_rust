use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::game::{Board, Game, Piece, Position, State};
use crate::ui::{AppEvent, BLOCK, Cursor, Module, ModuleKind, RenderCanvas, UI_OFFSET};
use crate::ui::scrollbars::VerticalScrollBar;

pub struct BoardDisplay {
    cursor: Cursor,
    vertical_scrollbar: VerticalScrollBar,
    enabled: bool,
    selected_piece: Option<IndexedPiece>
}

struct IndexedPiece {
    piece: Piece,
    index: usize
}

impl BoardDisplay {
    pub fn new(width: u16, height: u16) -> Self {
        let cursor = Cursor::simple(width, height);
        BoardDisplay {
            cursor,
            vertical_scrollbar: VerticalScrollBar::default(),
            enabled: true,
            selected_piece: None
        }
    }

    pub fn render_cursor(&mut self, lines: &mut [Line<'_>]) {
        match &self.selected_piece {
            Some(indexed_piece) => self.render_piece_cursor(lines, indexed_piece),
            None => self.render_simple_cursor(lines)
        }
    }

    fn render_piece_cursor(&self, lines: &mut [Line<'_>], indexed_piece: &IndexedPiece) {
        let piece = &indexed_piece.piece;
        let cursor_position = &self.cursor.area;
        for block in piece.blocks.iter() {
            let line = (cursor_position.y + block.y) as usize;
            let column = (cursor_position.x + block.x) as usize;
            lines[line].spans[column] = Span::styled(BLOCK, Style::default().fg(Color::Red));
        }
    }

    fn render_simple_cursor(&mut self, lines: &mut [Line<'_>]) {
        let cursor_position = &self.cursor.area;
        lines[cursor_position.y as usize].spans[cursor_position.x as usize] = Span::styled(BLOCK, Style::default().fg(Color::Red));
    }

    fn select_piece(&mut self, index: usize, game: &Game) {
        let piece = game.active_player_pieces()[index].clone();

        self.cursor.area.width = piece.num_columns();
        self.cursor.area.height = piece.num_lines();
        self.selected_piece = Some(IndexedPiece { piece, index });
    }

    fn rotate_piece(&mut self) {
        if let Some(piece_index) = &mut self.selected_piece {
            piece_index.piece.rotate()
        }
    }
}

impl Module for BoardDisplay {
    fn update(&mut self, event: AppEvent, game: &mut Game) -> Option<AppEvent> {
        if !self.enabled {
            if let AppEvent::PieceSelected(piece_index) = event {
                self.enabled = true;
                self.select_piece(piece_index, game);
            }
        } else {
            match event {
                AppEvent::MoveUp => self.cursor.move_up(),
                AppEvent::MoveDown => self.cursor.move_down(),
                AppEvent::MoveLeft => self.cursor.move_left(),
                AppEvent::MoveRight => self.cursor.move_right(),
                AppEvent::OpenPieceSelection => self.enabled = false,
                AppEvent::Rotate => self.rotate_piece(),
                _ => ()
            }
        }

        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, game: &mut Game) {
        let display_width = (game.width() * 2) + UI_OFFSET;
        let display_height = game.height() + UI_OFFSET;

        let width = display_width.min(area.width);
        let height = display_height.min(area.height);
        let board_render_area = Rect { x: area.x, y: area.y, width, height};
        self.vertical_scrollbar.update_scrollbar(board_render_area, &self.cursor);

        let mut lines = game.board.render();

        if self.enabled {
            self.render_cursor(&mut lines);
        }

        let border_color = if self.enabled { Color::default() } else { Color::Gray };

        frame.render_widget(
            Paragraph::new(lines)
                .scroll((self.vertical_scrollbar.offset(), 0))
                .block(Block::default()
                    .title("Board")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(border_color))
                    .padding(Padding::zero())
                ),
            board_render_area
        );

        self.vertical_scrollbar.render_scrollbar(frame, display_height, board_render_area);
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Board
    }
}

impl RenderCanvas for Board {
    fn render(&self) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = vec![];
        for y in 0..self.height {
            let mut line = vec![];
            for x in 0..self.width {
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