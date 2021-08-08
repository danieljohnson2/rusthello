use crate::game::*;
use cursive::*;
use std::cmp::*;

/// A view to display the score, and when the game is over
/// it declares the winner.
pub struct ScoreboardView {
    game: GameRef,
}

impl ScoreboardView {
    pub fn new(game: GameRef) -> ScoreboardView {
        ScoreboardView { game }
    }
}

impl View for ScoreboardView {
    fn draw(&self, printer: &Printer) {
        let game = self.game.borrow();
        let board = game.to_board();
        let game_over = board.is_game_over();
        let black_score = board.count_cells(Cell::Black);
        let white_score = board.count_cells(Cell::White);

        let line1 = format!("●: {}", black_score);
        printer.print(Vec2::new(0, 0), &line1);
        let line2 = format!("○: {}", white_score);
        printer.print(Vec2::new(0, 1), &line2);

        if game_over {
            printer.print(Vec2::new(0, 2), "GAME OVER ");

            let winner = match black_score.cmp(&white_score) {
                Ordering::Greater => "● WINS",
                Ordering::Less => "○ WINS",
                Ordering::Equal => "DRAW",
            };

            printer.print(Vec2::new(10, 2), winner)
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(20, 3)
    }
}
