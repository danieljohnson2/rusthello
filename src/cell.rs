use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    White,
    Black,
}

impl Cell {
    pub fn to_str(self) -> &'static str {
        match self {
            Cell::Empty => " ",
            Cell::White => "O",
            Cell::Black => "X",
        }
    }

    pub fn to_opposite(self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::White => Cell::Black,
            Cell::Black => Cell::White,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}
