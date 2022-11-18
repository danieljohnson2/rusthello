use super::*;
use crate::iterext::*;

/// Contains a move, which is a vec of cell changes to be
//  made to execute the move. The move may be invalid,
/// if it contains no changes to make.
#[derive(Clone, Default)]
pub struct Movement {
    flips: Vec<CellChange>,
}

impl Movement {
    /// Constructs a movement for a location on the board; the move may be
    /// invalid, if the location is not empty or would flip no other cells.
    pub fn new(board: &Board, loc: Loc, cell: Cell) -> Movement {
        if board[loc] == Cell::Empty {
            let mut flips: Vec<_> = Movement::find_flippable_around(board, loc, cell)
                .map(|loc| CellChange::new(cell, loc))
                .collect();
            if !flips.is_empty() {
                flips.insert(0, CellChange::new(cell, loc))
            }
            Movement { flips }
        } else {
            Movement::default()
        }
    }

    /// True if this is a valid, move false if not.
    pub fn is_valid(&self) -> bool {
        !self.flips.is_empty()
    }

    /// Plays a move; it flips the cells indicated by the move. If this move
    /// is invalid, this method does nothing. It removes the flip that it
    /// performs, so that the movement may become invalid.
    ///
    /// This returns true if it flips something, false if was invalid.
    pub fn play_one(&mut self, board: &mut Board) -> bool {
        if self.flips.is_empty() {
            false
        } else {
            board.apply_change(self.flips[0]);
            self.flips.remove(0);
            true
        }
    }

    /// Returns a score for this move; moves with higher scores
    /// are preferred. This returns MIN for invalid moves.
    pub fn get_score(&self, board: &Board) -> usize {
        if self.is_valid() {
            let mut score = self.flips.len() + 100; // stay positive
            let loc = self.flips[0].loc;
            let x_edge = loc.x == 0 || loc.x == board.get_width() - 1;
            let y_edge = loc.y == 0 || loc.y == board.get_height() - 1;

            if x_edge && y_edge {
                score += 100 // prefer corners
            } else if x_edge || y_edge {
                score -= 100 // avoid edges
            }

            score
        } else {
            usize::MIN
        }
    }

    /// Returns an iterator over all locations on the board that would be flipped
    /// by placing 'cell' and 'start'. If the location indicated is not empty
    /// this returns an empty vector.
    fn find_flippable_around(
        board: &Board,
        start: Loc,
        cell: Cell,
    ) -> impl Iterator<Item = Loc> + '_ {
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

        let offsets: &[(isize, isize)] = if board[start] != Cell::Empty {
            &[(0isize, 0isize); 0]
        } else {
            &OFFSETS
        };

        offsets.iter().flat_map(move |&(dx, dy)| {
            let candidates = board.cells_from(start, dx, dy).skip(1);
            Movement::find_flippable(board, cell, candidates)
        })
    }

    /// Finds all cells containing opposed cells to 'cell' starting after
    /// 'start' (not including 'start'!) and running until a location matching
    /// 'cell' is found. If no location matching 'cell' is found, this returns an
    /// empty vector.
    fn find_flippable(
        board: &Board,
        cell: Cell,
        candidates: impl Iterator<Item = Loc>,
    ) -> Vec<Loc> {
        let mut buffer: Vec<Loc> = candidates
            .take_while(|&c| board[c] != Cell::Empty)
            .take_up_to(|&c| board[c] == cell)
            .collect();

        if buffer.last().map(|&l| board[l]) == Some(cell) {
            buffer.pop();
        } else {
            buffer.clear();
        }

        buffer
    }
}

/// This contains a cell to write into a location
/// and the location to put it.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CellChange {
    pub cell: Cell,
    pub loc: Loc,
}

impl CellChange {
    pub fn new(cell: Cell, loc: Loc) -> CellChange {
        CellChange { cell, loc }
    }
}
