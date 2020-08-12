use std::io;

use lazy_static::lazy_static;
use regex::Regex;

use crate::board::{BoardSize, Dot};

pub fn read_dot() -> Dot {
    lazy_static! {
        static ref DOT_RE: Regex = Regex::new(r"^(?P<row>\d+)\D+(?P<col>\d+)$").unwrap();
    }

    let mut row: Option<BoardSize> = None;
    let mut col: Option<BoardSize> = None;
    loop {
        let response = read_trimmed();
        match DOT_RE.captures(&response) {
            Some(caps) => {
                // Clean this up?
                caps.name("row")
                    .map(|v| v.as_str().parse::<BoardSize>().map(|n| row = Some(n)));
                caps.name("col")
                    .map(|v| v.as_str().parse::<BoardSize>().map(|n| col = Some(n)));
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
