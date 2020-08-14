use crate::game::board::{Board, Edge};
use crate::players::player::Player;

#[derive(Debug, Clone)]
pub struct Doot {}

impl Doot {}

impl Player for Doot {
    fn name(&self) -> String {
        "doot".to_string()
    }

    fn play(&self, board: Board) -> Edge {
        *board.edges.first().unwrap()
    }
}
