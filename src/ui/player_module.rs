use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::game::Game;
use crate::ui::{AppEvent, BLOCK, Module, ModuleKind};

pub struct PlayerDisplay;

impl Module for PlayerDisplay {
    fn update(&mut self, _event: AppEvent, _game: &Game) {}

    fn render(&mut self, frame: &mut Frame, area: Rect, game: &mut Game) {
        let text: Vec<Line<'_>> = game.players().iter().map(|player|
            Span::styled(format!("{}  {}", BLOCK, player.name), Style::default().fg(player.color)).into()
        ).collect();
        frame.render_widget(
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Players")),
            area
        )
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Player
    }
}