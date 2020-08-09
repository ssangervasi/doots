use clap::{App, Arg};
use textwrap::dedent as dd;

use doots::board::{Board, BoardSize};
use doots::hoomin::{Hoomin, Player};

fn main() {
    let matches = App::new("doots")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .default_value("10")
                .help(&dd("
                    How many boxes wide and tall the game is, ex:
                        size 1 => 1x1 grid => 4 dots
                        size 2 => 2x2 grid => 9 dots
                    ")),
        )
        .get_matches();

    let board_size: BoardSize = match matches.value_of("size").unwrap().trim().parse() {
        Ok(int) => int,
        Err(_) => 10,
    };
    let mut board = Board::new(board_size);
    println!(
        "Game with {} squares ({}x{} dots)",
        board.size,
        board.dot_size(),
        board.dot_size()
    );
    println!("{}", board.to_string());

    let players = [Hoomin::one(), Hoomin::two()];
    let mut player_index = 0;

    loop {
        let player = &players[player_index];
        println!("Player {}'s turn", player.name());

        let player_edge = player.play(board.clone());
        board.draw(player_edge).expect(&format!(
            "Player {} attempted to play invalid move: {}",
            player.name(),
            player_edge
        ));
        player_index = (player_index + 1) % players.len();
        println!("{}\n", board.to_string())
    }
}

// let pairs: Vec<(u32, u32)> = (0..board_size)
//     .flat_map(|row| (0..board_size).map(move |col| (row, col)))
//     .collect();
// println!("{:?}", pairs);
