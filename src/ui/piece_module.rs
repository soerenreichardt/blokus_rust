use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::game::{Game, Piece};
use crate::ui::{AppEvent, BLOCK, Cursor, Module, ModuleKind, RenderCanvas, UI_OFFSET};
use crate::ui::scrollbars::VerticalScrollBar;

pub struct PieceDisplay {
    selection_index: usize,
    cursor: Cursor,
    scrollbar: VerticalScrollBar,
    enabled: bool
}

impl PieceDisplay {
    pub fn new() -> Self {
        PieceDisplay {
            selection_index: 0,
            cursor: Cursor::default(),
            scrollbar: VerticalScrollBar::default(),
            enabled: false
        }
    }

    fn render_piece<'a>(piece: &'a RenderPiece) -> Vec<Line<'a>> {
        let mut lines = piece.render();
        lines.push(Span::styled("\n", Style::default()).into());
        lines
    }

    fn move_cursor_down(&mut self, game: &Game) {
        if self.selection_index < game.active_player_pieces().len() - 1 {
            self.cursor.area.y += (&game.active_player_pieces()[self.selection_index]).num_lines() + 1;
            self.selection_index += 1;
            self.update_cursor_dimensions(&game.active_player_pieces()[self.selection_index]);
        }
    }

    fn move_cursor_up(&mut self, game: &Game) {
        if self.selection_index > 0 {
            self.selection_index -= 1;
            let active_piece = &game.active_player_pieces()[self.selection_index];
            self.cursor.area.y = self.cursor.area.y.saturating_sub(active_piece.num_lines() + 1);
            self.update_cursor_dimensions(active_piece);
        }
    }

    fn update_cursor_dimensions(&mut self, piece: &Piece) {
        self.cursor.area.height = piece.num_lines();
        self.cursor.area.width = piece.num_columns();
    }

    fn select_piece(&mut self) -> usize {
        self.enabled = false;
        self.selection_index
    }
}

impl Module for PieceDisplay {
    fn update(&mut self, event: AppEvent, game: &Game) -> Option<AppEvent> {
        if let AppEvent::OpenPieceSelection = event {
            self.enabled = true
        }
        if self.enabled {
            match event {
                AppEvent::MoveDown => self.move_cursor_down(game),
                AppEvent::MoveUp => self.move_cursor_up(game),
                AppEvent::Select => return Some(AppEvent::PieceSelected(self.select_piece())),
                _ => ()
            }
        }
        return None;
    }

    fn render(&mut self, frame: &mut Frame, widget_area: Rect, game: &mut Game) {
        let pieces = game.active_player_pieces();
        let render_pieces = pieces.iter()
            .enumerate()
            .map(|(row, piece)| RenderPiece::new(piece, self.selection_index, row))
            .collect::<Vec<_>>();
        let text = render_pieces.iter()
            .flat_map(Self::render_piece)
            .collect::<Vec<_>>();
        let text_len = text.len() as u16;

        self.scrollbar.update_scrollbar(widget_area, &self.cursor);

        let border_color = if self.enabled { Color::default() } else { Color::Gray };
        frame.render_widget(
            Paragraph::new(text)
                .centered()
                .scroll((self.scrollbar.offset(), 0))
                .block(Block::default()
                    .title(format!("{} - {}", self.selection_index, self.cursor.area.y))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .title("Pieces")
                ),
            widget_area
        );

        self.scrollbar.render_scrollbar(frame, text_len + UI_OFFSET, widget_area);
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Piece
    }
}

struct RenderPiece<'a> {
    piece: &'a Piece,
    selection_index: usize,
    position: usize
}

impl<'a> RenderPiece<'a> {
    fn new(piece: &'a Piece, selection_index: usize, position: usize) -> Self {
        RenderPiece {
            piece,
            selection_index,
            position
        }
    }
}

impl<'a> RenderCanvas for RenderPiece<'a> {
    fn render(&self) -> Vec<Line<'_>> {
        let empty_tile = Span::styled("  ", Style::default());
        let num_columns = self.piece.num_columns() as usize;
        let num_lines = self.piece.num_lines() as usize;

        let mut canvas = vec![vec![empty_tile; num_columns]; num_lines];
        let color = if self.position == self.selection_index { Color::Blue } else { Color::Gray };
        for block in self.piece.blocks.iter() {
            // casting block y|x to usize is a problem as rotated pieces can have negative coordinates
            canvas[block.y as usize][block.x as usize] = Span::styled(BLOCK, Style::default().fg(color))
        }
        canvas.into_iter().map(|line| line.into()).collect()
    }
}