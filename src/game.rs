use rand::random;
use ratatui::style::Color;

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

pub struct Players {
    players: Vec<Player>,
    active_player_index: usize
}

#[derive(Default)]
pub struct Player {
    pub name: String,
    pub color: Color,
    pub available_pieces: Vec<Piece>
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub(crate) blocks: Vec<Position>,
    pivot: Position,
    num_lines: u16,
    num_columns: u16
}

#[derive(Clone, Debug, Default, PartialEq)]
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

    pub fn players(&self) -> &[Player] {
        &self.players.players
    }

    pub fn active_player_pieces(&self) -> &[Piece] {
        &self.active_player().available_pieces
    }

    fn active_player(&self) -> &Player {
        &self.players.players[self.players.active_player_index]
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
            match self.get_state_on_position(&board_position)? {
                State::Free => self.occupy_position(&board_position, player_index)?,
                State::Occupied(_) => return Ok(false)
            }
        }
        Ok(true)
    }

    pub fn get_state_on_position(&self, position: &Position) -> Result<State, String> {
        position.check_within_bounds(self.width, self.height)?;
        Ok(self.tiles[position.y as usize][position.x as usize])
    }

    fn occupy_position(&mut self, position: &Position, player_index: usize) -> Result<(), String> {
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

    pub fn rotate_around_pivot(&mut self, pivot_position: &Position) {
        let temp_x = self.x;
        self.x = pivot_position.x + pivot_position.y - self.y;
        self.y = pivot_position.y - pivot_position.x + temp_x;
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

impl Piece {
    pub fn new(blocks: Vec<Position>) -> Self {
        let pivot = Self::find_pivot_position(&blocks);
        let num_lines = Self::calculate_num_lines(&blocks);
        let num_columns = Self::calculate_num_columns(&blocks);
        Piece { blocks, pivot, num_lines, num_columns }
    }

    pub fn rotate(&mut self) {
        for block in self.blocks.iter_mut() {
            block.rotate_around_pivot(&self.pivot)
        }
    }

    pub fn num_lines(&self) -> u16 {
        self.num_lines
    }

    pub fn num_columns(&self) -> u16 {
        self.num_columns
    }

    fn find_pivot_position(blocks: &[Position]) -> Position {
        let num_blocks = blocks.len();
        let mut pivot_x = 0.0f64;
        let mut pivot_y = 0.0f64;
        for block in blocks {
            pivot_x += block.x as f64 / num_blocks as f64;
            pivot_y += block.y as f64 / num_blocks as f64;
        }
        Position {
            x: pivot_x as i32,
            y: pivot_y as i32
        }
    }

    fn calculate_num_lines(blocks: &[Position]) -> u16 {
        let max_y = blocks.iter().map(|block| block.y).max().unwrap() as u16;
        max_y + 1
    }

    fn calculate_num_columns(blocks: &[Position]) -> u16 {
        let max_x = blocks.iter().map(|block| block.x).max().unwrap() as u16;
        max_x + 1
    }
}

impl Players {
    pub fn new(players: Vec<Player>) -> Self {
        let active_player_index = random::<usize>() % players.len();
        Players {
            players,
            active_player_index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piece_1x1() -> Piece {
        Piece::new(vec![Position { x: 0, y : 0 }])
    }

    #[test]
    fn should_place_block() {
        let mut board = Board::new(1, 1);
        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }, 0).unwrap();
        assert!(was_placed);

        assert_eq!(board.get_state_on_position(&Position { x: 0, y: 0 }).unwrap(), State::Occupied(0));

        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }, 0).unwrap();
        assert!(!was_placed)
    }

    #[test]
    fn should_rotate_block() {
        let mut piece = Piece::new(vec![Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 2, y: 0 }]);

        piece.rotate();
        assert_eq!(piece.blocks, vec![Position { x: 1, y: -1}, Position { x: 1, y: 0}, Position { x: 1, y: 1}]);

        piece.rotate();
        assert_eq!(piece.blocks, vec![Position { x: 2, y: 0}, Position { x: 1, y: 0}, Position { x: 0, y: 0}]);
    }
}