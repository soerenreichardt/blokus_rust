pub struct Game {
    board: Board,
    players: Players
}

struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<State>>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Free,
    Occupied
}

#[derive(Default)]
struct Players {
    players: Vec<Player>,
    active_player_index: usize
}

#[derive(Default)]
struct Player;

struct Piece {
    blocks: Vec<Position>
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize
}

impl Game {
    fn new(width: usize, height: usize, players: Players)  -> Self {
        Game {
            board: Board::new(width, height),
            players
        }
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

    fn place_piece(&mut self, piece: Piece, offset: Position) -> Result<bool, String> {
        for local_position in piece.blocks {
            let board_position = local_position + offset.clone();
            match self.get_state_on_position(board_position)? {
                State::Free => self.occupy_position(board_position)?,
                State::Occupied => return Ok(false)
            }
        }
        Ok(true)
    }

    fn get_state_on_position(&self, position: Position) -> Result<State, String> {
        position.check_within_bounds(self.width, self.height)?;
        Ok(self.tiles[position.y][position.x])
    }

    fn occupy_position(&mut self, position: Position) -> Result<(), String> {
        position.check_within_bounds(self.width, self.height)?;
        self.tiles[position.y][position.x] = State::Occupied;
        Ok(())
    }
}

impl Position {
    fn check_within_bounds(&self, width: usize, height: usize) -> Result<(), String> {
        match self {
            Position { x, y } if *x >= width || *y >= height => Err("Out of bounds".to_string()),
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
        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }).unwrap();
        assert!(was_placed);

        assert_eq!(board.get_state_on_position(Position { x: 0, y: 0 }).unwrap(), State::Occupied);

        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }).unwrap();
        assert!(!was_placed)
    }
}