use std::io;

use clap::App;

mod box_drawings;

fn main() {
    App::new("doots").get_matches();

    println!("Game size:");

    // let game_size = read_num();
    let game_size: u32 = 20;
    println!("Game of size {}", game_size);

    let pairs: Vec<(u32, u32)> = (0..game_size)
        .flat_map(|row| (0..game_size).map(move |col| (row, col)))
        .collect();
    println!("{:?}", pairs);

    let mut grid = String::new();
    for row in 0..game_size {
        for col in 0..game_size {
            // grid += if col % 3 < 2 {
            //     " | "
            // } else if row % 3 < 2 {
            //     "___"
            // } else {
            //     " · "
            // }
            let entry = box_drawings::lookup(box_drawings::BoxChar {
                up: 5 <= row && row <= 8,
                right: 5 <= col && col <= 8,
                down: 6 <= row && row <= 7,
                left: 6 <= col && col <= 7,
                value: '.',
            })
            .value;
            // grid.push(' ');
            grid.push(entry);
            // grid.push(' ');
        }
        grid += "\n"
    }
    print!("{}", grid)
}

fn read_num() -> u32 {
    loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("! Please enter a number !");
            }
        };
    }
}
