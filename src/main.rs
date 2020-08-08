use clap::{App, Arg};

mod board;
mod box_drawings;

fn main() {
    let matches = App::new("doots")
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .default_value("10")
                .help(
                    "
                    How many boxes wide and tall the game is, ex:
                        size 1 => 1x1 grid => 4 dots
                        size 2 => 2x2 grid => 9 dots
                    ",
                ),
        )
        .get_matches();

    let board_size: board::BoardSize = match matches.value_of("size").unwrap().trim().parse() {
        Ok(int) => int,
        Err(_) => 10,
    };
    println!("Game of size {}", board_size);
    let board = board::Board::new(board_size);
    println!("{}", board.to_string())
}

fn choose_char(row: board::BoardSize, col: board::BoardSize) -> box_drawings::BoxChar {
    box_drawings::lookup(box_drawings::BoxChar {
        up: (5 <= row && row <= 8) || (6 <= row - 1 && row - 1 <= 7),
        right: 5 <= col && col <= 8,
        down: (6 <= row && row <= 7) || (5 <= row + 1 && row + 1 <= 8),
        left: 6 <= col && col <= 7,
        value: '.',
    })
}

// let pairs: Vec<(u32, u32)> = (0..board_size)
//     .flat_map(|row| (0..board_size).map(move |col| (row, col)))
//     .collect();
// println!("{:?}", pairs);
