use crate::game::*;
use cursive::event::*;
use cursive::theme::*;
use cursive::*;

/// A view to display the board's cells; it also
/// tracks the cursor used by the player to make moves.
pub struct BoardView {
    game: GameRef,
    cursor: Loc,
}

impl BoardView {
    pub fn new(game: GameRef) -> BoardView {
        let cursor = game.borrow().to_board().get_board_center();
        BoardView { game, cursor }
    }

    fn get_bg_char(&self, xy: Vec2) -> &'static str {
        let game = self.game.borrow();
        let board = game.to_board();
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

    fn move_cursor(&mut self, dx: isize, dy: isize) -> bool {
        let game = self.game.borrow();
        let board = game.to_board();
        if let Some(l) = board.offset_within(self.cursor, dx, dy) {
            self.cursor = l;
            true
        } else {
            false
        }
    }

    fn render(&self, printer: &Printer) {
        let game = self.game.borrow();
        let board = game.to_board();
        let height = board.get_height();
        let width = board.get_width();
        let cursor = self.cursor;

        for y in 0..=height * 2 {
            for x in 0..=width * 2 {
                let xy = Vec2::new(x, y);
                printer.print(xy, self.get_bg_char(xy));
            }
        }

        if !board.is_game_over() {
            printer.print_box((cursor.x * 2, cursor.y * 2), (3, 3), false);
        }

        for loc in board.locations() {
            let cell = board[loc];
            let xy = XY::new(loc.x * 2 + 1, loc.y * 2 + 1);

            if loc == cursor && !board.is_game_over() {
                let candidate_move = Movement::new(&board, self.cursor, Cell::Black);
                let hilight = if candidate_move.is_valid() {
                    ColorStyle::back(Color::Light(BaseColor::White))
                } else {
                    ColorStyle::back(Color::Light(BaseColor::Red))
                };

                printer.with_color(hilight, |p| {
                    p.print(xy, cell.to_str());
                });
            } else {
                printer.print(xy, cell.to_str());
            }
        }
    }
}

impl View for BoardView {
    fn draw(&self, printer: &Printer) {
        if let Ok(mut game) = self.game.try_borrow_mut() {
            if game.check_move() == Cell::White {
                let mv = game.get_ai_movement();
                game.begin_movement(mv);
            }
        }
        self.render(printer);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let game = self.game.borrow();
        let board = game.to_board();
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
            Event::Char('q') => EventResult::with_cb(|s| s.quit()),
            _ => Ignored,
        };

        fn move_cursor(me: &mut BoardView, dx: isize, dy: isize) -> EventResult {
            me.move_cursor(dx, dy);
            Ignored
        }

        fn make_move(me: &mut BoardView) -> EventResult {
            let mut game = me.game.borrow_mut();
            if game.check_move() == Cell::Black {
                let mv = game.get_player_movement(me.cursor);
                game.begin_immediate_movement(mv);
            }
            Ignored
        }
    }
}
