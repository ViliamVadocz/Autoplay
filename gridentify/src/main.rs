// #[macro_use]
// extern crate rulinalg;
// use rulinalg::matrix::Matrix;

mod game;

use game::Game;

fn main() {
    let game = game::Game::new();
    let neighbours_of = game.get_neighbours_of();
    let moves = game.possible_moves();
    println!("{:?}", game);
    println!("{:?}", neighbours_of);
    println!("{}", moves.len());
}

// TODO

// make an implementation of gridentify
  // board
  // move generation
  // making moves
  // generating random tiles

// make a tree search func
  // evaluate possible board
  // generate possible moves
  // evaluate probable "goodness" and add to stack
  // try move and tree search

// hardcoded bot
  // static board evaluation

// make neural network
  // move eval (to explore moves)
  // static board eval

// train neural network
  // train on hardcoded bot
  // amplification
    // use more processing to get better moves
  // distillation
    // train on amplified network

// maybe
  // wildcards to reduce branching factor
