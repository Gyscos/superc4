pub type Player = usize;

mod board;
mod game;
mod error;

pub use self::board::{Pos,Cell,Board,Direction};
pub use self::game::{Game,Configuration,RuleError};
pub use self::error::Error;
