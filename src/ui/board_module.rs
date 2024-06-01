use std::collections::HashMap;

use ratatui::Frame;
use ratatui::layout::{Corner, Rect};
use ratatui::prelude::{Color, Line, Span, Style, Stylize};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::game::{Board, Game, Piece, Player, Position};
use crate::ui::{AppEvent, BLOCK, Cursor, Module, ModuleKind, RenderCanvas, SHADED_BLOCK, UI_OFFSET};
use crate::ui::scrollbars::VerticalScrollBar;

pub struct BoardDisplay {
    cursors: [Cursor; 4],
    cursor: Cursor,
    index: usize,
    vertical_scrollbar: VerticalScrollBar,
    state: State
}

struct IndexedPiece {
    piece: Piece,
    index: usize,
    rotations: u16
}

enum State {
    Default,
    PieceSelected(IndexedPiece),
    Disabled
}

impl BoardDisplay {
    pub fn new(width: u16, height: u16, player_index: usize) -> Self {
        let cursors = [
            Cursor::simple(Corner::TopLeft, width, height),
            Cursor::simple(Corner::TopRight, width, height),
            Cursor::simple(Corner::BottomLeft, width, height),
            Cursor::simple(Corner::BottomRight, width, height)
        ];
        let cursor = cursors[player_index].clone();
        BoardDisplay {
            cursors,
            cursor,
            index: player_index,
            vertical_scrollbar: VerticalScrollBar::default(),
            state: State::Default
        }
    }

    pub fn render_cursor(&mut self, lines: &mut [Line<'_>], board: &Board, color_map: &HashMap<usize, (Color, Color)>, player: &Player) {
        match &self.state {
            State::PieceSelected(indexed_piece) => self.render_piece_cursor(lines, indexed_piece, board, color_map, player),
            State::Default => self.render_simple_cursor(lines),
            _ => ()
        }
    }

    fn render_piece_cursor(&self, lines: &mut [Line<'_>], indexed_piece: &IndexedPiece, board: &Board, color_map: &HashMap<usize, (Color, Color)>, player: &Player) {
        let piece = &indexed_piece.piece;
        let cursor_position = &self.cursor.area;
        for block in piece.blocks() {
            let line = (cursor_position.y + block.y) as usize;
            let column = (cursor_position.x + block.x) as usize;
            let content = match board.get_state_on_position(&Position { x: column as u16, y: line as u16 }).expect("Out of bounds") {
                crate::game::State::Free => Span::styled(BLOCK, Style::default().fg(player.secondary_color)),
                crate::game::State::Occupied(player_index) => {
                    let (color, _) = *color_map.get(&player_index).unwrap();
                    Span::styled(SHADED_BLOCK, Style::default().fg(player.color).bg(color))
                }
            };
            lines[line].spans[column] = content;
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
        self.cursor.area.x = self.cursor.area.x.clamp(0, game.width() - piece.num_columns());
        self.cursor.area.y = self.cursor.area.y.clamp(0, game.height() - piece.num_lines());
        self.state = State::PieceSelected(IndexedPiece { piece, index, rotations: 0 });
    }

    /// As pieces are centered in a rectangular bounding box, the blocks that belong to a piece
    /// are not necessarily in the top left corner of the bounding box. Pieces are rendered with
    /// this offset in mind. When rotating a piece, the cursor must be moved to counteract the
    /// offset, then the piece is rotated, and finally the cursor is moved back according to the
    /// new offset.
    fn rotate_piece(&mut self) {
        if let State::PieceSelected(indexed_piece) = &mut self.state {
            // unapply the cursor offset
            self.cursor.move_cursor(-(indexed_piece.piece.bounding_box_offset.x as i32), -(indexed_piece.piece.bounding_box_offset.y as i32));

            indexed_piece.rotate();
            // swap the width and height
            self.cursor.rotate_cursor();
            // reapply the cursor offset with the rotated piece
            self.cursor.move_cursor(indexed_piece.piece.bounding_box_offset.x as i32, indexed_piece.piece.bounding_box_offset.y as i32);
        }
    }

    fn place_piece(&mut self, game: &mut Game) -> Option<AppEvent> {
        match &self.state {
            State::PieceSelected(indexed_piece) => if game.place_piece(indexed_piece.index, indexed_piece.rotations, Position { x: self.cursor.area.x, y: self.cursor.area.y }).expect("Out of bounds") {
                self.state = State::Default;
                Some(AppEvent::PiecePlaced)
            } else {
                None
                // render failure animation
            }
            _ => None
        }
    }

    fn is_enabled(&self) -> bool {
        match self.state {
            State::Disabled => false,
            _ => true
        }
    }
}

impl Module for BoardDisplay {
    fn update(&mut self, event: AppEvent, game: &mut Game) -> Option<AppEvent> {
        if let AppEvent::PiecePlaced = event {
            let index = game.active_player_index();
            let original_cursor = &mut self.cursors[self.index];
            original_cursor.area.x = self.cursor.area.x;
            original_cursor.area.y = self.cursor.area.y;
            self.cursor = self.cursors[index].clone();
            self.index = index;
        }
        if !self.is_enabled() {
            if let AppEvent::PieceSelected(piece_index) = event {
                self.select_piece(piece_index, game);
            }
        } else {
            match event {
                AppEvent::MoveUp => self.cursor.move_up(1),
                AppEvent::MoveDown => self.cursor.move_down(1),
                AppEvent::MoveLeft => self.cursor.move_left(1),
                AppEvent::MoveRight => self.cursor.move_right(1),
                AppEvent::OpenPieceSelection => self.state = State::Disabled,
                AppEvent::Rotate => self.rotate_piece(),
                AppEvent::Select => return self.place_piece(game),
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

        let board = &game.board;
        let color_map = game.get_color_map();
        let colored_board = ColoredBoard { board, colors: &color_map };
        let mut lines = colored_board.render();

        if self.is_enabled() {
            self.render_cursor(&mut lines, board, &color_map, game.active_player());
        }

        let border_color = if self.is_enabled() { Color::default() } else { Color::Gray };

        frame.render_widget(
            Paragraph::new(lines)
                .not_underlined()
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

struct ColoredBoard<'a> {
    board: &'a Board,
    colors: &'a HashMap<usize, (Color, Color)>
}

impl <'a> RenderCanvas for ColoredBoard<'a> {
    fn render(&self) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = vec![];
        for y in 0..self.board.height {
            let mut line = vec![];
            for x in 0..self.board.width {
                let color = match self.board.get_state_on_position(&Position { x, y }).unwrap() {
                    crate::game::State::Free => Color::Gray,
                    crate::game::State::Occupied(player_id) => self.colors.get(&player_id).unwrap().0
                };
                line.push(Span::styled(BLOCK, Style::default().fg(color)))
            }
            lines.push(line.into());
        }
        lines
    }
}

impl IndexedPiece {
    fn rotate(&mut self) {
        self.rotations = (self.rotations + 1) % 4;
        self.piece.rotate();
    }
}