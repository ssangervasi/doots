use crate::ai::doot::Doot;
use crate::players::hoomin::Hoomin;
use crate::players::player::{Player, PlayerId};

pub fn choose(one: &String, two: &String) -> Vec<(PlayerId, Box<dyn Player>)> {
    let mut players: Vec<(PlayerId, Box<dyn Player>)> = Vec::new();

    if one == "hoomin" {
        players.push((PlayerId::One, Box::new(Hoomin::one())));
    } else if one == "doot" {
        players.push((PlayerId::One, Box::new(Doot {})));
    };
    if two == "hoomin" {
        players.push((PlayerId::Two, Box::new(Hoomin::two())));
    } else if two == "doot" {
        players.push((PlayerId::Two, Box::new(Doot {})));
    }

    if players.len() != 2 {
        panic!("Unable to choose players for: {}, {}", one, two);
    }
    players
}

// pub fn fill(one: &str, two: &str, mut players: Vec<&dyn Player>) {
//     if one != two {
//         panic!("oh no");
//     }
//     let p1 = Hoomin::one();
//     let p2 = Hoomin::two();
//     // let b = Box::new(p1);
//     // vec![Box::new(p1), Box::new(p2)];
//     players[0] = &p1;
//     players[1] = &p2;
// }
