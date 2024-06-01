use std::collections::HashMap;

use rand::random;
use ratatui::style::Color;

pub struct Game {
    pub(crate) board: Board,
    players: Players,
}

pub(crate) struct Board {
    pub(crate) width: u16,
    pub(crate) height: u16,
    tiles: Vec<Vec<State>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Occupied(usize),
}

pub struct Players {
    players: Vec<Player>,
    active_player_index: usize,
}

#[derive(Default, PartialEq)]
pub struct Player {
    pub name: String,
    pub color: Color,
    pub secondary_color: Color,
    pub available_pieces: Vec<Piece>,
    pub first_move: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    blocks: Vec<Position>,
    pivot: f32,
    num_lines: u16,
    num_columns: u16,
    pub(crate) bounding_box_offset: Position,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Game {
    pub fn new(width: u16, height: u16, players: Players) -> Self {
        Game {
            board: Board::new(width, height),
            players,
        }
    }

    pub fn width(&self) -> u16 {
        self.board.width
    }

    pub fn height(&self) -> u16 {
        self.board.height
    }

    pub fn players(&self) -> &[Player] {
        &self.players.players
    }

    pub fn active_player_pieces(&self) -> &[Piece] {
        &self.active_player().available_pieces
    }

    pub fn place_piece(&mut self, piece_index: usize, rotations: u16, position: Position) -> Result<bool, String> {
        let player_index = self.players.active_player_index;
        let first_round = self.active_player().first_move;
        let mut piece = self.active_player_mut().take_piece(piece_index);

        (0..rotations).for_each(|_| piece.rotate());
        if let Some(piece) = self.board.place_piece(piece, position, player_index, first_round)? {
            self.return_piece_to_list(piece_index, rotations, piece);
            return Ok(false);
        }

        self.active_player_mut().first_move = false;
        self.switch_to_next_player();
        Ok(true)
    }

    pub fn active_player(&self) -> &Player {
        &self.players.players[self.players.active_player_index]
    }

    pub fn active_player_index(&self) -> usize {
        self.players.active_player_index
    }

    pub fn get_color_map(&self) -> HashMap<usize, (Color, Color)> {
        self.players()
            .iter()
            .enumerate()
            .map(|(id, player)| (id, (player.color, player.secondary_color)))
            .collect::<HashMap<usize, (Color, Color)>>()
    }

    fn switch_to_next_player(&mut self) {
        self.players.switch_to_next_player()
    }

    fn return_piece_to_list(&mut self, piece_index: usize, rotations: u16, mut piece: Piece) {
        (0..4 - rotations).for_each(|_| piece.rotate());
        self.active_player_mut().insert_piece(piece_index, piece);
    }

    fn active_player_mut(&mut self) -> &mut Player {
        &mut self.players.players[self.players.active_player_index]
    }
}

impl Board {
    fn new(width: u16, height: u16) -> Self {
        Board {
            width,
            height,
            tiles: vec![vec![State::Free; width as usize]; height as usize],
        }
    }

    fn place_piece(&mut self, piece: Piece, offset: Position, player_index: usize, first_round: bool) -> Result<Option<Piece>, String> {
        if !self.piece_can_be_placed(&piece, &offset, player_index, first_round) {
            return Ok(Some(piece));
        }

        for local_position in piece.blocks() {
            let board_position = &local_position + &offset;
            self.occupy_position(&board_position, player_index)?
        }

        Ok(None)
    }

    pub fn get_state_on_position(&self, position: &Position) -> Result<State, String> {
        position.check_within_bounds(self.width, self.height)?;
        Ok(self.tiles[position.y as usize][position.x as usize])
    }

    fn occupy_position(&mut self, position: &Position, player_index: usize) -> Result<(), String> {
        self.tiles[position.y as usize][position.x as usize] = State::Occupied(player_index);
        Ok(())
    }

    fn piece_can_be_placed(&self, piece: &Piece, offset: &Position, player_index: usize, first_round: bool) -> bool {
        let can_generally_be_placed = piece.blocks()
            .map(|block| &block + offset)
            .all(|position| self.block_position_is_not_occupied(&position)
                    && self.block_is_not_adjacent_to_other_blocks_from_same_player(&position, player_index));

        return if first_round {
            let touches_corner = piece.blocks()
                .map(|block| &block + offset).find(|position| self.block_touches_corner(position))
                .is_some();
            touches_corner && can_generally_be_placed
        } else {
            let is_diagonally_adjacent = piece.blocks()
                .map(|block| &block + offset).find(|position| self.block_is_diagonally_adjacent_to_block_from_same_player(position, player_index))
                .is_some();
            is_diagonally_adjacent && can_generally_be_placed
        }
    }

    fn block_position_is_not_occupied(&self, position: &Position) -> bool {
        match self.get_state_on_position(position).unwrap() {
            State::Free => true,
            State::Occupied(_) => false
        }
    }

    fn block_is_not_adjacent_to_other_blocks_from_same_player(&self, position: &Position, player_index: usize) -> bool {
        if position.x > 0 {
            match self.get_state_on_position(&Position { x: position.x - 1, y: position.y }).unwrap() {
                State::Occupied(index) if index == player_index => return false,
                _ => ()
            }
        }

        if position.x < self.width - 1 {
            match self.get_state_on_position(&Position { x: position.x + 1, y: position.y }).unwrap() {
                State::Occupied(index) if index == player_index => return false,
                _ => ()
            }
        }

        if position.y > 0 {
            match self.get_state_on_position(&Position { x: position.x, y: position.y - 1 }).unwrap() {
                State::Occupied(index) if index == player_index => return false,
                _ => ()
            }
        }

        if position.y < self.height - 1 {
            match self.get_state_on_position(&Position { x: position.x, y: position.y + 1 }).unwrap() {
                State::Occupied(index) if index == player_index => return false,
                _ => ()
            }
        }

        true
    }

    fn block_is_diagonally_adjacent_to_block_from_same_player(&self, position: &Position, player_index: usize) -> bool {
        if position.x > 0 && position.y > 0 {
            match self.get_state_on_position(&Position { x: position.x - 1, y: position.y - 1 }).unwrap() {
                State::Occupied(index) if index == player_index => return true,
                _ => ()
            }
        }

        if position.x < self.width - 1 && position.y > 0 {
            match self.get_state_on_position(&Position { x: position.x + 1, y: position.y - 1 }).unwrap() {
                State::Occupied(index) if index == player_index => return true,
                _ => ()
            }
        }

        if position.x > 0 && position.y < self.height - 1 {
            match self.get_state_on_position(&Position { x: position.x - 1, y: position.y + 1 }).unwrap() {
                State::Occupied(index) if index == player_index => return true,
                _ => ()
            }
        }

        if position.x < self.width - 1 && position.y < self.height - 1 {
            match self.get_state_on_position(&Position { x: position.x + 1, y: position.y + 1 }).unwrap() {
                State::Occupied(index) if index == player_index => return true,
                _ => ()
            }
        }

        false
    }

    fn block_touches_corner(&self, position: &Position) -> bool {
        if position.x == 0 && position.y == 0 {
            return true;
        }

        if position.x == self.width - 1 && position.y == 0 {
            return true;
        }

        if position.x == 0 && position.y == self.height - 1 {
            return true;
        }

        if position.x == self.width - 1 && position.y == self.height - 1 {
            return true;
        }

        false
    }
}

impl Position {
    pub fn check_within_bounds(&self, width: u16, height: u16) -> Result<(), String> {
        match self {
            Position { x, y } if *x >= width || *y >= height => Err(format!("Out of bounds ({x}, {y})").to_string()),
            _ => Ok(())
        }
    }

    pub fn rotate_around_pivot(&mut self, pivot_position: f32) {
        let temp_x = self.x;
        self.x = (pivot_position + pivot_position - self.y as f32) as u16;
        self.y = temp_x;
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Piece {
    pub fn new(blocks: Vec<Position>, pivot: f32) -> Self {
        let min_x = Self::min_x(&blocks);
        let min_y = Self::min_y(&blocks);
        let num_lines = Self::calculate_num_lines(&blocks, min_y);
        let num_columns = Self::calculate_num_columns(&blocks, min_x);
        let bounding_box_offset = Position { x: min_x, y: min_y };
        Piece { blocks, pivot, num_lines, num_columns, bounding_box_offset }
    }

    pub fn blocks(&self) -> impl Iterator<Item=Position> + '_ {
        self.blocks.iter().map(|block| block - &self.bounding_box_offset)
    }

    pub fn rotate(&mut self) {
        for block in self.blocks.iter_mut() {
            block.rotate_around_pivot(self.pivot);
        }
        std::mem::swap(&mut self.num_columns, &mut self.num_lines);
        self.bounding_box_offset = Position {
            x: Self::min_x(&self.blocks),
            y: Self::min_y(&self.blocks),
        }
    }

    pub fn num_lines(&self) -> u16 {
        self.num_lines
    }

    pub fn num_columns(&self) -> u16 {
        self.num_columns
    }

    fn calculate_num_lines(blocks: &[Position], min_y: u16) -> u16 {
        let max_y = blocks.iter().map(|block| block.y).max().unwrap();
        (max_y - min_y) + 1
    }

    fn calculate_num_columns(blocks: &[Position], min_x: u16) -> u16 {
        let max_x = blocks.iter().map(|block| block.x).max().unwrap();
        (max_x - min_x) + 1
    }

    fn min_x(blocks: &[Position]) -> u16 {
        blocks.iter().map(|block| block.x).min().unwrap()
    }

    fn min_y(blocks: &[Position]) -> u16 {
        blocks.iter().map(|block| block.y).min().unwrap()
    }
}

impl Players {
    pub fn new(players: Vec<Player>) -> Self {
        let active_player_index = random::<usize>() % players.len();
        Players {
            players,
            active_player_index,
        }
    }

    pub fn switch_to_next_player(&mut self) {
        self.active_player_index = (self.active_player_index + 1) % self.players.len();
    }
}

impl Player {
    pub fn new(name: String, color: Color, secondary_color: Color, available_pieces: Vec<Piece>) -> Self {
        Player {
            name,
            color,
            secondary_color,
            available_pieces,
            first_move: true,
        }
    }

    fn take_piece(&mut self, index: usize) -> Piece {
        self.available_pieces.remove(index)
    }

    fn insert_piece(&mut self, index: usize, piece: Piece) {
        self.available_pieces.insert(index, piece)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piece_1x1() -> Piece {
        Piece::new(vec![Position { x: 0, y: 0 }], 0.0)
    }

    #[test]
    fn should_place_block() {
        let mut board = Board::new(1, 1);
        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }, 0, true).unwrap();
        assert!(was_placed.is_none());

        assert_eq!(board.get_state_on_position(&Position { x: 0, y: 0 }).unwrap(), State::Occupied(0));

        let was_placed = board.place_piece(piece_1x1(), Position { x: 0, y: 0 }, 0, true).unwrap();
        assert!(was_placed.is_some())
    }

    #[test]
    fn should_rotate_block() {
        let mut piece = Piece::new(vec![Position { x: 0, y: 1 }, Position { x: 1, y: 1 }, Position { x: 2, y: 1 }], 1.0);

        piece.rotate();
        assert_eq!(piece.blocks, vec![Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 1, y: 2 }]);

        piece.rotate();
        assert_eq!(piece.blocks, vec![Position { x: 2, y: 1 }, Position { x: 1, y: 1 }, Position { x: 0, y: 1 }]);
    }

    #[test]
    fn should_rotate_box_block() {
        let mut piece = Piece::new(vec![Position { x: 0, y: 0 }, Position { x: 1, y: 0 }, Position { x: 0, y: 1 }, Position { x: 1, y: 1 }], 0.5);
        piece.rotate();
        assert_eq!(piece.blocks, vec![Position { x: 1, y: 0 }, Position { x: 1, y: 1 }, Position { x: 0, y: 0 }, Position { x: 0, y: 1 }])
    }
}