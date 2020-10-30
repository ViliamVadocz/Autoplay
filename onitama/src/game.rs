use bitmaps::Bitmap;
use typenum::U25;

use std::fmt;

use crate::cards::{Card, draw_cards};

pub struct Game {
    white: Bitmap<U25>,
    black: Bitmap<U25>,
    white_king: usize,
    black_king: usize,
    white_cards: [Card; 2],
    black_cards: [Card; 2],
    table_card: Card,
    white_to_move: bool,
}

impl Game {
    pub fn new() -> Game {
        // TODO: Generate cards
        let mut cards = draw_cards();
        let last_card = cards.pop().unwrap();
        let white_to_move = last_card.is_white();
        Game {
            white: board!(
                0 0 0 0 0 
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                1 1 1 1 1
            ),
            black: board!(
                1 1 1 1 1
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
                0 0 0 0 0
            ),
            white_king: 22,
            black_king: 2,
            white_cards: [cards.pop().unwrap(), cards.pop().unwrap()],
            black_cards: [cards.pop().unwrap(), cards.pop().unwrap()],
            table_card: last_card,
            white_to_move,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        // colour to move
        if self.white_to_move {
            output.push_str("White to move\n")
        } else {
            output.push_str("Black to move\n")
        }

        // board
        let mut board = String::new();
        for i in 0..25 {
            if self.white.get(i) {
                if i == self.white_king {
                    board.push('♔');
                } else {
                    board.push('♙');
                }
            } else if self.black.get(i) {
                if i == self.black_king {
                    board.push('♚');
                } else {
                    board.push('♟');
                }
            } else {
                board.push('◻');
            }
            board.push(' ');
            if i % 5 == 4 {
                board.push('\n')
            }
        }
        output.push_str(&board);

        // cards
        output.push_str(&format!("Black cards: {:?}\n", self.black_cards));
        output.push_str(&format!("White cards: {:?}\n", self.white_cards));
        output.push_str(&format!("Table card: {:?}\n", self.table_card));

        write!(f, "{}", output)
    }
}