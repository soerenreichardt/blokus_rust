use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::game::{Game, Player};
use crate::ui::{AppEvent, BLOCK, Module, ModuleKind, RenderCanvas};

pub struct PlayerDisplay;

impl Module for PlayerDisplay {
    fn update(&mut self, _event: AppEvent, _game: &mut Game) -> Option<AppEvent> {
        None
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, game: &mut Game) {
        let stateful_players = game.players().iter()
            .map(|player| StatefulPlayer { player, is_active: player == game.active_player() })
            .collect::<Vec<_>>();
        let text: Vec<Line<'_>> = stateful_players.iter().flat_map(StatefulPlayer::render).collect();
        frame.render_widget(
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Players")),
            area
        )
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Player
    }
}

struct StatefulPlayer<'a> {
    player: &'a Player,
    is_active: bool
}

impl <'a> RenderCanvas for StatefulPlayer<'a> {
    fn render(&self) -> Vec<Line<'_>> {
        let color = if self.is_active { self.player.color } else { Color::default() };
        vec![Span::styled(format!("{}  {}", BLOCK, self.player.name), Style::default().fg(color)).into()]
    }
}