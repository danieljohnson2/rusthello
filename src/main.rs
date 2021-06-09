use cursive::event::*;
use cursive::theme::*;
use cursive::views::*;
use cursive::*;

mod board;
mod cell;

use crate::board::*;
use crate::cell::*;

struct BoardView {
    board: BoardRef,
    cursor: Loc,
}

impl BoardView {
    fn new(board: BoardRef) -> BoardView {
        let cursor = get_default_cursor(&board);
        return BoardView { board, cursor };

        fn get_default_cursor(board: &BoardRef) -> Loc {
            let b = board.borrow();
            Loc::new(b.get_width() / 2, b.get_height() / 2)
        }
    }

    fn get_bg_char(&self, xy: Vec2) -> &'static str {
        let board = self.board.borrow();
        const LEFT: usize = 0b0001;
        const UP: usize = 0b0010;
        const RIGHT: usize = 0b0100;
        const DOWN: usize = 0b1000;

        let mut idx = 0;
        idx |= (LEFT | RIGHT) * (!xy.y & 1);
        idx |= (UP | DOWN) * (!xy.x & 1);
        idx = clear_if(idx, LEFT, xy.x == 0);
        idx = clear_if(idx, UP, xy.y == 0);
        idx = clear_if(idx, RIGHT, xy.x == board.get_width() * 2);
        idx = clear_if(idx, DOWN, xy.y == board.get_height() * 2);

        const BOX_CHARS: [&str; 16] = [
            " ", "╴", "╷", "┘", "╶", "─", "└", "┴", "╵", "┐", "│", "┤", "┌", "┬", "├", "┼",
        ];

        return BOX_CHARS[idx];

        fn clear_if(bits: usize, mask: usize, flag: bool) -> usize {
            if flag {
                bits & !mask
            } else {
                bits
            }
        }
    }

    fn place(&mut self, cell: Cell) -> bool {
        let mut board = self.board.borrow_mut();
        if board.place(self.cursor, cell) {
            let valid = board.find_valid_moves(Cell::Black);
            if !valid.is_empty() {
                board.place(valid[0], Cell::Black);
            };

            true
        } else {
            false
        }
    }

    fn move_cursor(&mut self, dx: isize, dy: isize) -> bool {
        let board = self.board.borrow();
        if let Some(l) = board.offset_within(self.cursor, dx, dy) {
            self.cursor = l;
            true
        } else {
            false
        }
    }

    fn print_cell(&self, printer: &Printer, loc: Loc, cell: Cell) {
        let mut xy = XY::new(loc.x * 2, loc.y * 2);

        printer.print(xy, self.get_bg_char(xy));
        xy.x += 1;
        printer.print(xy, self.get_bg_char(xy));
        xy.x += 1;
        printer.print(xy, self.get_bg_char(xy));

        xy.x -= 2;
        xy.y += 1;

        printer.print(xy, self.get_bg_char(xy));
        xy.x += 1;
        printer.print(xy, cell.to_str());
        xy.x += 1;
        printer.print(xy, self.get_bg_char(xy));

        xy.x -= 2;
        xy.y += 1;

        printer.print(xy, self.get_bg_char(xy));
        xy.x += 1;
        printer.print(xy, self.get_bg_char(xy));
        xy.x += 1;
        printer.print(xy, self.get_bg_char(xy));
    }
}

impl View for BoardView {
    fn draw(&'_ self, printer: &Printer) {
        let board = self.board.borrow();
        let height = board.get_height();
        let width = board.get_width();

        for y in 0..height {
            for x in 0..width {
                let loc = Loc::new(x, y);
                let cell = board[loc];
                self.print_cell(printer, loc, cell);
            }
        }

        let hilight: ColorStyle = if board.is_valid_move(self.cursor, Cell::White) {
            ColorStyle::back(Color::Light(BaseColor::White))
        } else {
            ColorStyle::back(Color::Light(BaseColor::Red))
        };

        printer.with_color(hilight, |p| {
            self.print_cell(p, self.cursor, board[self.cursor]);
        });
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let board = self.board.borrow();
        Vec2::new(board.get_width() * 2 + 1, board.get_height() * 2 + 1)
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
            me.place(Cell::White);
            Ignored
        }
    }
}

struct ScoreboardView {
    board: BoardRef,
}

impl ScoreboardView {
    pub fn new(board: BoardRef) -> ScoreboardView {
        ScoreboardView { board }
    }
}

impl View for ScoreboardView {
    fn draw(&self, printer: &Printer) {
        let board = self.board.borrow();
        let black_score = board.count_cells(Cell::Black);
        let white_score = board.count_cells(Cell::White);

        let line1 = format!("X: {}", black_score);
        printer.print(Vec2::new(0, 0), &line1);
        let line2 = format!("O: {}", white_score);
        printer.print(Vec2::new(0, 1), &line2);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(20, 2)
    }
}

fn main() {
    let mut siv = Cursive::default();
    let board = Board::new(8, 8).into_ref();
    let boardview = BoardView::new(board.clone());
    let scoreboard = ResizedView::with_fixed_size(
        (10, 6),
        ShadowView::new(Panel::new(ScoreboardView::new(board))),
    );

    siv.add_fullscreen_layer(
        LinearLayout::horizontal()
            .child(boardview)
            .child(scoreboard),
    );
    siv.run();
}
