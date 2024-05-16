use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::game::{Game, Player};
use crate::ui::{AppEvent, BLOCK, Module, ModuleKind, RenderCanvas};

pub struct PlayerDisplay;

impl Module for PlayerDisplay {
    fn update(&mut self, _event: AppEvent, _game: &Game) -> Option<AppEvent> {
        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, game: &mut Game) {
        let text: Vec<Line<'_>> = game.players().iter().flat_map(Player::render).collect();
        frame.render_widget(
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Players")),
            area
        )
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Player
    }
}

impl RenderCanvas for Player {
    fn render(&self) -> Vec<Line<'_>> {
        vec![Span::styled(format!("{}  {}", BLOCK, self.name), Style::default().fg(self.color)).into()]
    }
}