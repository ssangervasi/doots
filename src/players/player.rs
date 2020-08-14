use crate::game::board::{Board, Edge};

pub trait Player {
    fn name(&self) -> String;
    fn play(&self, board: Board) -> Edge;
}
