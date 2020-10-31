extern crate bitmaps;
extern crate typenum;
extern crate rand;

use bitmaps::Bitmap;
use typenum::U25;

#[macro_use]
mod macros;

mod cards;
mod game;

fn main() {
    // let mut g = game::Game::new();
    // println!("{}", g);
    // g.take_turn(game::Move{from: 23, to: 18, used_left_card: true});
    // println!("{}", g);

    let a = board!(
        1 0 0 0 0
        0 1 0 0 0
        0 0 1 1 1
        0 0 1 0 0
        0 0 1 0 0
    );
    cards::print_bitmap(&cards::shift_bitmap(&cards::reverse(&a), 11));
}
