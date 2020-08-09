use std::ops::Sub;

use crate::box_drawings::{lookup, BoxChar, LINE_H};

pub type BoardSize = u8;

#[derive(Default, Copy, Clone, Debug, Eq)]
pub struct Dot {
    pub row: BoardSize,
    pub col: BoardSize,
}

impl PartialEq for Dot {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
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

fn has_dot(edge: Edge, dot: Dot) -> bool {
    let (d1, d2) = edge;
    d1 == dot || d2 == dot
}

#[derive(Default, Clone, Debug)]
pub struct Board {
    pub size: BoardSize,
    pub edges: Vec<Edge>,
}

impl Board {
    pub fn dot_count() -> u8 {}

    pub fn new(size: BoardSize) -> Board {
        Board {
            size,
            ..Default::default()
        }
    }

    pub fn draw(&mut self, edge: Edge) -> Result<Edge, String> {
        if !is_valid(edge) {
            return Err(format!("Cannot draw invalid edge: {:?}", edge));
        } else if !self.is_free(edge) {
            return Err(format!("Cannot redraw edge: {:?}", edge));
        }
        self.edges.push(edge);
        Ok(edge)
    }

    pub fn is_free(&self, (d1, d2): Edge) -> bool {
        for connected in self.find_connected(d1).iter() {
            if d2 == *connected {
                return false;
            }
        }
        true
    }

    /* Whether the dot fits in this board. */
    pub fn contains(&self, dot: Dot) -> bool {
        dot.x < self.size
    }

    /**
     * O(n). Should be able to make this O(1) by mapping...
     */
    pub fn find_edges(&self, dot: Dot) -> Vec<Edge> {
        let mut edges: Vec<Edge> = vec![];
        for edge in self.edges.iter() {
            if has_dot(*edge, dot) {
                edges.push(*edge)
            }
        }
        edges
    }

    pub fn find_connected(&self, dot: Dot) -> Vec<Dot> {
        // let mut dots: Vec<Dot> = vec![];
        self.find_edges(dot)
            .iter()
            .map(|(d1, d2)| if dot == *d1 { *d2 } else { *d1 })
            .collect()
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
        let mut box_char = BoxChar::default();
        for connected in self.find_connected(dot).iter() {
            // Note that only one of these can be true because dots are not
            // allowed to connect diagonally.
            if connected.row < dot.row {
                box_char.up = true
            } else if dot.col < connected.col {
                box_char.right = true
            } else if dot.row < connected.row {
                box_char.down = true
            } else if connected.col < dot.col {
                box_char.left = true
            }
        }
        lookup(box_char)
    }
}
