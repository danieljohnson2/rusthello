use cursive::event::*;
use cursive::theme::*;
use cursive::*;
use std::fmt;
use std::ops::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    White,
    Black,
}

impl Cell {
    fn to_str(self) -> &'static str {
        match self {
            Cell::Empty => " ",
            Cell::White => "O",
            Cell::Black => "X",
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        use Cell::*;

        let cells = vec![Empty; width * height];

        let mut board = Board {
            width,
            height,
            cells,
        };

        let center = Vec2::new(width / 2, height / 2);
        board[Vec2::new(center.x, center.y)] = Black;
        board[Vec2::new(center.x - 1, center.y - 1)] = Black;
        board[Vec2::new(center.x, center.y - 1)] = White;
        board[Vec2::new(center.x - 1, center.y)] = White;

        board
    }
}

impl Index<Vec2> for Board {
    type Output = Cell;

    fn index(&self, index: Vec2) -> &Self::Output {
        let idx = index.y * self.height + index.x;
        &self.cells[idx]
    }
}

impl IndexMut<Vec2> for Board {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        let idx = index.y * self.height + index.x;
        &mut self.cells[idx]
    }
}

struct BoardView {
    board: Board,
    cursor: Vec2,
}

impl BoardView {
    fn new(board: Board) -> BoardView {
        let cursor = Vec2::new(board.width / 2, board.height / 2);
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

        for y in 0..board.height {
            for x in 0..board.width {
                let cell = board[Vec2::new(x, y)];
                print_cell(printer, x, y, cell);
            }
        }

        let hilight: ColorStyle = ColorStyle::back(Color::Light(BaseColor::White));
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
        Vec2::new(self.board.width * 2 + 1, self.board.height * 2 + 1)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        use EventResult::*;

        return match event {
            Event::Key(Key::Up) => move_cursor(self, 0, -1),
            Event::Key(Key::Down) => move_cursor(self, 0, 1),
            Event::Key(Key::Left) => move_cursor(self, -1, 0),
            Event::Key(Key::Right) => move_cursor(self, 1, 0),
            _ => Ignored,
        };

        fn move_cursor(me: &mut BoardView, dx: isize, dy: isize) -> EventResult {
            me.move_cursor(dx, dy);
            Ignored
        }
    }
}

fn main() {
    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(BoardView::new(Board::new(8, 8)));
    siv.run();
}
