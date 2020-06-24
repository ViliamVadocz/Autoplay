use crate::game::{Game, Player, Status};
use std::i8;

// First player = positive
// Second player = negative
fn static_eval(game: Game) -> Result<f64, &'static str> {
    let eval = match game.status {
        Status::Running => game.board[6] - game.board[13],
        Status::Ended => match game.get_winner() {
            Some((Player::First, (_, _))) => i8::MAX,
            Some((Player::Second, (_, _))) => i8::MIN,
            None => 0,
        },
    } as f64;
    // handle nan
    if eval.is_nan() {
        Err("encountered nan")
    } else {
        Ok(eval)
    }
}

pub fn tree_search(game: &Game, depth: u8) -> Result<usize, &'static str> {
    let moves = game.possible_moves();
    let evals = moves
        .iter()
        .map(|&my_move: &usize| {
            let mut imaginary_game = game.clone();
            imaginary_game.make_move(my_move)?;
            recursive_tree_search(imaginary_game, 1, depth)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .enumerate();
    // pick best move
    let best_move = match game.current_player {
        Player::First => moves.get(
            evals
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0,
        ),
        Player::Second => moves.get(
            evals
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0,
        ),
    };
    match best_move {
        Some(m) => Ok(*m),
        None => Err("cannot use tree_search for a game with no moves"),
    }
}

fn recursive_tree_search(game: Game, depth: u8, max_depth: u8) -> Result<f64, &'static str> {
    // end recursion
    if depth >= max_depth {
        return static_eval(game);
    }

    let moves = game.possible_moves();
    if moves.is_empty() {
        return static_eval(game);
    }

    // explore
    let scores = moves
        .into_iter()
        .map(|my_move| {
            let mut imaginary_game = game.clone();
            imaginary_game.make_move(my_move)?;
            recursive_tree_search(imaginary_game, depth + 1, max_depth)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter();

    match game.current_player {
        Player::First => scores.max_by(|a, b| a.partial_cmp(b).unwrap()),
        Player::Second => scores.min_by(|a, b| a.partial_cmp(b).unwrap()),
    }
    .ok_or("could not determine the max or min score in tree search")
}
