extern crate bitmaps;
extern crate typenum;
extern crate rand;

#[macro_use]
mod macros;

mod cards;
mod game;

fn main() {
    let mut g = game::Game::new();
    println!("{}", g);
    g.take_turn(game::Move{from: 23, to: 18, used_left_card: true});
    println!("{}", g);
}
