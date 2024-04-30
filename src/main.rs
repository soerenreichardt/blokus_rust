use std::io;
use crate::game::{Game, Players};

mod game;
mod ui;

fn main() -> io::Result<()>{
    let mut game = Game::new(16, 16, Players::default());
    ui::run(&mut game)
}
