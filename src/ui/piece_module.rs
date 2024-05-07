use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Line, Span};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::game::{Game, Piece};
use crate::ui::{AppEvent, BLOCK, Cursor, Module, ModuleKind, RenderCanvas};
use crate::ui::cursor_scrollbar::CursorScrollbar;

pub struct PieceDisplay {
    cursor: Cursor,
    cursor_scrollbar: CursorScrollbar,
    enabled: bool
}

impl PieceDisplay {
    pub fn new(height: usize) -> Self {
        let cursor = Cursor::new(0, height as i32);
        PieceDisplay {
            cursor,
            cursor_scrollbar: CursorScrollbar::default(),
            enabled: false
        }
    }

    fn render_piece(piece: &Piece) -> Vec<Line<'_>> {
        let mut lines = piece.render();
        lines.push(Span::styled("\n", Style::default()).into());
        lines
    }
}

impl Module for PieceDisplay {
    fn update(&mut self, event: AppEvent) {
        match event {
            AppEvent::OpenPieceSelection => self.enabled = true,
            _ => ()
        }
    }

    fn render(&mut self, frame: &mut Frame, widget_area: Rect, game: &mut Game) {
        let pieces = game.active_player_pieces();
        let text = pieces.iter().flat_map(Self::render_piece).collect::<Vec<_>>();

        let border_color = if self.enabled { Color::default() } else { Color::Gray };
        frame.render_widget(
            Paragraph::new(text)
                .centered()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color))
                    .title("Pieces")
                ),
            widget_area
        )
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Piece
    }
}

impl RenderCanvas for Piece {
    fn render(&self) -> Vec<Line<'_>> {
        let empty_tile = Span::styled("  ", Style::default());
        let mut canvas = vec![vec![empty_tile; self.num_columns() as usize]; self.num_lines() as usize];
        for block in self.blocks.iter() {
            // casting block y|x to usize is a problem as rotated pieces can have negative coordinates
            canvas[block.y as usize][block.x as usize] = Span::styled(BLOCK, Style::default().fg(Color::Blue))
        }
        canvas.into_iter().map(|line| line.into()).collect()
    }
}