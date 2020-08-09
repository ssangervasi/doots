use crate::board::{Board, Edge};
use crate::utils::read_dot;

pub trait Player {
    fn play(&self, board: Board) -> Edge;
}

pub struct Hoomin {}

impl Player for Hoomin {
    fn play(&self, board: Board) -> Edge {
        loop {
            println!("Draw an edge (row, col) -> (row, col):");
            println!("from: ");
            let dot_from = read_dot();
            println!("to  : ");
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
