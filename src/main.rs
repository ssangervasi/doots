use clap::{App, Arg};
use textwrap::dedent as dd;

use doots::game::board::BoardSize;
use doots::game::engine::{run_game, Opts};

fn main() {
    match cli() {
        Ok(_) => {}
        Err(msg) => eprintln!("{}", msg),
    }
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
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("In quiet mode the board is not printed until the end."),
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

    let quiet = matches.occurrences_of("quiet") > 0;

    let player_one = matches.value_of("player_one").unwrap().to_string();
    let player_two = matches.value_of("player_two").unwrap().to_string();

    run_game(&Opts {
        board_size,
        player_one,
        player_two,
        quiet,
    })
}
