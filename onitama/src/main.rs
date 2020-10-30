extern crate bitmaps;
extern crate typenum;
extern crate rand;

#[macro_use]
mod macros;

mod cards;
mod game;

fn main() {
    println!("{}", game::Game::new())
}
