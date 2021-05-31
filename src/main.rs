use cursive::direction::*;
use cursive::event::*;
use cursive::theme::*;
use cursive::views::*;
use cursive::*;

mod board;
mod cell;

use crate::board::*;
use crate::cell::*;

struct BoardView {
    board: Board,
    cursor: Loc,
}

impl BoardView {
    fn new(board: Board) -> BoardView {
        let cursor = Loc::new(board.get_width() / 2, board.get_height() / 2);
        BoardView { board, cursor }
    }

    fn move_cursor(&mut self, dx: isize, dy: isize) -> bool {
        if let Some(l) = self.board.offset_within(self.cursor, dx, dy) {
            self.cursor = l;
            true
        } else {
            false
        }
    }
}

impl View for BoardView {
    fn draw(&'_ self, printer: &Printer) {
        let board = &self.board;

        for y in 0..board.get_height() {
            for x in 0..board.get_width() {
                let cell = board[Loc::new(x, y)];
                print_cell(printer, x, y, cell);
            }
        }

        let hilight: ColorStyle = if self.board.is_valid_move(self.cursor, Cell::White) {
            ColorStyle::back(Color::Light(BaseColor::White))
        } else {
            ColorStyle::back(Color::Light(BaseColor::Red))
        };

        printer.with_color(hilight, |p| {
            print_cell(p, self.cursor.x, self.cursor.y, board[self.cursor]);
        });

        fn print_cell(printer: &Printer, x: usize, y: usize, cell: Cell) {
            let loc = XY::new(x * 2 + 1, y * 2 + 1);
            printer.print(Vec2::new(loc.x - 1, loc.y - 1), "+-+");
            printer.print(Vec2::new(loc.x - 1, loc.y), "| |");
            printer.print(Vec2::new(loc.x - 1, loc.y + 1), "+-+");
            printer.print(loc, cell.to_str());
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(
            self.board.get_width() * 2 + 1,
            self.board.get_height() * 2 + 1,
        )
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        use EventResult::*;

        return match event {
            Event::Key(Key::Up) => move_cursor(self, 0, -1),
            Event::Key(Key::Down) => move_cursor(self, 0, 1),
            Event::Key(Key::Left) => move_cursor(self, -1, 0),
            Event::Key(Key::Right) => move_cursor(self, 1, 0),
            Event::Char(' ') => make_move(self),
            _ => Ignored,
        };

        fn move_cursor(me: &mut BoardView, dx: isize, dy: isize) -> EventResult {
            me.move_cursor(dx, dy);
            Ignored
        }

        fn make_move(me: &mut BoardView) -> EventResult {
            let b = &mut me.board;

            if b.place(me.cursor, Cell::White) {
                let valid = b.find_valid_moves(Cell::Black);
                if !valid.is_empty() {
                    b.place(valid[0], Cell::Black);
                };
            }

            let black_score = b.count_cells(Cell::Black);
            let white_score = b.count_cells(Cell::White);

            EventResult::with_cb(move |siv| {
                siv.call_on_name("scoreboard", |v: &mut ScoreboardView| {
                    v.set_score(black_score, white_score)
                });
            })
        }
    }
}

struct ScoreboardView {
    black_score: usize,
    white_score: usize,
}

impl ScoreboardView {
    pub fn new(board: &Board) -> ScoreboardView {
        let black_score = board.count_cells(Cell::Black);
        let white_score = board.count_cells(Cell::White);
        ScoreboardView {
            black_score,
            white_score,
        }
    }

    pub fn set_score(&mut self, black_score: usize, white_score: usize) {
        self.black_score = black_score;
        self.white_score = white_score;
    }
}

impl View for ScoreboardView {
    fn draw(&self, printer: &Printer) {
        let line1 = format!("X: {}", self.black_score);
        printer.print(Vec2::new(0, 0), &line1);
        let line2 = format!("O: {}", self.white_score);
        printer.print(Vec2::new(0, 1), &line2);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(20, 2)
    }
}

fn main() {
    let mut siv = Cursive::default();
    let board = Board::new(8, 8);
    let scoreboard = ScoreboardView::new(&board);

    siv.add_fullscreen_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(BoardView::new(board))
            .child(NamedView::new("scoreboard", scoreboard)),
    );
    siv.run();
}
