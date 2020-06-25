use crate::game::{Game, Player, Status};
use ordered_float::OrderedFloat;
use std::{f64, i8};

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
        alpha_beta_minimax(
            imaginary_game,
            OrderedFloat(f64::NEG_INFINITY),
            OrderedFloat(f64::INFINITY),
            depth,
        )
        .map(OrderedFloat)
    };

    // pick best move
    let moves = game.possible_moves().into_iter();
    match game.current_player {
        Player::First => moves.max_by_key(eval_closure),
        Player::Second => moves.min_by_key(eval_closure),
    }
    .ok_or("cannot use tree_search for a game with no moves")
}

// alpha: best explored option along path to root for maximizer
// beta: best explored option along path to root for minimizer
fn alpha_beta_minimax(
    game: Game,
    mut alpha: OrderedFloat<f64>,
    mut beta: OrderedFloat<f64>,
    depth: u8,
) -> Result<f64, &'static str> {
    // depth reached
    if depth == 0 {
        return static_eval(game);
    }

    // game ended
    let moves = game.possible_moves();
    if moves.is_empty() {
        return static_eval(game);
    }

    // explore
    for my_move in moves.into_iter() {
        let mut imaginary_game = game.clone();
        imaginary_game.make_move(my_move)?;
        let score = alpha_beta_minimax(imaginary_game, alpha, beta, depth - 1).map(OrderedFloat)?;
        match game.current_player {
            Player::First => {
                if score > alpha {
                    alpha = score;
                }
                // prune
                if alpha >= beta {
                    break;
                }
            }
            Player::Second => {
                if score < beta {
                    beta = score;
                }
                // prune
                if alpha >= beta {
                    break;
                }
            }
        };
    }

    Ok(match game.current_player {
        Player::First => alpha,
        Player::Second => beta,
    }
    .into_inner())
}
