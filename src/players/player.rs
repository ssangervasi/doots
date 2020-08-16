use core::fmt;

use crate::game::board::{Board, Edge};

pub trait Player {
    fn name(&self) -> String;
    fn play(&self, board: Board) -> Edge;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum PlayerId {
    One = 1,
    Two = 2,
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                PlayerId::One => "One",
                PlayerId::Two => "Two",
            }
        )
    }
}
