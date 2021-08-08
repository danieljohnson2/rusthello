use std::cell::RefCell;
use std::rc::Rc;

use crate::board::*;
use crate::cell::*;
use crate::movement::*;

pub struct Game {
    board: Board,
}

/// A reference to a mutable board, allowing the board
/// to be shared by multiple views.
pub type GameRef = Rc<RefCell<Game>>;

impl Game {
    pub fn new(board: Board) -> Game {
        Game { board }
    }

    /// Creates a GameRef refering to this game,
    /// which is copied into a RefCell.
    pub fn into_ref(self) -> GameRef {
        GameRef::new(RefCell::new(self))
    }

    pub fn to_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_width(&self) -> usize {
        self.board.get_width()
    }

    pub fn get_board_height(&self) -> usize {
        self.board.get_height()
    }

    pub fn get_board_center(&self) -> Loc {
        Loc::new(self.board.get_width() / 2, self.board.get_height() / 2)
    }

    pub fn has_any_moves(&mut self, cell: Cell) -> bool {
        let board = &self.board;
        !board.find_valid_moves(cell).is_empty()
    }

    pub fn place_at(&mut self, loc: Loc, cell: Cell) -> bool {
        let mv = Movement::new(&self.board, loc, cell);

        if mv.is_valid() {
            mv.play(&mut self.board);
            true
        } else {
            false
        }
    }

    pub fn place_ai(&mut self, cell: Cell) -> bool {
        let valid = self.board.find_valid_moves(cell);
        if !valid.is_empty() {
            valid[0].play(&mut self.board);
            true
        } else {
            false
        }
    }
}