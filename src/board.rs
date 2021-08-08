use std::cmp::*;
use std::collections::*;
use std::iter;
use std::ops::*;

use crate::cell::*;
use crate::movement::*;

/// Holds the state of play; the board is essentially a two dimensional
/// array of cells, but also caches some values used for scoring.
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    cell_counts: HashMap<Cell, usize>,
    game_over: bool,
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
            cell_counts: HashMap::new(),
            game_over: false,
        };

        let center = Loc::new(width / 2, height / 2);
        *board.cell_at_mut(Loc::new(center.x, center.y)) = White;
        *board.cell_at_mut(Loc::new(center.x - 1, center.y - 1)) = White;
        *board.cell_at_mut(Loc::new(center.x, center.y - 1)) = Black;
        *board.cell_at_mut(Loc::new(center.x - 1, center.y)) = Black;

        board.update_board_info();

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

    // The center location in the board
    pub fn get_board_center(&self) -> Loc {
        Loc::new(self.width / 2, self.height / 2)
    }

    /// This adds a delta to a location, and returns the new location so long as
    /// it is in the board; if not it returns None.
    pub fn offset_within(&self, loc: Loc, dx: isize, dy: isize) -> Option<Loc> {
        loc.offset_within(dx, dy, self.width, self.height)
    }

    /// Returns an iterator over all the locations that are in the board.
    pub fn locations(&self) -> impl Iterator<Item = Loc> + '_ {
        let width = self.get_width();
        let height = self.get_height();
        (0..height).flat_map(move |y| (0..width).map(move |x| Loc::new(x, y)))
    }

    /// True if the game is over and no moves can be made.
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    /// This counts the number of board cells whose value is 'cell'.
    pub fn count_cells(&self, cell: Cell) -> usize {
        *self.cell_counts.get(&cell).unwrap_or(&0)
    }

    /// Returns all valid locations where a given cell can be placed.
    /// They are ordered so the one with the most flips is first; the
    /// AI chooses this move.
    pub fn find_valid_moves(&self, cell: Cell) -> Vec<Movement> {
        if !self.game_over {
            let moves = self.locations().map(|loc| Movement::new(self, loc, cell));

            let mut valid: Vec<_> = moves.filter(|m| m.is_valid()).collect();
            valid.sort_by(|left, right| left.get_score(self).cmp(&right.get_score(self)).reverse());
            valid
        } else {
            Vec::new()
        }
    }

    /// Applies a cell change to the board and returns true if any
    /// changes were made. If so, it also updates the board info.
    pub fn apply_change(&mut self, change: CellChange) -> bool {
        let mut changed = false;

        let cell = self.cell_at_mut(change.loc);

        if *cell != change.cell {
            *cell = change.cell;
            changed = true
        }
        if changed {
            self.update_board_info();
        }

        changed
    }

    // Returns an iterator the gives the locations starting from 'start'
    // and incrementing by (dx, dy). The iterator ends when it runs off the board.
    pub fn cells_from(&self, start: Loc, dx: isize, dy: isize) -> impl Iterator<Item = Loc> + '_ {
        iter::successors(Some(start), move |&l| self.offset_within(l, dx, dy))
    }

    /// Returns a mutable borrow of the slot indicated by the location;
    /// if you modify it, call update_board_info() to update the statistics
    /// we keep. You can mutate many cells before the update, though.
    fn cell_at_mut(&mut self, index: Loc) -> &mut Cell {
        let idx = index.y * self.height + index.x;
        &mut self.cells[idx]
    }

    /// Updates the state of hte board to reflect the
    /// cell array; this updates the counts of cells and
    /// game over flag.
    fn update_board_info(&mut self) {
        let mut cell_counts = HashMap::new();

        for here in self.locations() {
            let cell = self[here];
            let e = cell_counts.entry(cell).or_insert(0);
            *e += 1;
        }

        self.cell_counts = cell_counts;
        self.game_over = self.find_valid_moves(Cell::White).is_empty()
            && self.find_valid_moves(Cell::Black).is_empty();
    }
}

impl Index<Loc> for Board {
    type Output = Cell;

    fn index(&self, index: Loc) -> &Self::Output {
        let idx = index.y * self.height + index.x;
        &self.cells[idx]
    }
}

/// Represents a position on the board.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Loc {
    pub x: usize,
    pub y: usize,
}

impl Loc {
    /// Creates a new Loc with the given co-ordinates.
    pub fn new(x: usize, y: usize) -> Loc {
        Loc { x, y }
    }

    /// This adds a delta to a location, and returns the new location so long as
    /// it is in the indicated rage; if not it returns None. It can have an x
    /// co-ordinate from 0 to width-1, and y can go from 0 to height-1.
    pub fn offset_within(self, dx: isize, dy: isize, width: usize, height: usize) -> Option<Loc> {
        if let Some(x) = add(self.x, dx) {
            if let Some(y) = add(self.y, dy) {
                if x < width && y < height {
                    return Some(Loc::new(x, y));
                }
            }
        }

        return None;

        fn add(left: usize, right: isize) -> Option<usize> {
            match right.cmp(&0) {
                Ordering::Greater => left.checked_add(right as usize),
                Ordering::Less => left.checked_sub((-right) as usize),
                Ordering::Equal => Some(left),
            }
        }
    }
}
