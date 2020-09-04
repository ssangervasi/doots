use crate::game::board::{Board, Edge};
use crate::players::player::{Player, PlayerId};

pub const KEY: &str = "boox";

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
            PlayerId::One => "Boox One".to_string(),
            PlayerId::Two => "Boox Two".to_string(),
        }
    }

    fn play(&self, board: Board) -> Edge {
        let (claimers, others): (Vec<Edge>, Vec<Edge>) = board
            .iter_owned_edges()
            .filter(|&(owner, _)| owner != self.id)
            .map(|(_, edge)| edge)
            .collect::<Vec<Edge>>()
            .iter()
            .rev()
            .map(|&edge| edge)
            .flat_map(|edge| {
                board
                    .associated_boxes(edge)
                    .iter()
                    .flat_map(|&dotbox| dotbox.edges())
                    .collect::<Vec<Edge>>()
            })
            .filter(|&edge| board.is_free(edge))
            .partition(|&edge| board.would_claim_box(edge));

        if claimers.len() > 0 {
            return claimers[0];
        }
        if others.len() > 0 {
            return others[0];
        }
        board
            .iter_edges()
            .find(|&edge| board.is_free(edge))
            .unwrap()
    }
}
