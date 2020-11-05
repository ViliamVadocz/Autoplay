use crate::game::{Game, Move};

pub fn get_move(g: &Game) -> Move {
    let mut moves = g.gen_moves();
    moves.pop().unwrap()
}

pub fn perft(g: Game, depth: u64) -> u64 {
    if depth == 0 || !g.in_progress {
        1
    } else {
        let moves = g.gen_moves();
        moves
            .iter()
            .map(|m| {
                let new_g = g.take_turn(m);
                perft(new_g, depth - 1)
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

    #[bench]
    fn bench_perft_3(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(game, 3));
    }

    #[bench]
    fn bench_perft_4(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(game, 4));
    }
    #[bench]
    fn bench_perft_5(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(game, 5));
    }
    #[bench]
    fn bench_perft_6(b: &mut Bencher) {
        let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
        b.iter(|| perft(game, 6));
    }
    // #[bench]
    // fn bench_perft_7(b: &mut Bencher) {
    //     let game = test::black_box(Game::from_cards(Vec::from(CARDS)));
    //     b.iter(|| perft(game, 7));
    // }
}
