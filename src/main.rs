use clap::{App, Arg};
use textwrap::dedent as dd;

mod board;
mod box_drawings;
mod utils;

use crate::board::{Board, BoardSize, Dot};
use crate::utils::read_dot;

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
    println!("Game of size {}", board_size);
    let mut board = Board::new(board_size);

    println!("Drawing sample box:");
    board
        .draw((Dot { row: 2, col: 2 }, Dot { row: 2, col: 3 }))
        .expect("Shit");
    board
        .draw((Dot { row: 2, col: 3 }, Dot { row: 3, col: 3 }))
        .expect("Shit");
    board
        .draw((Dot { row: 3, col: 2 }, Dot { row: 3, col: 3 }))
        .expect("Shit");
    board
        .draw((Dot { row: 2, col: 2 }, Dot { row: 3, col: 2 }))
        .expect("Shit");
    println!("{}", board.to_string());

    loop {
        println!("Draw an edge (row, col) -> (row, col):");
        println!("from: ");
        let dot_from = read_dot();
        println!("to  : ");
        let dot_to = read_dot();
        println!("{:?} -> {:?}", dot_from, dot_to);

        match board.draw((dot_from, dot_to)) {
            Ok(_) => println!("{}", board.to_string()),
            Err(msg) => {
                println!("{}", msg);
                println!("Try again.");
            }
        }
    }
}

// let pairs: Vec<(u32, u32)> = (0..board_size)
//     .flat_map(|row| (0..board_size).map(move |col| (row, col)))
//     .collect();
// println!("{:?}", pairs);
