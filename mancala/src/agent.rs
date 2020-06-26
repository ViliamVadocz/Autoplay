use crate::game::{Game, Player, Status};
use ordered_float::OrderedFloat;
use std::{f64, i8};

pub trait Agent {
    /// First player = positive
    /// Second player = negative
    fn evaluate_game(&self, game: &Game) -> Result<f64, &'static str>;

    fn tree_search(&self, game: &Game, depth: u8) -> Result<usize, &'static str> {
        let moves = game.possible_moves();
        let mut best = *moves
            .first()
            .ok_or("cannot use tree_search for a game with no moves")?;
        let mut alpha = OrderedFloat(f64::NEG_INFINITY);
        let mut beta = OrderedFloat(f64::INFINITY);

        // pick best move
        for my_move in moves.into_iter() {
            let score = self
                .alpha_beta_minimax(game, my_move, alpha, beta, depth)
                .map(OrderedFloat)?;
            match game.current_player {
                Player::First => {
                    if score > alpha {
                        alpha = score;
                        best = my_move;
                    }
                }
                Player::Second => {
                    if score < beta {
                        beta = score;
                        best = my_move;
                    }
                }
            };
        }
        Ok(best)
    }

    // PRUNING DID NOT GIVE THE SAME RESULT. BUG STILL EXISTS HERE.
    // alpha: best explored option along path to root for maximizer
    // beta: best explored option along path to root for minimizer
    fn alpha_beta_minimax(
        &self,
        game: &Game,
        my_move: usize,
        mut alpha: OrderedFloat<f64>,
        mut beta: OrderedFloat<f64>,
        depth: u8,
    ) -> Result<f64, &'static str> {
        // exit conditions
        if depth == 0 {
            return self.evaluate_game(game);
        }
        if let Status::Ended = game.status {
            return self.evaluate_game(game);
        }

        let mut imaginary_game = game.clone();
        imaginary_game.make_move(my_move)?;

        // explore
        for my_move in imaginary_game.possible_moves().into_iter() {
            let score = self
                .alpha_beta_minimax(&imaginary_game, my_move, alpha, beta, depth - 1)
                .map(OrderedFloat)?;
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
}

// example
pub struct SimpleAgent {}

impl Agent for SimpleAgent {
    fn evaluate_game(&self, game: &Game) -> Result<f64, &'static str> {
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
}
