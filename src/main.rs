use clap::{App, Arg};

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

    let game_size: u32 = match matches.value_of("size").unwrap().trim().parse() {
        Ok(int) => int,
        Err(_) => 10,
    };
    println!("Game of size {}", game_size);

    let mut grid = String::new();
    for row in 0..=game_size {
        // grid += format!("\n{}", 1);
        for col in 0..=game_size {
            if row == 0 && col == 0 {
                grid.push_str("  ");
                continue;
            } else if row == 0 {
                grid.push_str(&format!("{:2} ", col));
                continue;
            } else if col == 0 {
                grid.push_str(&format!("\n{:2} ", row));
                continue;
            }
            let entry = choose_char(row, col);
            grid.push(entry.value);
            grid.push_str(if entry.right { "──" } else { "  " });
        }
    }
    print!("{}", grid)
}

fn choose_char(row: u32, col: u32) -> box_drawings::BoxChar {
    box_drawings::lookup(box_drawings::BoxChar {
        up: (5 <= row && row <= 8) || (6 <= row - 1 && row - 1 <= 7),
        right: 5 <= col && col <= 8,
        down: (6 <= row && row <= 7) || (5 <= row + 1 && row + 1 <= 8),
        left: 6 <= col && col <= 7,
        value: '.',
    })
}

// let pairs: Vec<(u32, u32)> = (0..game_size)
//     .flat_map(|row| (0..game_size).map(move |col| (row, col)))
//     .collect();
// println!("{:?}", pairs);
