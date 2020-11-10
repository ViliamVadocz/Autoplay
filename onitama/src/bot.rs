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

pub fn get_move(g: &Game) -> Move {
    *g.gen_moves()
        .iter()
        .map(|m| (eval_move(g, m, 5), m))
        .max_by_key(|x| x.0)
        .unwrap()
        .1
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
