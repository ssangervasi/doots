use crate::game::board::{Board, Edge};
use crate::players::io::read_dot;
use crate::players::player::{Player, PlayerId};

pub const KEY: &str = "hoomin";

#[derive(Debug, Clone)]
pub struct Hoomin {
    id: PlayerId,
    name: String,
}

impl Hoomin {
    pub fn new(id: PlayerId) -> Self {
        Hoomin {
            id,
            name: match id {
                PlayerId::One => "Hoomin One".to_string(),
                PlayerId::Two => "Hoomin Two".to_string(),
            },
        }
    }

    pub fn named(id: PlayerId, name: String) -> Self {
        Self { id, name }
    }
}

impl Player for Hoomin {
    fn name(&self) -> String {
        self.name.to_string()
    }

    fn play(&self, board: Board) -> Edge {
        loop {
            println!("Draw an edge (row, col) -> (row, col):");
            println!("from: ");
            let dot_from = read_dot();
            println!("to: ");
            let dot_to = read_dot();
            println!("{:?} -> {:?}", dot_from, dot_to);

            let player_edge = Edge(dot_from, dot_to);

            match board.validate_draw(player_edge) {
                Ok(_) => return player_edge,
                Err(msg) => {
                    println!("{}", msg);
                    println!("Try again.");
                }
            }
        }
    }
}
