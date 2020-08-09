use crate::board::{edge, Board, Edge};

pub trait Player {
    fn play(&self, board: Board) -> Edge {
        for i in 0..board.dot_count() {
            if i == 1 {
                return edge((i, i), (i, i + 1));
            }
        }
        edge((0, 0), (0, 1))
    }
}
