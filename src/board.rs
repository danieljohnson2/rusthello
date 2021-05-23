use crate::cell::*;
use std::ops::*;

/// Holds the state of play; the board is essentially a two dimensional
/// array of cells.
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    /// Creates a new board with the usual pattern of initial
    /// cells- mostly empty.
    pub fn new(width: usize, height: usize) -> Board {
        use Cell::*;

        let cells = vec![Empty; width * height];

        let mut board = Board {
            width,
            height,
            cells,
        };

        let center = Loc::new(width / 2, height / 2);
        board[Loc::new(center.x, center.y)] = Black;
        board[Loc::new(center.x - 1, center.y - 1)] = Black;
        board[Loc::new(center.x, center.y - 1)] = White;
        board[Loc::new(center.x - 1, center.y)] = White;

        board
    }

    /// The width of the board.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// The height of the board.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Plays a move; it places a cell at the location indicated, and
    /// flips any other cells that ought to be. If the location given is not
    /// empty, or if this would flip nothing, then this does not play the
    /// move and returns false.
    pub fn place(&mut self, loc: Loc, cell: Cell) -> bool {
        if self[loc] == Cell::Empty {
            let flips = self.find_flippable_around(loc, cell);

            if !flips.is_empty() {
                self[loc] = cell;

                for f in flips {
                    self[f] = cell
                }
                return true;
            }
        }

        false
    }

    /// Returns all valid locations where a given cell can be placed.
    pub fn find_valid_moves(&self, cell: Cell) -> Vec<Loc> {
        let mut valid = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let here = Loc::new(x, y);

                if self.is_valid_move(here, cell) {
                    valid.push(here)
                }
            }
        }

        valid
    }

    /// True if the location and cell can be played, but does not
    /// play the move.
    pub fn is_valid_move(&self, loc: Loc, cell: Cell) -> bool {
        !self.find_flippable_around(loc, cell).is_empty()
    }

    /// Returns a vector with all locations on the board that would be flipped
    /// by placing 'cell' and 'start'. If the location indicated is not empty
    // this returns an empty vector.
    fn find_flippable_around(&self, start: Loc, cell: Cell) -> Vec<Loc> {
        let mut buffer: Vec<Loc> = Vec::new();

        if self[start] == Cell::Empty {
            buffer.append(&mut self.find_flippable(start, cell, -1, -1));
            buffer.append(&mut self.find_flippable(start, cell, -1, 0));
            buffer.append(&mut self.find_flippable(start, cell, -1, 1));

            buffer.append(&mut self.find_flippable(start, cell, 0, -1));
            buffer.append(&mut self.find_flippable(start, cell, 0, 1));

            buffer.append(&mut self.find_flippable(start, cell, 1, -1));
            buffer.append(&mut self.find_flippable(start, cell, 1, 0));
            buffer.append(&mut self.find_flippable(start, cell, 1, 1));
        }

        buffer
    }

    /// This finds all cells containing opposed cells to 'cell' starting after
    /// 'start' (not including 'start'!) and running until a location matching
    /// 'cell' is found. If no location matching 'cell' is found, this returns an
    /// empty vector.
    fn find_flippable(&self, start: Loc, cell: Cell, dx: isize, dy: isize) -> Vec<Loc> {
        let mut buffer: Vec<Loc> = Vec::new();
        let mut here = start;

        loop {
            if let Some(next) = self.offset_within(here, dx, dy) {
                here = next;
                if self[here] == cell {
                    return buffer;
                } else if self[here] == cell.to_opposite() {
                    buffer.push(here);
                    continue;
                }
            }
            return Vec::new();
        }
    }

    /// This adds a delta to a location, and returns the new location so long as
    /// it is in the board; if not it returns None.
    pub fn offset_within(&self, loc: Loc, dx: isize, dy: isize) -> Option<Loc> {
        if let Some(x) = add(loc.x, dx) {
            if let Some(y) = add(loc.y, dy) {
                if x < self.width && y < self.height {
                    return Some(Loc::new(x, y));
                }
            }
        }

        return None;

        #[allow(clippy::comparison_chain)]
        fn add(left: usize, right: isize) -> Option<usize> {
            if right > 0 {
                left.checked_add(right as usize)
            } else if right < 0 {
                left.checked_sub((-right) as usize)
            } else {
                Some(left)
            }
        }
    }
}

impl Index<Loc> for Board {
    type Output = Cell;

    fn index(&self, index: Loc) -> &Self::Output {
        let idx = index.y * self.height + index.x;
        &self.cells[idx]
    }
}

impl IndexMut<Loc> for Board {
    fn index_mut(&mut self, index: Loc) -> &mut Self::Output {
        let idx = index.y * self.height + index.x;
        &mut self.cells[idx]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Loc {
    pub x: usize,
    pub y: usize,
}

impl Loc {
    pub fn new(x: usize, y: usize) -> Loc {
        Loc { x, y }
    }
}
