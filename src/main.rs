use cursive::event::*;
use cursive::theme::*;
use cursive::*;

mod board;
mod cell;

use crate::board::*;
use crate::cell::*;

struct BoardView {
    board: Board,
    cursor: Vec2,
}

impl BoardView {
    fn new(board: Board) -> BoardView {
        let cursor = Vec2::new(board.get_width() / 2, board.get_height() / 2);
        BoardView { board, cursor }
    }

    fn move_cursor(&mut self, dx: isize, dy: isize) {
        self.cursor.x = (self.cursor.x as isize + dx) as usize;
        self.cursor.y = (self.cursor.y as isize + dy) as usize;
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        let board = &self.board;

        for y in 0..board.get_height() {
            for x in 0..board.get_width() {
                let cell = board[Vec2::new(x, y)];
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
            if me.board.place(me.cursor, Cell::White) {
                let valid = me.board.find_valid_moves(Cell::Black);
                if !valid.is_empty() {
                    me.board.place(valid[0], Cell::Black);
                };
            }
            Ignored
        }
    }
}

fn main() {
    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(BoardView::new(Board::new(8, 8)));
    siv.run();
}
