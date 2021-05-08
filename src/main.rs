use cursive::*;

struct BoardView {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

impl BoardView {
    fn new(width: usize, height: usize) -> BoardView {
        let cells = vec![' '; width * height];
        BoardView {
            width,
            height,
            cells,
        }
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        let mut index = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let loc = XY::new(x * 2, y * 2);
                printer.print(loc, "+-+");
                printer.print(loc + XY::new(0, 1), "| |");
                printer.print(loc + XY::new(0, 2), "+-+");

                let cell = self.cells[index];
                printer.print(loc + XY::new(1, 1), &cell.to_string());
                index += 1;
            }
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        Vec2::new(self.width * 2 + 1, self.height * 2 + 1)
    }
}

fn main() {
    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(BoardView::new(8, 8));
    siv.run();
}
