use crate::game::board::{Board, Edge};
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
        let mut scored_edges: Vec<(i8, Edge)> = vec![];

        for edge in board.iter_edges() {
            if !board.is_free(edge) {
                continue;
            }
            let mut score: i8 = 1;

            // If a box can be taken, return immediately.
            if board.would_claim_box(edge) {
                score += 2
            }

            // Check if drawing the edge would open up a box for the opponent.
            let mut hypothetical_board = board.clone();
            hypothetical_board
                .draw((self.id, edge))
                .expect("Invalid draw");
            let gives_to_opponent = hypothetical_board
                .iter_edges()
                .any(|opponent_edge| hypothetical_board.would_claim_box(opponent_edge));
            if gives_to_opponent {
                score -= 1
            }

            scored_edges.push((score, edge));
        }

        scored_edges.sort_by_key(|&(s, _)| s);
        scored_edges.last().unwrap().1
    }
}
