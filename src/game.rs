pub struct Game {
    pub(crate) board: Board,
    players: Players
}

pub(crate) struct Board {
    pub(crate) width: usize,
    pub(crate) height: usize,
    tiles: Vec<Vec<State>>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Occupied(usize)
}

#[derive(Default)]
pub struct Players {
    players: Vec<Player>,
    active_player_index: usize
}

#[derive(Default)]
struct Player {
    available_pieces: Vec<Piece>
}

#[derive(Debug)]
struct Piece {
    blocks: Vec<Position>
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Game {
    pub fn new(width: usize, height: usize, players: Players) -> Self {
        Game {
            board: Board::new(width, height),
            players
        }
    }

    pub fn width(&self) -> usize {
        self.board.width
    }

    pub fn height(&self) -> usize {
        self.board.height
    }
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            tiles: vec![vec![State::Free; width]; height]
        }
    }

    fn place_piece(&mut self, piece: Piece, offset: Position, player_index: usize) -> Result<bool, String> {
        for local_position in piece.blocks {
            let board_position = local_position + offset.clone();
            match self.get_state_on_position(board_position)? {
                State::Free => self.occupy_position(board_position, player_index)?,
                State::Occupied(_) => return Ok(false)
            }
        }
        Ok(true)
    }

    pub fn get_state_on_position(&self, position: Position) -> Result<State, String> {
        position.check_within_bounds(self.width, self.height)?;
        Ok(self.tiles[position.y as usize][position.x as usize])
    }

    fn occupy_position(&mut self, position: Position, player_index: usize) -> Result<(), String> {
        position.check_within_bounds(self.width, self.height)?;
        self.tiles[position.y as usize][position.x as usize] = State::Occupied(player_index);
        Ok(())
    }
}

impl Position {
    pub fn check_within_bounds(&self, width: usize, height: usize) -> Result<(), String> {
        match self {
            Position { x, y } if *x < 0 || *x >= width as i32 || *y < 0 || *y >= height as i32 => Err(format!("Out of bounds ({x}, {y})").to_string()),
            _ => Ok(())
        }
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piece_1x1() -> Piece {
        Piece { blocks: vec![Position { x: 0, y : 0 }]}
    }

    #[test]
    fn should_place_block() {
        let mut board = Board::new(1, 1);
        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }, 0).unwrap();
        assert!(was_placed);

        assert_eq!(board.get_state_on_position(Position { x: 0, y: 0 }).unwrap(), State::Occupied);

        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }, 0).unwrap();
        assert!(!was_placed)
    }
}