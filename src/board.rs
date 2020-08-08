use crate::box_drawings::{lookup, BoxChar};

// mod box_drawings;

pub type BoardSize = u8;

#[derive(Default, Copy, Clone)]
pub struct Dot {
    pub row: BoardSize,
    pub col: BoardSize,
}

pub type Edge = (Dot, Dot);

#[derive(Default, Clone)]
pub struct Board {
    pub size: BoardSize,
    pub edges: Vec<Edge>,
}

impl Board {
    pub fn new(size: BoardSize) -> Board {
        Board {
            size: size,
            ..Default::default()
        }
    }

    pub fn to_string(&self) -> String {
        let mut grid = String::new();
        for row in 0..=self.size {
            for col in 0..=self.size {
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
                let entry = self.choose_char(Dot { row, col });
                grid.push(entry.value);
                grid.push_str(if entry.right { "──" } else { "  " });
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
            value: '.',
        })
    }
}
