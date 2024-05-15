use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::game::{Game, Piece};
use crate::ui::{AppEvent, BLOCK, Cursor, Module, ModuleKind, RenderCanvas, UI_OFFSET};
use crate::ui::scrollbars::VerticalScrollBar;

pub struct PieceDisplay {
    cursor: Cursor,
    piece_index: usize,
    scrollbar: VerticalScrollBar,
    enabled: bool
}

impl PieceDisplay {
    pub fn new(height: usize) -> Self {
        let cursor = Cursor::new(0, height as i32);
        PieceDisplay {
            cursor,
            piece_index: 0,
            scrollbar: VerticalScrollBar::default(),
            enabled: false
        }
    }

    fn render_piece<'a>(piece: &'a RenderPiece) -> Vec<Line<'a>> {
        let mut lines = piece.render();
        lines.push(Span::styled("\n", Style::default()).into());
        lines
    }
}

impl Module for PieceDisplay {
    fn update(&mut self, event: AppEvent) {
        if let AppEvent::OpenPieceSelection = event {
            self.enabled = true
        }
        if self.enabled {
            match event {
                AppEvent::MoveDown => self.cursor.move_down(),
                AppEvent::MoveUp => self.cursor.move_up(),
                _ => ()
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, widget_area: Rect, game: &mut Game) {
        let pieces = game.active_player_pieces();
        let render_pieces = pieces.iter()
            .enumerate()
            .map(|(row, piece)| RenderPiece::new(piece, &self.cursor, row as i32))
            .collect::<Vec<_>>();
        let text = render_pieces.iter()
            .flat_map(Self::render_piece)
            .collect::<Vec<_>>();
        let text_len = text.len() as u16;

        let pieces_prefix_sum = render_pieces.iter().scan(0, |sum, piece| {
            *sum += piece.piece.num_lines() + 1;
            Some(*sum)
        }).collect::<Vec<_>>();

        let mut virtual_cursor = Cursor::new(0, *pieces_prefix_sum.last().unwrap() as i32);
        virtual_cursor.position.y = pieces_prefix_sum[self.cursor.position.y as usize] as i32 - pieces_prefix_sum[0] as i32;
        self.scrollbar.update_scrollbar(widget_area, &virtual_cursor);

        let border_color = if self.enabled { Color::default() } else { Color::Gray };
        frame.render_widget(
            Paragraph::new(text)
                .centered()
                .scroll((self.scrollbar.offset(), 0))
                .block(Block::default()
                    .title(format!("{} - {}", self.cursor.position.y, virtual_cursor.position.y))
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
    cursor: &'a Cursor,
    position: i32
}

impl<'a> RenderPiece<'a> {
    fn new(piece: &'a Piece, cursor: &'a Cursor, position: i32) -> Self {
        RenderPiece {
            piece,
            cursor,
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
        let color = if self.position == self.cursor.position.y { Color::Blue } else { Color::Gray };
        for block in self.piece.blocks.iter() {
            // casting block y|x to usize is a problem as rotated pieces can have negative coordinates
            canvas[block.y as usize][block.x as usize] = Span::styled(BLOCK, Style::default().fg(color))
        }
        canvas.into_iter().map(|line| line.into()).collect()
    }
}