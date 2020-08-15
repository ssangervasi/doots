use clap::{App, Arg};
use textwrap::dedent as dd;

use doots::game::board::{Board, BoardSize};
use doots::players::choose::choose;

fn main() {
    match cli() {
        Ok(_) => println!("Done."),
        Err(msg) => println!("{}", msg),
    }
}

struct CLIOpts {
    board_size: BoardSize,
    player_two: String,
    player_one: String,
}

fn cli() -> Result<(), String> {
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
        .arg(
            Arg::with_name("player_one")
                .short("1")
                .long("player-one")
                .takes_value(true)
                .default_value("hoomin")
                .help("Player one type"),
        )
        .arg(
            Arg::with_name("player_two")
                .short("2")
                .long("player-two")
                .takes_value(true)
                .default_value("hoomin")
                .help("Player two type"),
        )
        .get_matches();

    let board_size: BoardSize = match matches.value_of("size").unwrap().trim().parse() {
        Ok(int) => int,
        Err(_) => 10,
    };
    if 100 < board_size {
        return Err(format!(
            "{} squares? Ain't nobody got time for that!",
            board_size
        ));
    }

    let player_one = matches.value_of("player_one").unwrap().to_string();
    let player_two = matches.value_of("player_two").unwrap().to_string();

    run_game(&CLIOpts {
        board_size,
        player_one,
        player_two,
    })
}

fn run_game(cli_opts: &CLIOpts) -> Result<(), String> {
    let mut board = Board::new(cli_opts.board_size);
    println!(
        "Game with {} squares ({}x{} dots)",
        board.size(),
        board.dot_size(),
        board.dot_size()
    );
    println!("{}", board.to_string());

    let players = choose(&cli_opts.player_one, &cli_opts.player_two);
    let mut player_index = 0;

    loop {
        let (player_id, player) = &players[player_index];
        println!("Player ({:?})'s turn", player_id);

        let player_edge = player.play(board.clone());
        match board.draw(player_edge) {
            Err(_) => {
                return Err(format!(
                    "Player {:?} ({}) attempted to play invalid move: {}",
                    player_id,
                    player.name(),
                    player_edge,
                ));
            }
            _ => {}
        };

        player_index = (player_index + 1) % players.len();
        println!("{}\n", board.to_string());

        if board.is_full() {
            break;
        };
    }

    Ok(())
}

// let pairs: Vec<(u32, u32)> = (0..board_size)
//     .flat_map(|row| (0..board_size).map(move |col| (row, col)))
//     .collect();
// println!("{:?}", pairs);
