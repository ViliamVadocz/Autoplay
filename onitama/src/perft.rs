use crate::game::Game;

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
        // assert_eq!(perft_cheat(&game, 8), 2353802670);
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
