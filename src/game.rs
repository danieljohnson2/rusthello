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
    next_move_time: Instant,
}

/// A reference to a mutable board, allowing the board
/// to be shared by multiple views.
pub type GameRef = Rc<RefCell<Game>>;

impl Game {
    pub fn new(board: Board) -> Game {
        let start = Instant::now();

        Game {
            board,
            next_move: Cell::Black,
            ongoing_movement: Movement::default(),
            next_move_time: start,
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

    pub fn has_any_moves(&mut self, cell: Cell) -> bool {
        let board = &self.board;
        !board.find_valid_moves(cell).is_empty()
    }

    pub fn check_move(&mut self) -> Cell {
        if self.ongoing_movement.is_valid() {
            let now = Instant::now();
            while now >= self.next_move_time {
                self.next_move_time += Duration::from_millis(100);

                if !self.ongoing_movement.play_one(&mut self.board) {
                    break;
                }

                if !self.ongoing_movement.is_valid() {
                    let f = self.next_move.flipped();

                    if self.has_any_moves(f) {
                        self.next_move = f;
                    }
                }
            }
        }

        if self.ongoing_movement.is_valid() {
            Cell::Empty // while a movement it ongoing, it is nobody's turn
        } else {
            self.next_move
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

    pub fn begin_immediate_movement(&mut self, mv: Movement) -> bool {
        if self.begin_movement(mv) {
            self.next_move_time = Instant::now();
            true
        } else {
            false
        }
    }
}
