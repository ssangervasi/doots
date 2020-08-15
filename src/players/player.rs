use crate::game::board::{Board, Edge};

pub trait Player {
    fn name(&self) -> String;
    fn play(&self, board: Board) -> Edge;
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PlayerId {
    One = 1,
    Two = 2,
}
