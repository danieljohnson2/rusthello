use cursive::*;
use std::fmt;
use std::ops::*;

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

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        use Cell::*;

        let cells = vec![Empty; width * height];

        let mut board = Board {
            width,
            height,
            cells,
        };

        let center = Vec2::new(width / 2, height / 2);
        board[Vec2::new(center.x, center.y)] = Black;
        board[Vec2::new(center.x - 1, center.y - 1)] = Black;
        board[Vec2::new(center.x, center.y - 1)] = White;
        board[Vec2::new(center.x - 1, center.y)] = White;

        board
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn place(&mut self, loc: Vec2, cell: Cell) -> bool {
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

    pub fn find_valid_moves(&self, cell: Cell) -> Vec<Vec2> {
        let mut valid = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let here = Vec2::new(x, y);

                if self.is_valid_move(here, cell) {
                    valid.push(here)
                }
            }
        }

        valid
    }

    pub fn is_valid_move(&self, loc: Vec2, cell: Cell) -> bool {
        !self.find_flippable_around(loc, cell).is_empty()
    }

    fn find_flippable_around(&self, start: Vec2, cell: Cell) -> Vec<Vec2> {
        let mut buffer: Vec<Vec2> = Vec::new();

        if self[start] == Cell::Empty {
            buffer.append(&mut self.find_flippable(start, cell, XY::new(-1, -1)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(-1, 0)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(-1, 1)));

            buffer.append(&mut self.find_flippable(start, cell, XY::new(0, -1)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(0, 1)));

            buffer.append(&mut self.find_flippable(start, cell, XY::new(1, -1)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(1, 0)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(1, 1)));
        }

        buffer
    }

    fn find_flippable(&self, start: Vec2, cell: Cell, delta: XY<isize>) -> Vec<Vec2> {
        let mut buffer: Vec<Vec2> = Vec::new();
        let mut here = start;

        loop {
            if let Some(next) = self.offset_within(here, delta) {
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

    fn offset_within(&self, vec: Vec2, delta: XY<isize>) -> Option<Vec2> {
        if let Some(next) = vec.checked_add(delta) {
            if next.x < self.width && next.y < self.height {
                return Some(next);
            }
        }
        None
    }
}

impl Index<Vec2> for Board {
    type Output = Cell;

    fn index(&self, index: Vec2) -> &Self::Output {
        let idx = index.y * self.height + index.x;
        &self.cells[idx]
    }
}

impl IndexMut<Vec2> for Board {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        let idx = index.y * self.height + index.x;
        &mut self.cells[idx]
    }
}
