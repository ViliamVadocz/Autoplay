use crate::cards::shift_bitmap;
use crate::game::{Game, Move};
use bitwise::ClearBit;

const PIECE_WEIGHT: i64 = 10;
const SQUARE_WEIGHT: i64 = 1;

// positive is good for white, negative is good for black
pub fn game_eval(g: Game) -> i64 {
    if !g.in_progress {
        let winner_is_white = !g.white_to_move;
        if winner_is_white {
            i64::MAX
        } else {
            i64::MIN
        }
    } else {
        // calculate controlled squares
        let mut white_control = 0u32;
        let mut pieces = g.white.pieces;
        while pieces != 0 {
            let pos = pieces.trailing_zeros();
            pieces = pieces.clear_bit(pos);
            white_control |= shift_bitmap(g.white.cards[0].get_white(), pos);
            white_control |= shift_bitmap(g.white.cards[1].get_white(), pos);
        }
        let mut black_control = 0u32;
        let mut pieces = g.black.pieces;
        while pieces != 0 {
            let pos = pieces.trailing_zeros();
            pieces = pieces.clear_bit(pos);
            black_control |= shift_bitmap(g.black.cards[0].get_black(), pos);
            black_control |= shift_bitmap(g.black.cards[1].get_black(), pos);
        }
        let square_diff = white_control.count_ones() as i64 - black_control.count_ones() as i64;
        let piece_diff = g.white.pieces.count_ones() as i64 - g.black.pieces.count_ones() as i64;
        PIECE_WEIGHT * piece_diff + SQUARE_WEIGHT * square_diff
    }
}

pub fn eval_move(g: &Game, the_move: &Move, depth: u8) -> i64 {
    let new_g = g.take_turn(the_move);
    if !new_g.in_progress || depth == 0 {
        game_eval(new_g)
    } else {
        let mut white_best = i64::MIN;
        let mut black_best = i64::MAX;
        for m in new_g.gen_moves().iter() {
            let eval = eval_move(&new_g, m, depth - 1);
            if eval > white_best {
                white_best = eval;
            } else if eval < black_best {
                black_best = eval;
            }
        }

        if new_g.white_to_move {
            white_best
        } else {
            black_best
        }
    }
}

const SEARCH_DEPTH: u8 = 5;
pub fn get_move(g: &Game) -> Move {
    let mut best_move = None;
    if g.white_to_move {
        let mut best = i64::MIN;
        for m in g.gen_moves().into_iter() {
            let eval = eval_move(g, &m, SEARCH_DEPTH);
            if eval >= best {
                best_move = Some(m);
                best = eval;
            }
        }
        best_move.unwrap()
    } else {
        let mut best = i64::MAX;
        for m in g.gen_moves().into_iter() {
            let eval = eval_move(g, &m, SEARCH_DEPTH);
            if eval <= best {
                best_move = Some(m);
                best = eval;
            }
        }
        best_move.unwrap()
    }
}
