use crate::game::{Game, Player, Status};
use ordered_float::OrderedFloat;
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
    // make closure
    let eval_closure = |&my_move: &usize| {
        let mut imaginary_game = game.clone();
        imaginary_game.make_move(my_move)?;
        recursive_tree_search(imaginary_game, 1, depth).map(OrderedFloat)
    };

    // pick best move
    let moves = game.possible_moves().into_iter();
    match game.current_player {
        Player::First => moves.max_by_key(eval_closure),
        Player::Second => moves.min_by_key(eval_closure),
    }
    .ok_or("cannot use tree_search for a game with no moves")
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
    let scores = moves.into_iter().map(|my_move| {
        let mut imaginary_game = game.clone();
        imaginary_game.make_move(my_move)?;
        recursive_tree_search(imaginary_game, depth + 1, max_depth).map(OrderedFloat)
    });

    match game.current_player {
        Player::First => scores.max(),
        Player::Second => scores.min(),
    }
    .ok_or("could not determine the max or min score in tree search")?
    .map(|val| val.into_inner())
}
