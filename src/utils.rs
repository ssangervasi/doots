use std::io;

use lazy_static::lazy_static;
use regex::Regex;

use crate::board::Dot;

// pub fn read_num() -> u32 {
//     loop {
//         let mut response = String::new();
//         io::stdin()
//             .read_line(&mut response)
//             .expect("Failed to read line");

//         match response.trim().parse() {
//             Ok(n) => return n,
//             Err(_) => {
//                 println!("! Please enter a number !");
//             }
//         };
//     }
// }

pub fn read_dot() -> Dot {
    lazy_static! {
        static ref DOT_RE: Regex = Regex::new(r"^(?P<row>\d+)\D+(?P<col>\d+)$").unwrap();
    }

    let mut row: Option<u8> = None;
    let mut col: Option<u8> = None;
    loop {
        let response = read_trimmed();
        match DOT_RE.captures(&response) {
            Some(caps) => {
                // Clean this up?
                caps.name("row")
                    .map(|v| v.as_str().parse::<u8>().map(|n| row = Some(n)));
                caps.name("col")
                    .map(|v| v.as_str().parse::<u8>().map(|n| col = Some(n)));
            }
            _ => {
                // println!("No match: {:?} for {:?}", response, DOT_RE.to_string());
            }
        }

        match (row, col) {
            (Some(r), Some(c)) => return Dot { row: r, col: c },
            _ => {
                println!("Enter two numbers like: 3 15");
            }
        }
    }
}

pub fn read_trimmed() -> String {
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.trim().to_string()
}
