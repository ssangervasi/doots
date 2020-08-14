use crate::game::board::{edge, Board, Edge};
use crate::players::player::Player;

#[derive(Debug, Clone)]
pub struct Doot {}

impl Doot {}

impl Player for Doot {
    fn name(&self) -> String {
        "doot".to_string()
    }

    fn play(&self, board: Board) -> Edge {
        for edge in board.iter_edges() {
            if board.is_free(edge) {
                return edge;
            }
        }
        return edge((0, 0), (0, 1));
    }
}
