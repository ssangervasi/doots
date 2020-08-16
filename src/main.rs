use clap::{App, Arg};
use textwrap::dedent as dd;

use doots::game::board::{Board, BoardSize, WinnerResult};
use doots::players::choose::choose;

fn main() {
    match cli() {
        Ok(_) => println!("Done."),
        Err(msg) => eprintln!("{}", msg),
    }
}

struct CLIOpts {
    board_size: BoardSize,
    player_two: String,
    player_one: String,
}

const SIZE_DEFAULT: BoardSize = 10;
const SIZE_DEFAULT_STR: &str = "10";
const SIZE_MIN: BoardSize = 1;
const SIZE_MAX: BoardSize = 100;

fn cli() -> Result<(), String> {
    let matches = App::new("doots")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .default_value(SIZE_DEFAULT_STR)
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

    let size_str = matches.value_of("size").unwrap().trim();
    let board_size = match size_str.parse::<BoardSize>() {
        Err(_) => {
            eprintln!(
                "Size {} doesn't make sense so I'm going to use {}.",
                size_str, SIZE_DEFAULT
            );
            SIZE_DEFAULT
        }
        Ok(size_parsed) => {
            if size_parsed < SIZE_MIN {
                eprintln!(
                    "{} squares is a bit small so I'm going to use {} instead.",
                    size_parsed,
                    &SIZE_DEFAULT.to_string()
                );
                SIZE_DEFAULT
            } else if SIZE_MAX < size_parsed {
                eprintln!(
                    "{} squares? Ain't nobody got time for that! Let's just do {}",
                    size_parsed, SIZE_MAX
                );
                SIZE_MAX
            } else {
                size_parsed
            }
        }
    };

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

    for turn in 0..(board.edge_count() as usize) {
        let player_index = turn % players.len();
        let (player_id, player) = &players[player_index];
        println!("Player {}'s turn", player_id);

        // Note that the board clone is intentional as we don't want our
        // players to have any way of mutating the offical board state.
        let player_edge = player.play(board.clone());
        match board.draw(player_edge) {
            Err(_) => {
                return Err(format!(
                    "Player {} ({}) attempted to draw an invalid edge: {}",
                    player_id,
                    player.name(),
                    player_edge,
                ));
            }
            _ => {}
        };
        println!("Player {} drew: {}", player_id, player_edge);

        println!("{}\n", board.to_string());
    }

    print!(
        "{}",
        dd(&format!(
            "
            · ──────────────── ·
            │ GAME OVER        │
            │ Player {} wins! │
            · ──────────────── ·
            ",
            match board.winner() {
                WinnerResult::Winner(_, _) => "somebody",
                _ => "nobody",
            }
        ))
    );

    Ok(())
}

// let pairs: Vec<(u32, u32)> = (0..board_size)
//     .flat_map(|row| (0..board_size).map(move |col| (row, col)))
//     .collect();
// println!("{:?}", pairs);
