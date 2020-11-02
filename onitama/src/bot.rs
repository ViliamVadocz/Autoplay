use crate::game::{Game, Move};

pub fn get_move(g: &Game) -> Move {
    let mut moves = g.gen_moves();
    moves.pop().unwrap()
}
