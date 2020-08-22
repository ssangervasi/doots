use crate::game::board::{edge, Board, Edge};
use crate::players::player::{Player, PlayerId};

pub const KEY: &str = "doot";

#[derive(Debug, Clone)]
pub struct AI {
    id: PlayerId,
}

impl AI {
    pub fn new(id: PlayerId) -> Self {
        Self { id }
    }
}

impl Player for AI {
    fn name(&self) -> String {
        match self.id {
            PlayerId::One => "Doot One".to_string(),
            PlayerId::Two => "Doot Two".to_string(),
        }
    }

    fn play(&self, board: Board) -> Edge {
        let mut last_free_edge = edge((0, 0), (0, 1));
        for edge in board.iter_edges() {
            if !board.is_free(edge) {
                continue;
            }
            last_free_edge = edge;

            // If a box can be taken, return immediately.
            if board.would_claim_box(edge) {
                return edge;
            }
        }

        // If no squares were found, just dumly take a free edge.
        last_free_edge
    }
}
