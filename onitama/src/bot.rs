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

pub fn get_move(g: &Game) -> Move {
    let (white_best, black_best) = (i64::MIN, i64::MAX);
    let mut white_best_move = None;
    let mut black_best_move = None;
    for m in g.gen_moves().into_iter() {
        let eval = eval_move(g, &m, 5);
        if eval >= white_best {
            white_best_move = Some(m)
        } else if eval <= black_best {
            black_best_move = Some(m)
        }
    }

    if g.white_to_move {
        white_best_move.unwrap()
    } else {
        black_best_move.unwrap()
    }
}

pub fn perft(g: &Game, depth: u8) -> usize {
    if !g.in_progress || depth == 0 {
        1
    } else {
        let moves = g.gen_moves();
        moves
            .iter()
            .map(|m| {
                let new_g = g.take_turn(m);
                perft(&new_g, depth - 1)
            })
            .sum()
    }
}

pub fn perft_cheat(g: &Game, depth: u8) -> usize {
    if !g.in_progress {
        1
    } else if depth == 1 {
        g.count_moves()
    } else {
        let moves = g.gen_moves();
        moves
            .iter()
            .map(|m| {
                let new_g = g.take_turn(m);
                perft_cheat(&new_g, depth - 1)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;
    use test::Bencher;

    const CARDS: [Card; 5] = [
        Card::Ox,
        Card::Boar,
        Card::Horse,
        Card::Elephant,
        Card::Crab,
    ];

    #[test]
    fn test_pertf() {
        let game = Game::from_cards(Vec::from(CARDS));
        assert_eq!(perft_cheat(&game, 1), 10);
        assert_eq!(perft_cheat(&game, 2), 130);
        assert_eq!(perft_cheat(&game, 3), 1989);
        assert_eq!(perft_cheat(&game, 4), 28509);
        assert_eq!(perft_cheat(&game, 5), 487780);
        assert_eq!(perft_cheat(&game, 6), 7748422);
        assert_eq!(perft_cheat(&game, 7), 137281607);
        assert_eq!(perft_cheat(&game, 8), 2353802670);
    }

    #[bench]
    fn bench_perft_3(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(&game, 3));
    }
    #[bench]
    fn bench_perft_4(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(&game, 4));
    }
    #[bench]
    fn bench_perft_5(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(&game, 5));
    }
    #[bench]
    fn bench_perft_6(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(&game, 6));
    }
    #[bench]
    fn bench_perft_7(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(&game, 7));
    }
}
