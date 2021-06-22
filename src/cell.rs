use std::fmt;

/// Lists the states a cell on the board can be in.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    White,
    Black,
}

impl Cell {
    /// Returns the text to display for a cell
    pub fn to_str(self) -> &'static str {
        match self {
            Cell::Empty => " ",
            Cell::White => "○",
            Cell::Black => "●",
        }
    }

    /// Returns the reversed cell; white for black and
    /// black for white. Empty cells are returned unchanged.
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
