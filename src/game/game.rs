use std::iter;

use super::{Pos,Board,Player,Error,Cell,Direction};

/// A Game represents a unique match.
///
/// It starts with a given tile configuration, and ends when a player wins, or with a draw.
///
/// It should hold all of the game logic. Given a Game, invalid moves are impossible.
pub struct Game {
    board: Board,

    next_player: Player,
}

/// Starter configuration for a board.
///
/// Simple configurations are just empty grids; some can have patterns of blocked tiles.
/// A configuration is always valid: there is no duplicated blocked tile, the size is correct, etc.
///
/// TODO: load a configuration from a config file to allow user-defined ones.
pub struct Configuration {
    size: usize,
    blocked_tiles: Vec<Pos>,
}

#[derive(Debug)]
pub enum RuleError {
    InvalidTurn,
    NonEmptyCell,
    FloatingCell,
}


impl Configuration {
    pub fn new(size: usize) -> Self {
        Configuration {
            size: size,
            blocked_tiles: Vec::new(),
        }
    }

    pub fn add_blocked(mut self, pos: Pos) -> Self {
        self.blocked_tiles.push(pos);
        self
    }

    pub fn clean_8() -> Self {
        Self::new(8)
    }

    pub fn clean_10() -> Self {
        Self::new(10)
    }

    pub fn corner_8() -> Self {
        Self::corner_n(8)
    }

    pub fn corner_10() -> Self {
        Self::corner_n(10)
    }

    pub fn corner_n(size: usize) -> Self {
        let i_size = size as i8;
        Self::new(size)
            .add_blocked((1,2))
            .add_blocked((2,i_size-2))
            .add_blocked((i_size-2,i_size-3))
            .add_blocked((i_size-3,1))
    }

}

// WIN if in one axis, there is 4+ consecutive cells of the same player
fn is_winning(board: &Board, player: Player, pos: Pos) -> bool {
    for &dir in Direction::AXIS {
        let count = 1
            + count_cells(board, pos, dir, player)
            + count_cells(board, pos, dir.flip(), player);
        if count >= 4 {
            return true;
        }
    }

    false
}

fn count_cells(board: &Board, mut pos: Pos, dir: Direction, player: Player) -> u32 {
    let mut result = 0;

    loop {
        pos = dir.from(pos);

        if !board.is_valid(pos) { break; }
        if board.get(pos) != Cell::Full(player) { break; }

        result += 1;
    }

    result
}

// Return TRUE if the given cell is floating, IE inaccessible
fn is_floating(board: &Board, pos: Pos) -> bool {
    for &dir in Direction::CARDINAL {
        if is_full_line(board, pos, dir) {
            return false;
        }
    }

    true
}

// Return TRUE if a complete line joins the given cell to any wall
fn is_full_line(board: &Board, mut pos: Pos, d: Direction) -> bool {

    loop {
        pos = d.from(pos);

        if !board.is_valid(pos) {
            return true;
        }

        if board.is_empty(pos) {
            return false;
        }
    }
}

impl Game {

    /// Creates a new game from the given board configuration
    pub fn new(config: Configuration) -> Self {

        let mut board = Board::new(config.size);
        for tile in config.blocked_tiles.iter() {
            board.block(*tile);
        }

        Game {
            board: board,
            next_player: 0,
        }
    }

    /// Returns the player who will play next
    pub fn get_next_player(&self) -> Player {
        self.next_player
    }

    /// Play on the given cell.
    pub fn play(&mut self, player: Player, pos: Pos) -> Result<(),Error> {
        // Step 1: Is it even our turn?
        if player != self.next_player {
            return Err(Error::Rule(RuleError::InvalidTurn));
        }

        // Step 2: is the cell empty?
        if let Cell::Empty = self.board.get(pos) {
            return Err(Error::Rule(RuleError::NonEmptyCell));
        }

        // Step 3: Is the cell accessible?
        // We only need one direction
        if is_floating(&self.board, pos) {
            return Err(Error::Rule(RuleError::FloatingCell));
        }

        // Everything's allright, we can play
        self.board.play(pos, player);

        // Now check for victory condition
        if is_winning(&self.board, player, pos) {

        }

        Ok(())
    }
}
