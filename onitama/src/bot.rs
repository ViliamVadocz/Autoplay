use std::cmp::Ordering;

use bitwise::TestBit;

use crate::cards::{shift_bitmap, BitIter};
use crate::game::{Game, Move};

const PIECE_WEIGHT: i64 = 10;
const SQUARE_WEIGHT: i64 = 1;
const CHECK_WEIGHT: i64 = 10;

// positive is good for white, negative is good for black
pub fn game_eval(g: &Game) -> i64 {
    let mut my_control = 0u32;
    let my_card = g.my.cards[0].get_move(g.color) | g.my.cards[1].get_move(g.color);
    for pos in BitIter(g.my.pieces) {
        my_control |= shift_bitmap(my_card, pos);
    }
    let mut other_control = 0u32;
    let other_card =
        g.other.cards[0].get_move(g.color.next()) | g.other.cards[1].get_move(g.color.next());
    for pos in BitIter(g.other.pieces) {
        other_control |= shift_bitmap(other_card, pos);
    }
    let mut checks = 0i64;
    if my_control.test_bit(g.other.king) {
        checks += 1
    }
    if other_control.test_bit(g.my.king) {
        checks -= 1
    }
    if shift_bitmap(my_card, g.my.king as u32).test_bit(g.goal()) {
        checks += 1
    }
    if shift_bitmap(other_card, g.other.king as u32).test_bit(24 - g.goal()) {
        checks -= 1
    }
    let square_diff = my_control.count_ones() as i64 - other_control.count_ones() as i64;
    let piece_diff = g.my.pieces.count_ones() as i64 - g.other.pieces.count_ones() as i64;
    PIECE_WEIGHT * piece_diff + SQUARE_WEIGHT * square_diff + checks * CHECK_WEIGHT
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Value {
    Win(u8),
    Loss(u8),
    Eval(i64),
}

impl Value {
    fn next(self) -> Self {
        match self {
            Value::Win(x) => Value::Loss(x + 1),
            Value::Loss(x) => Value::Win(x + 1),
            Value::Eval(y) => Value::Eval(-y),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Value::Win(x) => match other {
                Value::Win(y) => y.cmp(x),
                Value::Eval(_) => Ordering::Greater,
                Value::Loss(_) => Ordering::Greater,
            },
            Value::Eval(x) => match other {
                Value::Win(_) => Ordering::Less,
                Value::Eval(y) => x.cmp(y),
                Value::Loss(_) => Ordering::Greater,
            },
            Value::Loss(x) => match other {
                Value::Win(_) => Ordering::Less,
                Value::Eval(_) => Ordering::Less,
                Value::Loss(y) => x.cmp(y),
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const EVAL_FOR_DEEPENING: f64 = 15.;
fn get_value(g: &Game, nodes: f64, eval: i64) -> Value {
    if !g.in_progress {
        Value::Loss(0)
    } else {
        let new_eval = game_eval(&g);
        let nodes = nodes / ((new_eval + eval + 6) as f64 / EVAL_FOR_DEEPENING).exp();

        if nodes <= 1. {
            Value::Eval(eval)
        } else {
            let moves = g.gen_moves();
            let nodes = nodes / moves.len() as f64;
            moves
                .iter()
                .map(|m| get_value(&g.take_turn(m), nodes, new_eval))
                .min()
                .unwrap()
                .next()
        }
    }
}

const SEARCH_NODES: f64 = 100000000.;
pub fn get_move(g: &Game) -> Move {
    let moves = g.gen_moves();
    let nodes = SEARCH_NODES / moves.len() as f64;
    let eval = game_eval(g);
    let (eval, m) = moves
        .into_iter()
        .map(|m| (get_value(&g.take_turn(&m), nodes, eval), m))
        .min_by_key(|x| x.0)
        .unwrap();
    println!("{:?}", eval.next());
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        use Value::*;

        assert!(Win(1) > Win(2));
        assert!(Loss(2) > Loss(1));
        assert!(Win(10) > Loss(10));
        assert!(Eval(-1) > Loss(10));
        assert!(Win(10) > Eval(20));
        assert!(Eval(10) > Eval(-10));
    }
}
