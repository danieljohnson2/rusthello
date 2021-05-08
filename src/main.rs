use cursive::*;
use std::ops::*;
use std::string::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    White,
    Black
}

impl Cell {
    fn to_str(self) -> &'static str {
        match self {
            Cell::Empty => " ",
            Cell::White => "O",
            Cell::Black => "X"
        }
    }    
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
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
}

impl BoardView {
    fn new(board: Board) -> BoardView {
        BoardView { board }
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        let board = &self.board;

        for y in 0..board.height {
            for x in 0..board.width {
                let loc = XY::new(x * 2, y * 2);
                printer.print(loc, "+-+");
                printer.print(loc + XY::new(0, 1), "| |");
                printer.print(loc + XY::new(0, 2), "+-+");

                let cell = board[Vec2::new(x, y)];
                printer.print(loc + XY::new(1, 1), &cell.to_string());
            }
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(self.board.width * 2 + 1, self.board.height * 2 + 1)
    }
}

fn main() {
    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(BoardView::new(Board::new(8, 8)));
    siv.run();
}
