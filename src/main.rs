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

    fn to_opposite(self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::White => Cell::Black,
            Cell::Black => Cell::White,
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

    fn place(&mut self, loc: Vec2, cell: Cell) -> bool {
        if self[loc] == Cell::Empty {
            let flips = self.find_flippable_around(loc, cell);

            if !flips.is_empty() {
                self[loc] = cell;

                for f in flips {
                    self[f] = cell
                }
                return true;
            }
        }

        false
    }

    fn find_valid_moves(&self, cell: Cell) -> Vec<Vec2> {
        let mut valid = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let here = Vec2::new(x, y);

                if self.is_valid_move(here, cell) {
                    valid.push(here)
                }
            }
        }

        valid
    }

    fn is_valid_move(&self, loc: Vec2, cell: Cell) -> bool {
        !self.find_flippable_around(loc, cell).is_empty()
    }

    fn find_flippable_around(&self, start: Vec2, cell: Cell) -> Vec<Vec2> {
        let mut buffer: Vec<Vec2> = Vec::new();

        if self[start] == Cell::Empty {
            buffer.append(&mut self.find_flippable(start, cell, XY::new(-1, -1)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(-1, 0)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(-1, 1)));

            buffer.append(&mut self.find_flippable(start, cell, XY::new(0, -1)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(0, 1)));

            buffer.append(&mut self.find_flippable(start, cell, XY::new(1, -1)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(1, 0)));
            buffer.append(&mut self.find_flippable(start, cell, XY::new(1, 1)));
        }

        buffer
    }

    fn find_flippable(&self, start: Vec2, cell: Cell, delta: XY<isize>) -> Vec<Vec2> {
        let mut buffer: Vec<Vec2> = Vec::new();
        let mut here = start;

        loop {
            if let Some(next) = self.offset_within(here, delta) {
                here = next;
                if self[here] == cell {
                    return buffer;
                } else if self[here] == cell.to_opposite() {
                    buffer.push(here);
                    continue;
                }
            }
            return Vec::new();
        }
    }

    fn offset_within(&self, vec: Vec2, delta: XY<isize>) -> Option<Vec2> {
        if let Some(next) = vec.checked_add(delta) {
            if next.x < self.width && next.y < self.height {
                return Some(next);
            }
        }
        None
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
        Vec2::new(self.board.width * 2 + 1, self.board.height * 2 + 1)
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
