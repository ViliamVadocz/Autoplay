use crate::cards::shift_bitmap;
use crate::game::{Game, Move};
use bitwise::ClearBit;

const PIECE_WEIGHT: i64 = 10;
const SQUARE_WEIGHT: i64 = 1;

// positive is good for white, negative is good for black
pub fn game_eval(g: Game) -> i64 {
    if !g.in_progress {
        i64::MIN
    } else {
        // calculate controlled squares
        let mut my_control = 0u32;
        let mut pieces = g.my.pieces;
        while pieces != 0 {
            let pos = pieces.trailing_zeros();
            pieces = pieces.clear_bit(pos);
            my_control |= shift_bitmap(g.my.cards[0].get_move(g.color), pos);
            my_control |= shift_bitmap(g.my.cards[1].get_move(g.color), pos);
        }
        let mut other_control = 0u32;
        let mut pieces = g.other.pieces;
        while pieces != 0 {
            let pos = pieces.trailing_zeros();
            pieces = pieces.clear_bit(pos);
            other_control |= shift_bitmap(g.other.cards[0].get_move(g.color.next()), pos);
            other_control |= shift_bitmap(g.other.cards[1].get_move(g.color.next()), pos);
        }
        let square_diff = my_control.count_ones() as i64 - other_control.count_ones() as i64;
        let piece_diff = g.my.pieces.count_ones() as i64 - g.other.pieces.count_ones() as i64;
        PIECE_WEIGHT * piece_diff + SQUARE_WEIGHT * square_diff
    }
}

pub fn eval_move(g: &Game, the_move: &Move, depth: u8) -> i64 {
    let new_g = g.take_turn(the_move);
    if !new_g.in_progress || depth == 0 {
        game_eval(new_g)
    } else {
        new_g
            .gen_moves()
            .iter()
            .map(|m| -eval_move(&new_g, m, depth - 1))
            .max()
            .unwrap()
    }
}

const SEARCH_DEPTH: u8 = 5;
pub fn get_move(g: &Game) -> Move {
    g.gen_moves()
        .into_iter()
        .map(|m| (eval_move(g, &m, SEARCH_DEPTH), m))
        .max_by_key(|x| x.0)
        .unwrap()
        .1
}
