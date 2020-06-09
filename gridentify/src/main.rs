// #[macro_use]
// extern crate rulinalg;
// use rulinalg::matrix::Matrix;

mod game;
mod moves;

use game::{Game, Status};
use moves::possible_moves;

fn main() {
    let mut game = Game::new();
    println!("{:?}", game);

    loop {
        let mut moves = possible_moves(&game.board);
        println!("num moves: {}", moves.len());
        let my_move = moves.pop().unwrap();
        println!("{:?}", my_move);
        game.make_move(my_move);
        println!("{:?}", game);
        if let Status::Ended = game.status {
            println!("score: {}", game.score);
            break;
        }
    }

    
}

// TODO

// [X] make an implementation of gridentify
//   [X] board
//   [X] move generation
//   [X] making moves
//   [X] generating random tiles
//   [X] detecting game finish

// [ ] make a tree search func
//   [ ] evaluate possible board
//   [ ] generate possible moves
//   [ ] evaluate probable "goodness" and add to stack
//   [ ] try move and tree search

// [ ] hardcoded bot
//   [ ] static board evaluation

// [ ] make neural network
//   [ ] move eval (to explore moves)
//   [ ] static board eval

// [ ] train neural network
//   [ ] train on hardcoded bot
//   [ ] amplification & distillation
//     [ ] use more processing to get better moves
//     [ ] train on amplified network

// [ ] maybe
//   [ ] wildcards to reduce branching factor
