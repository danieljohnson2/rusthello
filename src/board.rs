use std::cell::RefCell;
use std::collections::*;
use std::iter;
use std::rc::Rc;

use crate::cell::*;
use std::cmp::*;
use std::ops::*;

/// Holds the state of play; the board is essentially a two dimensional
/// array of cells, but also caches some values used for scoring.
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    cell_counts: HashMap<Cell, usize>,
    game_over: bool,
}

/// A reference to a mutable board, allowing the board
/// to be shared by multiple views.
pub type BoardRef = Rc<RefCell<Board>>;

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
        *board.cell_at_mut(Loc::new(center.x, center.y)) = Black;
        *board.cell_at_mut(Loc::new(center.x - 1, center.y - 1)) = Black;
        *board.cell_at_mut(Loc::new(center.x, center.y - 1)) = White;
        *board.cell_at_mut(Loc::new(center.x - 1, center.y)) = White;

        board.update_board_info();

        board
    }

    /// Creates a BoardRef refering to this board,
    /// which is copied into a RefCell.
    pub fn into_ref(self) -> BoardRef {
        BoardRef::new(RefCell::new(self))
    }

    /// The width of the board.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// The height of the board.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// This adds a delta to a location, and returns the new location so long as
    /// it is in the board; if not it returns None.
    pub fn offset_within(&self, loc: Loc, dx: isize, dy: isize) -> Option<Loc> {
        loc.offset_within(dx, dy, self.width, self.height)
    }

    /// True if the game is over and no moves can be made.
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    /// This counts the number of board cells whose value is 'cell'.
    pub fn count_cells(&self, cell: Cell) -> usize {
        *self.cell_counts.get(&cell).unwrap_or(&0)
    }

    /// True if the location and cell can be played, but does not
    /// play the move.
    pub fn is_valid_move(&self, loc: Loc, cell: Cell) -> bool {
        self.find_flippable_around(loc, cell).next().is_some()
    }

    /// Plays a move; it places a cell at the location indicated, and
    /// flips any other cells that ought to be. If the location given is not
    /// empty, or if this would flip nothing, then this does not play the
    /// move and returns false.
    pub fn place(&mut self, loc: Loc, cell: Cell) -> bool {
        if self[loc] == Cell::Empty {
            let flips: Vec<_> = self.find_flippable_around(loc, cell).collect();

            if !flips.is_empty() {
                *self.cell_at_mut(loc) = cell;

                for f in flips {
                    *self.cell_at_mut(f) = cell
                }

                self.update_board_info();
                return true;
            }
        }

        false
    }

    /// Returns all valid locations where a given cell can be placed.
    /// They are ordered so the one with the most flips is first; the
    /// AI chooses this move.
    pub fn find_valid_moves(&self, cell: Cell) -> Vec<Move> {
        let mut valid = Vec::new();

        if !self.game_over {
            for y in 0..self.height {
                for x in 0..self.width {
                    let loc = Loc::new(x, y);
                    let flip_count = self.find_flippable_around(loc, cell).count();

                    if flip_count > 0 {
                        valid.push(Move { loc, flip_count });
                    }
                }
            }
        }

        valid.sort_by(|left, right| left.get_score(self).cmp(&right.get_score(self)).reverse());

        valid
    }

    /// Returns a vector with all locations on the board that would be flipped
    /// by placing 'cell' and 'start'. If the location indicated is not empty
    /// this returns an empty vector.
    fn find_flippable_around(&self, start: Loc, cell: Cell) -> impl Iterator<Item = Loc> + '_ {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let offsets: &[(isize, isize)] = if self[start] != Cell::Empty {
            &[(0isize, 0isize); 0]
        } else {
            &OFFSETS
        };

        offsets.iter().flat_map(move |(dx, dy)| {
            let candidates = self.cells_from(start, *dx, *dy);
            self.find_flippable(cell, candidates)
        })
    }

    /// This finds all cells containing opposed cells to 'cell' starting after
    /// 'start' (not including 'start'!) and running until a location matching
    /// 'cell' is found. If no location matching 'cell' is found, this returns an
    /// empty vector.
    fn find_flippable(&self, cell: Cell, candidates: impl Iterator<Item = Loc>) -> Vec<Loc> {
        let mut buffer: Vec<Loc> = Vec::new();

        for next in candidates {
            if self[next] == cell {
                return buffer;
            } else if self[next] == cell.to_opposite() {
                buffer.push(next);
            } else {
                break;
            }
        }

        Vec::new()
    }

    // Returns an iterator the gives the locations starting from 'start'
    // and incrementing by (dx, dy)- but 'start' itself is not included.
    // The iterator ends when it runs off the board.
    fn cells_from(&self, start: Loc, dx: isize, dy: isize) -> impl Iterator<Item = Loc> + '_ {
        let mut here = start;
        iter::from_fn(move || {
            if let Some(next) = self.offset_within(here, dx, dy) {
                here = next;
                Some(next)
            } else {
                None
            }
        })
    }

    /// Returns a mutable borrow of the slow indicated by the location;
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
        self.cell_counts.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let here = Loc::new(x, y);
                let cell = self[here];
                let e = self.cell_counts.entry(cell).or_insert(0);
                *e += 1;
            }
        }

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

/// Repreents a position on the board.
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

/// Contains a location to move, and the count
/// of other cells flipped- we use that as a score
/// to decide the best move.
pub struct Move {
    pub loc: Loc,
    pub flip_count: usize,
}

impl Move {
    fn get_score(&self, board: &Board) -> usize {
        let mut score = self.flip_count + 100; // stay positive
        let loc = self.loc;
        let x_edge = loc.x == 0 || loc.x == board.get_width() - 1;
        let y_edge = loc.y == 0 || loc.y == board.get_height() - 1;

        if x_edge && y_edge {
            score += 100 // prefer corners
        } else if x_edge || y_edge {
            score -= 100 // avoid edges
        }

        score
    }
}
