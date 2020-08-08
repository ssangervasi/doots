use std::ops::Sub;

use crate::box_drawings::{lookup, BoxChar, DOT, LINE_H};

pub type BoardSize = u8;

#[derive(Default, Copy, Clone, Debug)]
pub struct Dot {
    pub row: BoardSize,
    pub col: BoardSize,
}

impl Sub for Dot {
    type Output = Dot;

    fn sub(self, other: Dot) -> Dot {
        Dot {
            row: abs_sub(self.row, other.row),
            col: abs_sub(self.col, other.col),
        }
    }
}

fn abs_sub(a: BoardSize, b: BoardSize) -> BoardSize {
    ((a as i8) - (b as i8)).abs() as BoardSize
}

pub type Edge = (Dot, Dot);

fn is_valid(edge: Edge) -> bool {
    let (d1, d2) = edge;
    let diff = d2 - d1;
    diff.row + diff.col == 1
}

#[derive(Default, Clone, Debug)]
pub struct Board {
    pub size: BoardSize,
    pub edges: Vec<Edge>,
}

impl Board {
    pub fn new(size: BoardSize) -> Board {
        Board {
            size,
            ..Default::default()
        }
    }

    pub fn draw(&mut self, edge: Edge) {
        if !is_valid(edge) {
            panic!("Cannot draw invalid edge: {:?}", edge);
        }
        self.edges.push(edge);
    }

    pub fn to_string(&self) -> String {
        let mut grid = String::new();
        for row in 0..=self.size {
            for col in 0..=self.size {
                // Handle header row and left column:
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

                // Othwerwise, pick the appropriate box intersection:
                let entry = self.choose_char(Dot { row, col });
                grid.push(entry.value);

                // Extend right to account for horizontal space:
                let spacer = if entry.right {
                    LINE_H.to_string().repeat(2)
                } else {
                    "  ".to_string()
                };
                grid.push_str(&spacer)
            }
        }
        grid
    }

    pub fn choose_char(&self, dot: Dot) -> BoxChar {
        lookup(BoxChar {
            up: dot.row == 1,
            right: false,
            down: false,
            left: false,
            value: DOT,
        })
    }
}
