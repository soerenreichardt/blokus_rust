use std::io;
use std::str::FromStr;
use ratatui::prelude::Color;

use crate::game::{Game, Piece, Player, Players, Position};

mod game;
mod ui;

fn main() -> io::Result<()>{
    let piece_set = read_piece_set().unwrap();
    let players = Players::new(vec![
        Player::new("Bob".to_string(), Color::Green, Color::LightGreen, piece_set.clone()),
        Player::new("Alice".to_string(), Color::Blue, Color::LightBlue, piece_set.clone()),
        Player::new("Eve".to_string(), Color::Yellow, Color::LightYellow, piece_set.clone()),
        Player::new("Pete".to_string(), Color::Red, Color::LightRed, piece_set.clone()),
    ]);
    let mut game = Game::new(20, 20, players);
    ui::run(&mut game)
}

fn read_piece_set() -> Result<Vec<Piece>, String> {
    std::str::from_utf8(include_bytes!("res/standard_pieces"))
        .unwrap()
        .split("\n\n")
        .map(Piece::from_str)
        .collect()
}

impl FromStr for Piece {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let blocks = string
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, c)| match c {
                'x' => Some(Position { x: x as u16, y: y as u16 }),
                _ => None
            }))
            .collect::<Vec<_>>();

        let bounding_box_dimension = (string.lines().count() - 1) as f32;
        let pivot_position = bounding_box_dimension / 2.0;
        Ok(Piece::new(blocks, pivot_position))
    }
}