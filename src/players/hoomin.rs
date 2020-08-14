use crate::game::board::{Board, Edge};
use crate::game::utils::read_dot;

pub trait Player {
    fn name(&self) -> String;
    fn play(&self, board: Board) -> Edge;
}

#[derive(Debug, Clone)]
pub struct Hoomin {
    name: String,
}

impl Hoomin {
    pub fn one() -> Hoomin {
        Hoomin {
            name: "one".to_string(),
        }
    }

    pub fn two() -> Hoomin {
        Hoomin {
            name: "two".to_string(),
        }
    }
}

impl Player for Hoomin {
    fn name(&self) -> String {
        self.name.clone()
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
