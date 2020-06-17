use crate::game::{Game, Player, Status};
use std::i8;

// First player = positive
// Second player = negative
fn static_eval(game: Game) -> i8 {
    match game.status {
        Status::Running => game.board[6] - game.board[13],
        Status::Ended => match game.get_winner() {
            Some((Player::First, (_, _))) => i8::MAX,
            Some((Player::Second, (_, _))) => i8::MIN,
            None => 0,
        },
    }
}

pub fn tree_search(game: &Game, depth: u8) -> Result<usize, &'static str> {
    // create closure
    let move_eval_closure = |&my_move: &usize| {
        let mut imaginary_game = game.clone();
        imaginary_game.make_move(my_move)?;
        recursive_tree_search(imaginary_game, 1, depth)
    };

    // pick best move
    let moves = game.possible_moves().into_iter();
    let best_move = match game.current_player {
        Player::First => moves.max_by_key(move_eval_closure),
        Player::Second => moves.min_by_key(move_eval_closure),
    };
    match best_move {
        Some(m) => Ok(m),
        None => Err("cannot use tree_search for a game with no moves"),
    }
}

fn recursive_tree_search(game: Game, depth: u8, max_depth: u8) -> Result<i8, &'static str> {
    // end recursion
    if depth >= max_depth {
        return Ok(static_eval(game));
    }

    let moves = game.possible_moves();
    if moves.is_empty() {
        return Ok(static_eval(game));
    }

    // explore
    let scores = moves.into_iter().map(|my_move| {
        let mut imaginary_game = game.clone();
        imaginary_game.make_move(my_move)?;
        recursive_tree_search(imaginary_game, depth + 1, max_depth)
    });
    match game.current_player {
        Player::First => scores.max(),
        Player::Second => scores.min(),
    }
    .unwrap()
}
