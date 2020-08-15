use crate::game::board::{edge, Board, Edge};
use crate::players::player::{Player, PlayerId};

pub const KEY: &str = "doot";

#[derive(Debug, Clone)]
pub struct Doot {
    id: PlayerId,
}

impl Doot {
    pub fn new(id: PlayerId) -> Self {
        Self { id }
    }
}

impl Player for Doot {
    fn name(&self) -> String {
        match self.id {
            PlayerId::One => "Doot One".to_string(),
            PlayerId::Two => "Doot Two".to_string(),
        }
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
