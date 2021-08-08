use std::cell::RefCell;
use std::rc::Rc;
use std::time::*;

mod board;
mod cell;
mod movement;

pub use board::*;
pub use cell::*;
pub use movement::*;

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

    /// Provides an immutable reference to the board.
    pub fn to_board(&self) -> &Board {
        &self.board
    }

    /// Checks the game staet and returns the cell of the player
    /// whose move it now is. This will also apply ongoing moves, so it
    /// can change the game state. If there are any ongoing moves pending
    /// when it returns, it returns Empty- nobody can move until outstanding
    /// moves clear.
    ///
    /// Outsanding moves run on a timer; this may do nothing (and return Empty)
    /// if there are outstanding moves that are not yet due.
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

                    if !self.board.find_valid_moves(f).is_empty() {
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

    /// Constructs a movement for a move at the location indicated. This
    /// can result in an invalid move, if 'loc' is not a valid location
    /// or it is not anyone's turn.
    pub fn get_player_movement(&self, loc: Loc) -> Movement {
        if self.next_move != Cell::Empty {
            Movement::new(&self.board, loc, self.next_move)
        } else {
            Movement::default()
        }
    }

    /// Constructs a movement for the current player. It may be an invalid
    /// move if is not anyone's turn, or the current player has no valid moves.
    pub fn get_ai_movement(&self) -> Movement {
        if self.next_move != Cell::Empty {
            let valid = self.board.find_valid_moves(self.next_move);
            valid.into_iter().next().unwrap_or_default()
        } else {
            Movement::default()
        }
    }

    /// This plays a move. The move will play out over time, and
    /// it will switch to a new player's turn only when complete.Duration
    ///
    /// This returns false if the movement is invalid, or if another
    /// movement is ongoing. In this case no new movement is begun,
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

    /// This plays a move, like begin_movement. This will also speed the move
    /// up slightly so the first flip will occur at once.
    pub fn begin_immediate_movement(&mut self, mv: Movement) -> bool {
        if self.begin_movement(mv) {
            self.next_move_time = Instant::now();
            self.check_move();
            true
        } else {
            false
        }
    }
}
