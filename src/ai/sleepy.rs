use std::thread;
use std::time::Duration;

use chrono::{Datelike, Local, Weekday};

use crate::game::board::{edge, Board, Edge};
use crate::players::player::{Player, PlayerId};

pub const KEY: &str = "sleepy";

#[derive(Debug, Clone)]
pub struct Sleepy {
    id: PlayerId,
}

impl Sleepy {
    pub fn new(id: PlayerId) -> Self {
        Self { id }
    }
}

impl Player for Sleepy {
    fn name(&self) -> String {
        match self.id {
            PlayerId::One => "Sleepy One".to_string(),
            PlayerId::Two => "Sleepy Two".to_string(),
        }
    }

    /*
     * Just for fun, this AI sleeps before making a move. Cute, but annoying
     * to test as you have to wait for it to finish. Also, you would have to
     * mock/modify the system clock to test the "special" behaviors.
     */
    fn play(&self, board: Board) -> Edge {
        let weekday = Local::today().weekday();
        if weekday == Weekday::Mon {
            panic!("I HATE MONDAYS!");
        }

        let sleep_secs = if weekday == Weekday::Sat || weekday == Weekday::Sun {
            // Sleeps extra long on weekends...
            5
        } else {
            1
        };
        for _ in 0..sleep_secs {
            println!("snore... ");
            thread::sleep(Duration::from_secs(1));
        }

        // And it's not even good at playing the game...
        for edge in board.iter_edges() {
            if board.is_free(edge) {
                return edge;
            }
        }
        return edge((0, 0), (0, 1));
    }
}
