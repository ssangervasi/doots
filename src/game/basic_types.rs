use core::fmt;
use std::cmp::Ordering;
use std::ops;

use crate::game::box_drawings::{DOT, LINE_H};
use crate::players::player::PlayerId;

pub type BoardSize = u16;

/*
 * Shorthand constructors to create dots and edges from tuples.
 */
pub fn dot(row: BoardSize, col: BoardSize) -> Dot {
    Dot { row, col }
}
pub fn edge((r1, c1): (BoardSize, BoardSize), (r2, c2): (BoardSize, BoardSize)) -> Edge {
    Edge(dot(r1, c1), dot(r2, c2))
}

/*
 * Dot
 */
#[derive(Default, Copy, Clone, Debug, Eq)]
pub struct Dot {
    pub row: BoardSize,
    pub col: BoardSize,
}

impl Dot {
    pub fn transpose(&self) -> Self {
        dot(self.row, self.col)
    }
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl PartialEq for Dot {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Ord for Dot {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.col.cmp(&other.row),
            ne_result => ne_result,
        }
    }
}

impl PartialOrd for Dot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Sub for Dot {
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

impl ops::Add for Dot {
    type Output = Dot;

    fn add(self, other: Dot) -> Dot {
        Dot {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

/*
 * Edge
 */
#[derive(Default, Copy, Clone, Debug, Eq)]
pub struct Edge(pub Dot, pub Dot);

impl Edge {
    pub fn is_valid(&self) -> bool {
        let diff = self.1 - self.0;
        diff.row + diff.col == 1
    }

    pub fn has_dot(&self, dot: Dot) -> bool {
        self.0 == dot || self.1 == dot
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}", self.0, DOT, LINE_H, DOT, self.1)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let Edge(s1, s2) = self;
        let Edge(o1, o2) = other;
        (s1 == o1 && s2 == o2) || (s1 == o2 && s2 == o1)
    }
}

/*
 * Dot
 */
#[derive(Default, Copy, Clone, Debug, Eq)]
pub struct DotBox(pub Dot);

impl DotBox {
    pub fn upper_left(&self) -> Dot {
        self.0
    }

    pub fn upper_right(&self) -> Dot {
        self.upper_left() + dot(0, 1)
    }

    pub fn lower_right(&self) -> Dot {
        self.upper_left() + dot(1, 1)
    }

    pub fn lower_left(&self) -> Dot {
        self.upper_left() + dot(1, 0)
    }

    pub fn top(&self) -> Edge {
        Edge(self.upper_left(), self.upper_right())
    }

    pub fn right(&self) -> Edge {
        Edge(self.upper_right(), self.lower_right())
    }

    pub fn bottom(&self) -> Edge {
        Edge(self.lower_left(), self.lower_right())
    }

    pub fn left(&self) -> Edge {
        Edge(self.upper_left(), self.lower_left())
    }

    pub fn edges(&self) -> Vec<Edge> {
        vec![self.top(), self.right(), self.bottom(), self.left()]
    }
}

impl PartialEq for DotBox {
    fn eq(&self, other: &Self) -> bool {
        self.upper_left() == other.upper_left()
    }
}

/*
 * WinnerResult
 */
#[derive(Clone, Debug, Eq)]
pub enum WinnerResult {
    Winner(PlayerId, usize),
    Tie(Vec<PlayerId>, usize),
    None,
}

impl PartialEq for WinnerResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Winner(id_a, c_a), Self::Winner(id_b, c_b)) => id_a == id_b && c_a == c_b,
            (Self::Tie(ids_a, c_a), Self::Tie(ids_b, c_b)) => {
                c_a == c_b && {
                    // 😬
                    let mut sids_a = ids_a.clone();
                    let mut sids_b = ids_b.clone();
                    sids_a.sort();
                    sids_b.sort();
                    sids_a == sids_b
                }
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}
