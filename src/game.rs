use std::cell::RefCell;
use std::rc::Rc;
use std::time::*;

use crate::board::*;
use crate::cell::*;
use crate::movement::*;

pub struct Game {
    board: Board,
    next_move: Cell,
    ongoing_movement: Movement,
    start: Instant,
}

/// A reference to a mutable board, allowing the board
/// to be shared by multiple views.
pub type GameRef = Rc<RefCell<Game>>;

impl Game {
    pub fn new(board: Board) -> Game {
        Game {
            board,
            next_move: Cell::Black,
            ongoing_movement: Movement::default(),
            start: Instant::now(),
        }
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

    pub fn check_move(&mut self) -> Cell {
        if self.ongoing_movement.is_valid() {
            if self.start.elapsed() >= Duration::from_millis(100) {
                self.start = Instant::now()
            } else {
                return Cell::Empty;
            }

            if self.ongoing_movement.play_one(&mut self.board) {
                if !self.ongoing_movement.is_valid() {
                    let f = self.next_move.flipped();

                    if self.has_any_moves(f) {
                        self.next_move = f;
                    }
                }
            }
        }

        self.next_move
    }

    pub fn begin_movement(&mut self, mv: Movement) -> bool {
        if self.ongoing_movement.is_valid() {
            false
        } else if mv.is_valid() {
            self.ongoing_movement = mv;
            true
        } else {
            false
        }
    }

    pub fn get_player_movement(&self, loc: Loc) -> Movement {
        if self.next_move != Cell::Empty {
            Movement::new(&self.board, loc, self.next_move)
        } else {
            Movement::default()
        }
    }

    pub fn get_ai_movement(&self) -> Movement {
        if self.next_move != Cell::Empty {
            let valid = self.board.find_valid_moves(self.next_move);
            valid.into_iter().next().unwrap_or(Movement::default())
        } else {
            Movement::default()
        }
    }
}
