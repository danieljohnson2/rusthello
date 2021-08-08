use cursive::theme::*;
use cursive::view::*;
use cursive::views::*;
use cursive::*;
use cursive_aligned_view::Alignable;

mod game;
mod iterext;
mod ui;

use crate::game::*;
use crate::ui::*;

fn main() {
    let mut siv = Cursive::default();
    siv.set_fps(60);

    let board = Board::new(8, 8);
    let game = Game::new(board).into_ref();
    let boardview = BoardView::new(game.clone());

    let scoreboard = ShadowView::new(Layer::with_color(
        Panel::new(ScoreboardView::new(game).fixed_size((18, 3))),
        ColorStyle::back(Color::Dark(BaseColor::White)),
    ))
    .align_center();

    siv.add_fullscreen_layer(Layer::with_color(
        LinearLayout::vertical().child(
            LinearLayout::horizontal()
                .child(boardview)
                .child(scoreboard),
        ),
        ColorStyle::back(Color::Dark(BaseColor::Blue)),
    ));
    siv.run();
}
