use std::cmp::Ordering;

use crate::cards::{shift_bitmap, BitIter};
use crate::game::{Game, Move};

const PIECE_WEIGHT: i64 = 10;
const SQUARE_WEIGHT: i64 = 1;

// positive is good for white, negative is good for black
pub fn game_eval(g: &Game) -> i64 {
    let mut my_control = 0u32;
    for pos in BitIter(g.my.pieces) {
        my_control |= shift_bitmap(g.my.cards[0].get_move(g.color), pos);
        my_control |= shift_bitmap(g.my.cards[1].get_move(g.color), pos);
    }
    let mut other_control = 0u32;
    for pos in BitIter(g.other.pieces) {
        other_control |= shift_bitmap(g.other.cards[0].get_move(g.color.next()), pos);
        other_control |= shift_bitmap(g.other.cards[1].get_move(g.color.next()), pos);
    }
    let square_diff = my_control.count_ones() as i64 - other_control.count_ones() as i64;
    let piece_diff = g.my.pieces.count_ones() as i64 - g.other.pieces.count_ones() as i64;
    PIECE_WEIGHT * piece_diff + SQUARE_WEIGHT * square_diff
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

fn get_value(g: &Game, depth: u8) -> Value {
    if !g.in_progress {
        Value::Loss(0)
    } else if depth == 0 {
        Value::Eval(game_eval(g))
    } else {
        g.gen_moves()
            .iter()
            .map(|m| get_value(&g.take_turn(m), depth - 1))
            .min()
            .unwrap()
            .next()
    }
}

const SEARCH_DEPTH: u8 = 6;
pub fn get_move(g: &Game) -> Move {
    let (eval, m) = g
        .gen_moves()
        .into_iter()
        .map(|m| (get_value(&g.take_turn(&m), SEARCH_DEPTH), m))
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
