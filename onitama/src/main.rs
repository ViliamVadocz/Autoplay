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
    let mut g = game::Game::new();
    println!("{}", g);
    while g.in_progress {
        let mut moves = g.gen_moves();
        g.take_turn(moves.pop().unwrap());
        println!("{}", g);
    }
}
