use crate::players::hoomin;
use crate::players::player::{Player, PlayerId};

use crate::ai::doot;
use crate::ai::sleepy;

/*
 * Constructs the players for a game from the known Player types. Any name that
 * doesn't
 */
pub fn choose(name_one: &str, name_two: &str) -> Vec<(PlayerId, Box<dyn Player>)> {
    let mut players: Vec<(PlayerId, Box<dyn Player>)> = Vec::new();

    for (name, id) in [(name_one, PlayerId::One), (name_two, PlayerId::Two)].iter() {
        if *name == doot::KEY {
            players.push((*id, Box::new(doot::Doot::new(*id))));
        } else if *name == sleepy::KEY {
            players.push((*id, Box::new(sleepy::Sleepy::new(*id))));
        } else {
            // Fall back to a named human to allow any names for custom games:
            players.push((
                *id,
                Box::new(hoomin::Hoomin::named(*id, name_one.to_string())),
            ));
        }
    }

    if players.len() != 2 {
        panic!("Unable to choose players for: {}, {}", name_one, name_two);
    }
    players
}
